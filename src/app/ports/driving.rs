use anyhow::Result;

use crate::app::ports::types::{IndeedDiscoverInput, Job, LinkedinDiscoverInput, Snapshot};

pub trait ForHandlingLinkedinSnapshot {
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

pub trait ForHandlingLinkedinJobs {
    fn get_jobs(
        &self,
        inputs: Vec<LinkedinDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<Job>>>;
}

pub trait ForHandlingIndeedSnapshot {
    fn list_snapshots(&self) -> impl std::future::Future<Output = Result<Vec<Snapshot>>>;
    fn download_snapshot(
        &self,
        snapshot_id: &str,
    ) -> impl std::future::Future<Output = Result<String>>;
    fn trigger_fetching_jobs(
        &self,
        inputs: Vec<IndeedDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> impl std::future::Future<Output = Result<String>>;
}

pub trait ForHandlingIndeedJobs {
    fn get_jobs(
        &self,
        inputs: Vec<IndeedDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<Job>>>;
}
