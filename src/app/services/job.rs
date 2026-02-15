use crate::app::ports::{
    driven::{ForFetchingLinkedinJobs, JobSaver},
    driving::ForHandlingJobs,
    types::{Job, LinkedinDiscoverInput},
};
use anyhow::Result;

pub struct JobFetcherService<FJ, JS>
where
    FJ: ForFetchingLinkedinJobs,
    JS: JobSaver,
{
    job_client: FJ,
    job_saver: JS,
}

impl<FJ, JS> JobFetcherService<FJ, JS>
where
    FJ: ForFetchingLinkedinJobs,
    JS: JobSaver,
{
    pub fn new(job_client: FJ, job_saver: JS) -> Self {
        Self {
            job_client,
            job_saver,
        }
    }
}

impl<FJ, JS> ForHandlingJobs for JobFetcherService<FJ, JS>
where
    FJ: ForFetchingLinkedinJobs,
    JS: JobSaver,
{
    async fn get_jobs(
        &self,
        inputs: Vec<LinkedinDiscoverInput>,
        limit_per_input: Option<u32>,
    ) -> Result<Vec<Job>> {
        let jobs = self.job_client.get_jobs(inputs, limit_per_input).await?;
        self.job_saver.save_jobs(&jobs).await?;
        Ok(jobs)
    }
}
