use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const LIMIT_PER_INPUT: u32 = 10;
const DATASET_ID: &str = "gd_lpfll7v5hcqtkxl6l";

#[derive(Debug, Deserialize, Serialize)]
pub struct JobPosting {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub job_posting_id: Option<String>,
    #[serde(default)]
    pub job_title: Option<String>,
    #[serde(default)]
    pub company_name: Option<String>,
    #[serde(default)]
    pub job_location: Option<String>,
    #[serde(default)]
    pub job_summary: Option<String>,
    #[serde(default)]
    pub job_description_formatted: Option<String>,
    #[serde(default)]
    pub job_seniority_level: Option<String>,
    #[serde(default)]
    pub job_employment_type: Option<String>,
    #[serde(default)]
    pub job_function: Option<String>,
    #[serde(default)]
    pub job_industries: Option<String>,
    #[serde(default)]
    pub job_posted_date: Option<String>,
    #[serde(default)]
    pub application_availability: Option<bool>,
    #[serde(default)]
    pub discovery_input: Option<Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct JobFetcherParams {
    pub location: String,
    pub keyword: String,
    pub limit_per_input: u32,
    pub dataset_id: String,
}

impl JobFetcherParams {
    pub fn new(location: String, keyword: String) -> Self {
        Self {
            location,
            keyword,
            limit_per_input: LIMIT_PER_INPUT,
            dataset_id: DATASET_ID.to_string(),
        }
    }

    pub fn set_limit_per_input(&mut self, limit: u32) {
        self.limit_per_input = limit;
    }

    pub fn set_dataset_id(&mut self, dataset_id: String) {
        self.dataset_id = dataset_id;
    }

    pub fn get_params(&self) -> String {
        format!(
            "dataset_id={}&notify=false&include_errors=true&type=discover_new&discover_by=keyword&limit_per_input={}",
            self.dataset_id, self.limit_per_input
        )
    }

    pub fn get_payload(&self) -> Value {
        json!({
            "input": [
                {
                    "location": self.location,
                    "keyword": self.keyword,
                    "country": "",
                    "time_range": "",
                    "job_type": "",
                    "experience_level": "",
                    "remote": "",
                    "company": "",
                    "location_radius": ""
                }
            ]
        })
    }
}
