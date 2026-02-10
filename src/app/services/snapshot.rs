use crate::app::ports::{
    driven::ForFetchingSnapshot, driving::ForHandlingSnapshot, types::Snapshot,
};
use anyhow::Result;

pub struct SnapshotService<SC>
where
    SC: ForFetchingSnapshot,
{
    snapshot_client: SC,
}

impl<SC> SnapshotService<SC>
where
    SC: ForFetchingSnapshot,
{
    pub fn new(snapshot_client: SC) -> Self {
        Self { snapshot_client }
    }
}

impl<SC> ForHandlingSnapshot for SnapshotService<SC>
where
    SC: ForFetchingSnapshot,
{
    async fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        self.snapshot_client.list_snapshots().await
    }

    async fn download_snapshot(&self, snapshot_id: &str) -> Result<String> {
        self.snapshot_client.download_snapshot(snapshot_id).await
    }

    async fn trigger_fetching_jobs(
        &self,
        location: String,
        keyword: String,
        limit_per_input: Option<u32>,
    ) -> Result<String> {
        self.snapshot_client
            .trigger_fetching_jobs(location, keyword, limit_per_input)
            .await
    }
}
