use crate::{
    actors::driven::brightdata::client::BrightDataClient,
    app::ports::{
        driven::{ForFetchingIndeedJobs, ForFetchingIndeedSnapshot},
        types::{IndeedDiscoverInput, Job, Snapshot},
    },
};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use tokio::time::{Duration, Instant, sleep};

pub struct BrightDataIndeedAdapter<'a> {
    client: &'a BrightDataClient,
}

const INDEED_DATASET_ID: &str = "gd_l4dx9j9sscpvs7no2";
const LIMIT_PER_INPUT: u32 = 100;

impl<'a> BrightDataIndeedAdapter<'a> {
    pub fn new(client: &'a BrightDataClient) -> Self {
        Self { client }
    }

    async fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        let resp = self.client.list_snapshots().await?;
        Ok(resp
            .snapshots
            .into_iter()
            .map(|s| Snapshot {
                id: s.id,
                created: s.created,
                status: s.status,
                dataset_id: s.dataset_id,
                dataset_size: s.dataset_size,
            })
            .collect())
    }

    async fn download_snapshot(&self, snapshot_id: &str) -> Result<String> {
        self.client.get_snapshot(snapshot_id).await
    }

    async fn trigger_fetching_jobs(
        &self,
        inputs: Vec<IndeedDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> Result<String> {
        let limit_per_input = limit_per_input.unwrap_or(LIMIT_PER_INPUT);
        let params = format!(
            "?dataset_id={}&notify=false&include_errors=true&type=discover_new&discover_by=keyword&limit_per_input={}",
            INDEED_DATASET_ID, limit_per_input
        );

        let payload_inputs: Vec<Value> = inputs
            .into_iter()
            .map(IndeedDiscoverInput::into_brightdata_payload)
            .collect();

        self.client
            .trigger_new_snapshot(params, json!({"input": payload_inputs}))
            .await
    }

    async fn wait_snapshot_ready(&self, snapshot_id: &str) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(10 * 60);
        let mut sleep_for = Duration::from_secs(2);

        loop {
            if Instant::now() >= deadline {
                anyhow::bail!("Timed out waiting for snapshot {snapshot_id} to be ready");
            }

            if self
                .client
                .is_snapshot_ready(snapshot_id)
                .await
                .context("failed checking snapshot progress")?
            {
                println!("Snapshot is ready");
                return Ok(());
            }

            println!(
                "Snapshot {} not ready yet, sleeping for {} seconds...",
                snapshot_id,
                sleep_for.as_secs()
            );
            sleep(sleep_for).await;
            sleep_for = std::cmp::min(sleep_for * 2, Duration::from_secs(15));
        }
    }

    fn parse_trigger_response_snapshot_id(body: &str) -> Result<String> {
        let trimmed = body.trim();
        if trimmed.is_empty() {
            anyhow::bail!("BrightData trigger returned empty body");
        }

        if let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed) {
            match value {
                serde_json::Value::String(s) => return Ok(s),
                serde_json::Value::Object(map) => {
                    for key in ["snapshot_id", "snapshotId", "id"] {
                        if let Some(serde_json::Value::String(s)) = map.get(key) {
                            return Ok(s.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(trimmed.to_string())
    }

    fn extract_string(value: &Value, keys: &[&str]) -> String {
        let obj = match value.as_object() {
            Some(o) => o,
            None => return String::new(),
        };

        for key in keys {
            if let Some(v) = obj.get(*key) {
                if let Some(s) = v.as_str()
                    && !s.trim().is_empty()
                {
                    return s.to_string();
                }

                if let Some(o) = v.as_object()
                    && let Some(s) = o.get("name").and_then(|x| x.as_str())
                    && !s.trim().is_empty()
                {
                    return s.to_string();
                }

                if let Some(arr) = v.as_array() {
                    for item in arr {
                        if let Some(s) = item.as_str()
                            && !s.trim().is_empty()
                        {
                            return s.to_string();
                        }
                    }
                }
            }
        }

        String::new()
    }

    fn parse_jobs_from_snapshot_body(body: &str) -> Result<Vec<Job>> {
        let trimmed = body.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        let records: Vec<Value> = match serde_json::from_str::<Vec<Value>>(trimmed) {
            Ok(v) => v,
            Err(_) => {
                let mut out = Vec::new();
                for line in trimmed.lines().map(str::trim).filter(|l| !l.is_empty()) {
                    if let Ok(v) = serde_json::from_str::<Value>(line) {
                        out.push(v);
                    }
                }
                out
            }
        };

        let mut jobs = Vec::new();
        for record in records {
            let Some(obj) = record.as_object() else {
                continue;
            };

            if obj.contains_key("error") || obj.contains_key("errors") {
                continue;
            }

            let url = Self::extract_string(&record, &["url", "job_url", "jobUrl", "link"]);
            let job_title =
                Self::extract_string(&record, &["job_title", "title", "jobTitle", "position"]);
            let company_name = Self::extract_string(
                &record,
                &["company_name", "company", "companyName", "employer"],
            );
            let job_location =
                Self::extract_string(&record, &["job_location", "location", "jobLocation"]);
            let job_description = Self::extract_string(
                &record,
                &[
                    "description_text",
                    "description",
                    "job_description",
                    "job_description_formatted",
                    "job_summary",
                ],
            );
            let job_posted_date = Self::extract_string(
                &record,
                &["job_posted_date", "posted_at", "date_posted", "date"],
            );

            let job = Job {
                url,
                job_title,
                company_name,
                job_location,
                job_description,
                job_posted_date,
            };

            if job.url.is_empty() && job.job_title.is_empty() {
                continue;
            }

            jobs.push(job);
        }

        Ok(jobs)
    }
}

impl ForFetchingIndeedJobs for &BrightDataIndeedAdapter<'_> {
    async fn get_jobs(
        &self,
        inputs: Vec<IndeedDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> Result<Vec<Job>> {
        let trigger_body = (**self)
            .trigger_fetching_jobs(inputs, limit_per_input)
            .await
            .context("failed to trigger BrightData snapshot")?;

        let snapshot_id =
            BrightDataIndeedAdapter::parse_trigger_response_snapshot_id(&trigger_body)
                .context("failed to extract snapshot id from trigger response")?;

        (**self)
            .wait_snapshot_ready(&snapshot_id)
            .await
            .context("snapshot did not become ready")?;

        println!("Downloading Snapshot {}", snapshot_id);
        let snapshot_body = (**self)
            .download_snapshot(&snapshot_id)
            .await
            .context("failed to download snapshot")?;

        BrightDataIndeedAdapter::parse_jobs_from_snapshot_body(&snapshot_body)
            .context("failed to parse jobs from snapshot")
    }
}

impl ForFetchingIndeedSnapshot for &BrightDataIndeedAdapter<'_> {
    async fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        (**self).list_snapshots().await
    }

    async fn download_snapshot(&self, snapshot_id: &str) -> Result<String> {
        (**self).download_snapshot(snapshot_id).await
    }

    async fn trigger_fetching_jobs(
        &self,
        inputs: Vec<IndeedDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> Result<String> {
        (**self)
            .trigger_fetching_jobs(inputs, limit_per_input)
            .await
    }
}
