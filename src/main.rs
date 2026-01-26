use anyhow::{Context, Result};
use dotenvy::dotenv;
use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Parser)]
#[command(name = "autoseeker", about = "Fetch and classify job postings via Bright Data")] 
struct Args {
    /// Bright Data API token (or set BRIGHTDATA_TOKEN env var)
    #[arg(long, env = "BRIGHTDATA_TOKEN")]
    token: Option<String>,

    /// Dataset ID to query
    #[arg(long, default_value = "gd_lpfll7v5hcqtkxl6l")]
    dataset_id: String,

    /// Discovery location (city or region)
    #[arg(long, default_value = "Montpellier")]
    location: String,

    /// Keyword to search (e.g., Rust)
    #[arg(long, default_value = "Rust")]
    keyword: String,

    /// Max results per input
    #[arg(long, default_value_t = 10)]
    limit_per_input: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct JobPosting {
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    job_posting_id: Option<String>,
    #[serde(default)]
    job_title: Option<String>,
    #[serde(default)]
    company_name: Option<String>,
    #[serde(default)]
    job_location: Option<String>,
    #[serde(default)]
    job_summary: Option<String>,
    #[serde(default)]
    job_description_formatted: Option<String>,
    #[serde(default)]
    job_seniority_level: Option<String>,
    #[serde(default)]
    job_employment_type: Option<String>,
    #[serde(default)]
    job_function: Option<String>,
    #[serde(default)]
    job_industries: Option<String>,
    #[serde(default)]
    job_posted_date: Option<String>,
    #[serde(default)]
    application_availability: Option<bool>,
    #[serde(default)]
    discovery_input: Option<Value>,
    #[serde(flatten)]
    extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Serialize)]
struct Classification {
    seniority: Option<String>,
    contains_rust: bool,
    remote_like: bool,
    city: Option<String>,
    company: Option<String>,
}

fn classify(job: &JobPosting) -> Classification {
    let text = [
        job.job_title.as_deref().unwrap_or(""),
        job.job_summary.as_deref().unwrap_or(""),
        job.job_description_formatted.as_deref().unwrap_or(""),
    ]
    .join(" \n ");

    let lower = text.to_lowercase();

    let contains_rust = lower.contains("rust");
    let remote_like = lower.contains("remote")
        || lower.contains("télétravail")
        || job
            .job_location
            .as_deref()
            .map(|l| l.to_lowercase().contains("remote") || l.to_lowercase().contains("télétravail"))
            .unwrap_or(false);

    let seniority = job
        .job_seniority_level
        .as_ref()
        .map(|s| s.to_string())
        .or_else(|| {
            if lower.contains("senior") {
                Some("Senior".into())
            } else if lower.contains("junior") {
                Some("Junior".into())
            } else if lower.contains("mid") || lower.contains("intermediate") {
                Some("Mid".into())
            } else {
                None
            }
        });

    let city = job
        .job_location
        .as_ref()
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .or_else(|| {
            job.discovery_input
                .as_ref()
                .and_then(|v| v.get("location").and_then(|x| x.as_str().map(|s| s.to_string())))
        });

    Classification {
        seniority,
        contains_rust,
        remote_like,
        city,
        company: job.company_name.clone(),
    }
}

async fn fetch_jobs(args: &Args, token: &str) -> Result<Vec<JobPosting>> {
    let endpoint = format!(
        "https://api.brightdata.com/datasets/v3/scrape?dataset_id={}&notify=false&include_errors=true&type=discover_new&discover_by=keyword&limit_per_input={}",
        args.dataset_id, args.limit_per_input
    );

    // Build the JSON payload
    let payload = json!({
        "input": [
            {
                "location": args.location,
                "keyword": args.keyword,
                "country": "",
                "time_range": "",
                "job_type": "",
                "experience_level": "",
                "remote": "",
                "company": "",
                "location_radius": ""
            }
        ]
    });

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let bearer = format!("Bearer {}", token);
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&bearer).context("invalid token for Authorization header")?,
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let resp = client
        .post(&endpoint)
        .json(&payload)
        .send()
        .await
        .context("request failed")?;

    let status = resp.status();
    let body = resp.text().await.context("failed reading response body")?;

    if !status.is_success() {
        anyhow::bail!("HTTP {}: {}", status, body);
    }

    // Try to parse as a JSON array first
    if let Ok(v) = serde_json::from_str::<Value>(&body) {
        if let Some(arr) = v.as_array() {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                let job: JobPosting = serde_json::from_value(item.clone())?;
                out.push(job);
            }
            return Ok(out);
        }
    }

    // Fallback: NDJSON (one JSON object per line)
    let mut out = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        match serde_json::from_str::<JobPosting>(line) {
            Ok(job) => out.push(job),
            Err(e) => eprintln!("Skipping line: {} (err: {})", line.get(..100).unwrap_or(line), e),
        }
    }

    Ok(out)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment from .env if present
    dotenv().ok();

    let args = Args::parse();

    let token = args.token.clone().unwrap_or_default();
    if token.trim().is_empty() {
        anyhow::bail!("Missing token. Set BRIGHTDATA_TOKEN in env or pass --token.");
    }

    let jobs = fetch_jobs(&args, &token).await?;

    if jobs.is_empty() {
        println!("[]");
        return Ok(());
    }

    // Emit one JSON object per job with classification attached
    let mut results = Vec::with_capacity(jobs.len());
    for job in jobs.iter() {
        let cls = classify(job);
        let mut v = serde_json::to_value(job)?;
        if let Some(obj) = v.as_object_mut() {
            obj.insert("classification".into(), serde_json::to_value(cls)?);
        }
        results.push(v);
    }

    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
