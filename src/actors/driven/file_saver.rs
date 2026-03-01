use crate::app::ports::{driven::JobSaver, types::Job};
use anyhow::{Context, Result};
use serde_json::to_string_pretty;

fn tmp_path_for(path: &std::path::Path) -> std::path::PathBuf {
    match path.file_name() {
        Some(name) => {
            let mut tmp_name = name.to_os_string();
            tmp_name.push(".tmp");
            path.with_file_name(tmp_name)
        }
        None => std::path::PathBuf::from("jobs.json.tmp"),
    }
}

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

    fn save_jobs_impl(&self, jobs: &[Job]) -> Result<()> {
        let path = std::path::Path::new("jobs.json");

        let mut existing: Vec<Job> = if path.exists() {
            let body = std::fs::read_to_string(path)
                .with_context(|| format!("failed reading {}", path.display()))?;
            serde_json::from_str::<Vec<Job>>(&body)
                .with_context(|| format!("{} is not a valid JSON array of jobs", path.display()))?
        } else {
            Vec::new()
        };

        existing.extend(jobs.iter().cloned());

        let json = to_string_pretty(&existing)?;

        let tmp_path = tmp_path_for(path);
        std::fs::write(&tmp_path, json.as_bytes())
            .with_context(|| format!("failed writing {}", tmp_path.display()))?;
        std::fs::rename(&tmp_path, path).with_context(|| {
            format!(
                "failed renaming {} to {}",
                tmp_path.display(),
                path.display()
            )
        })?;

        Ok(())
    }
}

impl JobSaver for FileSaver {
    async fn save_jobs(&self, jobs: &[Job]) -> Result<()> {
        self.save_jobs_impl(jobs)
    }
}

impl JobSaver for &FileSaver {
    async fn save_jobs(&self, jobs: &[Job]) -> Result<()> {
        (*self).save_jobs_impl(jobs)
    }
}
