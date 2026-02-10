use anyhow::Result;

use crate::app::ports::types::{Job, Snapshot};

pub trait ForHandlingSnapshot {
    fn list_snapshots(&self) -> impl std::future::Future<Output = Result<Vec<Snapshot>>>;
    fn download_snapshot(
        &self,
        snapshot_id: &str,
    ) -> impl std::future::Future<Output = Result<String>>;
    fn trigger_fetching_jobs(
        &self,
        location: String,
        keyword: String,
        limit_per_input: Option<u32>,
    ) -> impl std::future::Future<Output = Result<String>>;
}

pub trait ForHandlingJobs {
    fn get_jobs(
        &self,
        location: String,
        keyword: String,
        limit_per_input: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<Job>>>;
}
