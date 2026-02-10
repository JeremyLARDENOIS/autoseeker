use crate::{
    actors::driven::brightdata::{client::BrightDataClient, jobs::JobFetcherParams},
    app::ports::{
        driven::{ForFetchingJobs, ForFetchingSnapshot},
        types::{Job, Snapshot},
    },
};
use anyhow::{Context, Result};
use tokio::time::{sleep, Duration, Instant};
pub struct BrightDataLinkedinAdapter<'a> {
    client: &'a BrightDataClient,
}

const LINKEDIN_DATASET_ID: &str = "gd_lpfll7v5hcqtkxl6l";
const LIMIT_PER_INPUT: u32 = 100;

impl<'a> BrightDataLinkedinAdapter<'a> {
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
        location: String,
        keyword: String,
        limit_per_input: Option<u32>,
    ) -> Result<String> {
        let limit_per_input = limit_per_input.unwrap_or(LIMIT_PER_INPUT);
        let params = JobFetcherParams {
            location,
            keyword,
            limit_per_input,
            dataset_id: LINKEDIN_DATASET_ID.to_string(),
        };
        self.client.trigger_new_snapshot(&params).await
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

    fn parse_jobs_from_snapshot_body(body: &str) -> Result<Vec<Job>> {
        use crate::actors::driven::brightdata::jobs::JobPosting;

        let trimmed = body.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        let postings: Vec<JobPosting> = match serde_json::from_str::<Vec<JobPosting>>(trimmed) {
            Ok(v) => v,
            Err(_) => {
                // BrightData may return JSONL; try per-line parsing.
                let mut out = Vec::new();
                for line in trimmed.lines().map(str::trim).filter(|l| !l.is_empty()) {
                    if let Ok(p) = serde_json::from_str::<JobPosting>(line) {
                        out.push(p);
                    }
                }
                out
            }
        };

        let jobs = postings
            .into_iter()
            .map(|p| Job {
                url: p.url.unwrap_or_default(),
                job_title: p.job_title.unwrap_or_default(),
                company_name: p.company_name.unwrap_or_default(),
                job_location: p.job_location.unwrap_or_default(),
                job_description: p
                    .job_description_formatted
                    .or(p.job_summary)
                    .unwrap_or_default(),
                job_posted_date: p.job_posted_date.unwrap_or_default(),
            })
            .filter(|j| !j.url.is_empty() || !j.job_title.is_empty())
            .collect();

        Ok(jobs)
    }
}

impl ForFetchingJobs for &BrightDataLinkedinAdapter<'_> {
    async fn get_jobs(
        &self,
        location: String,
        keyword: String,
        limit_per_input: Option<u32>,
    ) -> Result<Vec<Job>> {
        let trigger_body = (**self)
            .trigger_fetching_jobs(location, keyword, limit_per_input)
            .await
            .context("failed to trigger BrightData snapshot")?;

        let snapshot_id =
            BrightDataLinkedinAdapter::parse_trigger_response_snapshot_id(&trigger_body)
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

        BrightDataLinkedinAdapter::parse_jobs_from_snapshot_body(&snapshot_body)
            .context("failed to parse jobs from snapshot")
    }
}

impl ForFetchingSnapshot for &BrightDataLinkedinAdapter<'_> {
    async fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        (**self).list_snapshots().await
    }

    async fn download_snapshot(&self, snapshot_id: &str) -> Result<String> {
        (**self).download_snapshot(snapshot_id).await
    }

    async fn trigger_fetching_jobs(
        &self,
        location: String,
        keyword: String,
        limit_per_input: Option<u32>,
    ) -> Result<String> {
        (**self)
            .trigger_fetching_jobs(location, keyword, limit_per_input)
            .await
    }
}
