use crate::app::ports::{driven::JobSaver, types::Job};
use anyhow::Result;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::Write;

pub struct FileSaver {}

impl Default for FileSaver {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSaver {
    pub fn new() -> Self {
        Self {}
    }
}

impl JobSaver for FileSaver {
    async fn save_jobs(&self, jobs: &[Job]) -> Result<()> {
        let json = to_string_pretty(jobs)?;
        let mut file = File::create("jobs.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
