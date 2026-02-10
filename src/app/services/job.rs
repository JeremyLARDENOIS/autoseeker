use crate::app::ports::{
    driven::{ForFetchingJobs, JobSaver},
    driving::ForHandlingJobs,
    types::Job,
};
use anyhow::Result;

pub struct JobFetcherService<FJ, JS>
where
    FJ: ForFetchingJobs,
    JS: JobSaver,
{
    job_client: FJ,
    job_saver: JS,
}

impl<FJ, JS> JobFetcherService<FJ, JS>
where
    FJ: ForFetchingJobs,
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
    FJ: ForFetchingJobs,
    JS: JobSaver,
{
    async fn get_jobs(
        &self,
        location: String,
        keyword: String,
        limit_per_input: Option<u32>,
    ) -> Result<Vec<Job>> {
        let jobs = self
            .job_client
            .get_jobs(location, keyword, limit_per_input)
            .await?;
        self.job_saver.save_jobs(&jobs).await?;
        Ok(jobs)
    }
}
