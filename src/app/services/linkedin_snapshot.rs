use crate::app::ports::{
    driven::ForFetchingLinkedinSnapshot,
    driving::ForHandlingLinkedinSnapshot,
    types::{LinkedinDiscoverInput, Snapshot},
};
use anyhow::Result;

pub struct LinkedinSnapshotService<SC>
where
    SC: ForFetchingLinkedinSnapshot,
{
    snapshot_client: SC,
}

impl<SC> LinkedinSnapshotService<SC>
where
    SC: ForFetchingLinkedinSnapshot,
{
    pub fn new(snapshot_client: SC) -> Self {
        Self { snapshot_client }
    }
}

impl<SC> ForHandlingLinkedinSnapshot for LinkedinSnapshotService<SC>
where
    SC: ForFetchingLinkedinSnapshot,
{
    async fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        self.snapshot_client.list_snapshots().await
    }

    async fn download_snapshot(&self, snapshot_id: &str) -> Result<String> {
        self.snapshot_client.download_snapshot(snapshot_id).await
    }

    async fn trigger_fetching_jobs(
        &self,
        inputs: Vec<LinkedinDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> Result<String> {
        self.snapshot_client
            .trigger_fetching_jobs(inputs, limit_per_input)
            .await
    }
}
