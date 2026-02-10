use std::fmt::{Debug, Formatter};

use serde::Serialize;

#[derive(Serialize)]
pub struct Job {
    pub url: String,
    pub job_title: String,
    pub company_name: String,
    pub job_location: String,
    pub job_description: String,
    pub job_posted_date: String,
}

impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Job Title: {}\nCompany: {}\nLocation: {}\nPosted Date: {}\nURL: {}\nDescription: {}",
            self.job_title,
            self.company_name,
            self.job_location,
            self.job_posted_date,
            self.url,
            self.job_description
        )
    }
}

pub struct Snapshot {
    pub id: String,
    pub created: String,
    pub status: String,
    pub dataset_id: String,
    pub dataset_size: u64,
}

impl Debug for Snapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Snapshot ID: {}\nCreated: {}\nStatus: {}\nDataset ID: {}\nDataset Size: {}",
            self.id, self.created, self.status, self.dataset_id, self.dataset_size
        )
    }
}
