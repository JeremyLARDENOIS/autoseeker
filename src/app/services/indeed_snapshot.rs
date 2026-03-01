use crate::app::ports::{
    driven::ForFetchingIndeedSnapshot,
    driving::ForHandlingIndeedSnapshot,
    types::{IndeedDiscoverInput, Snapshot},
};
use anyhow::Result;

pub struct IndeedSnapshotService<SC>
where
    SC: ForFetchingIndeedSnapshot,
{
    snapshot_client: SC,
}

impl<SC> IndeedSnapshotService<SC>
where
    SC: ForFetchingIndeedSnapshot,
{
    pub fn new(snapshot_client: SC) -> Self {
        Self { snapshot_client }
    }
}

impl<SC> ForHandlingIndeedSnapshot for IndeedSnapshotService<SC>
where
    SC: ForFetchingIndeedSnapshot,
{
    async fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        self.snapshot_client.list_snapshots().await
    }

    async fn download_snapshot(&self, snapshot_id: &str) -> Result<String> {
        self.snapshot_client.download_snapshot(snapshot_id).await
    }

    async fn trigger_fetching_jobs(
        &self,
        inputs: Vec<IndeedDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> Result<String> {
        self.snapshot_client
            .trigger_fetching_jobs(inputs, limit_per_input)
            .await
    }
}
