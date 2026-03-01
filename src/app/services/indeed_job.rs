use crate::app::ports::{
    driven::{ForFetchingIndeedJobs, JobSaver},
    driving::ForHandlingIndeedJobs,
    types::{IndeedDiscoverInput, Job},
};
use anyhow::Result;

pub struct IndeedJobFetcherService<FJ, JS>
where
    FJ: ForFetchingIndeedJobs,
    JS: JobSaver,
{
    job_client: FJ,
    job_saver: JS,
}

impl<FJ, JS> IndeedJobFetcherService<FJ, JS>
where
    FJ: ForFetchingIndeedJobs,
    JS: JobSaver,
{
    pub fn new(job_client: FJ, job_saver: JS) -> Self {
        Self {
            job_client,
            job_saver,
        }
    }
}

impl<FJ, JS> ForHandlingIndeedJobs for IndeedJobFetcherService<FJ, JS>
where
    FJ: ForFetchingIndeedJobs,
    JS: JobSaver,
{
    async fn get_jobs(
        &self,
        inputs: Vec<IndeedDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> Result<Vec<Job>> {
        let jobs = self.job_client.get_jobs(inputs, limit_per_input).await?;
        self.job_saver.save_jobs(&jobs).await?;
        Ok(jobs)
    }
}
