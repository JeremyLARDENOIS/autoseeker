use anyhow::Result;

use crate::app::ports::types::{Job, LinkedinDiscoverInput, Snapshot};

pub trait ForFetchingLinkedinJobs {
    fn get_jobs(
        &self,
        inputs: Vec<LinkedinDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<Job>>>;
}

pub trait ForFetchingSnapshot {
    fn list_snapshots(&self) -> impl std::future::Future<Output = Result<Vec<Snapshot>>>;
    fn download_snapshot(
        &self,
        snapshot_id: &str,
    ) -> impl std::future::Future<Output = Result<String>>;
    fn trigger_fetching_jobs(
        &self,
        inputs: Vec<LinkedinDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> impl std::future::Future<Output = Result<String>>;
}

pub trait JobSaver {
    fn save_jobs(&self, jobs: &[Job]) -> impl std::future::Future<Output = Result<()>>;
}
