use anyhow::{Context, Result};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

use crate::actors::driven::brightdata::snapshots::{Snapshot, SnapshotsResponse};

const BASE_URL: &str = "https://api.brightdata.com/datasets/v3";

pub struct BrightDataClient {
    client: reqwest::Client,
}

impl BrightDataClient {
    pub fn new(token: String) -> Result<Self> {
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
        Ok(Self { client })
    }

    async fn get(&self, endpoint: &str) -> Result<reqwest::Response> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("request failed")?;
        Ok(resp)
    }

    async fn post(&self, endpoint: &str, body: &serde_json::Value) -> Result<reqwest::Response> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let resp = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .context("request failed")?;
        Ok(resp)
    }

    pub async fn trigger_new_snapshot(
        &self,
        params: String,
        payload: serde_json::Value,
    ) -> Result<String> {
        let endpoint = format!("/trigger{}", params);

        let resp = self
            .post(&endpoint, &payload)
            .await
            .context("request failed")?;

        let status = resp.status();
        let body = resp.text().await.context("failed reading response body")?;
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, body);
        }
        Ok(body)
    }

    pub async fn list_snapshots(&self) -> Result<SnapshotsResponse> {
        let endpoint = "/snapshots";
        let resp = self.get(endpoint).await.context("request failed")?;

        let status = resp.status();
        let body = resp.text().await.context("failed reading response body")?;
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, body);
        }

        Ok(SnapshotsResponse {
            snapshots: serde_json::from_str::<Vec<Snapshot>>(&body)
                .context("failed parsing snapshots response")?,
        })
    }

    pub async fn get_snapshot(&self, snapshot_id: &str) -> Result<String> {
        let endpoint = format!("/snapshot/{}", snapshot_id);

        let resp = self.get(&endpoint).await.context("request failed")?;
        let status = resp.status();
        let body = resp.text().await.context("failed reading response body")?;
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, body);
        }
        Ok(body)
    }

    pub async fn is_snapshot_ready(&self, snapshot_id: &str) -> Result<bool> {
        let endpoint = format!("/progress/{}", snapshot_id);

        let resp = self.get(&endpoint).await.context("request failed")?;

        let status = resp.status();
        let body = resp.text().await.context("failed reading response body")?;
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, body);
        }

        let progress_response: serde_json::Value =
            serde_json::from_str(&body).context("failed parsing progress response")?;

        Ok(progress_response
            .get("status")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.eq_ignore_ascii_case("ready")))
    }
}
