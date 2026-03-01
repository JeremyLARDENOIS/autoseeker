use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

fn deserialize_empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(s) if s.trim().is_empty() => Ok(None),
        other => T::deserialize(other)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

fn deserialize_indeed_date_posted<'de, D>(
    deserializer: D,
) -> Result<Option<IndeedDatePosted>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    let from_legacy_num = |n: u64| -> Option<IndeedDatePosted> {
        match n {
            1 => Some(IndeedDatePosted::Last24Hours),
            3 => Some(IndeedDatePosted::Last3Days),
            7 => Some(IndeedDatePosted::Last7Days),
            14 => Some(IndeedDatePosted::Last14Days),
            _ => None,
        }
    };

    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(s) if s.trim().is_empty() => Ok(None),
        serde_json::Value::String(s) => {
            if let Ok(n) = s.trim().parse::<u64>() {
                if let Some(v) = from_legacy_num(n) {
                    return Ok(Some(v));
                }
            }

            let v = serde_json::Value::String(s);
            IndeedDatePosted::deserialize(v)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        serde_json::Value::Number(n) => n
            .as_u64()
            .and_then(from_legacy_num)
            .ok_or_else(|| serde::de::Error::custom("invalid date_posted number"))
            .map(Some),
        other => Err(serde::de::Error::custom(format!(
            "invalid date_posted value: {other}"
        ))),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeRange {
    #[serde(rename = "Past 24 hours")]
    Past24Hours,
    #[serde(rename = "Past week")]
    PastWeek,
    #[serde(rename = "Past month")]
    PastMonth,
    #[serde(rename = "Any time")]
    AnyTime,
}

impl TimeRange {
    pub fn as_brightdata_str(self) -> &'static str {
        match self {
            TimeRange::Past24Hours => "Past 24 hours",
            TimeRange::PastWeek => "Past week",
            TimeRange::PastMonth => "Past month",
            TimeRange::AnyTime => "Any time",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobType {
    #[serde(rename = "Full-time")]
    FullTime,
    #[serde(rename = "Part-time")]
    PartTime,
    #[serde(rename = "Contract")]
    Contract,
    #[serde(rename = "Temporary")]
    Temporary,
    #[serde(rename = "Volunteer")]
    Volunteer,
}

impl JobType {
    pub fn as_brightdata_str(self) -> &'static str {
        match self {
            JobType::FullTime => "Full-time",
            JobType::PartTime => "Part-time",
            JobType::Contract => "Contract",
            JobType::Temporary => "Temporary",
            JobType::Volunteer => "Volunteer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperienceLevel {
    #[serde(rename = "Internship")]
    Internship,
    #[serde(rename = "Entry level")]
    EntryLevel,
    #[serde(rename = "Associate")]
    Associate,
    #[serde(rename = "Mid-Senior level")]
    MidSeniorLevel,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Executive")]
    Executive,
}

impl ExperienceLevel {
    pub fn as_brightdata_str(self) -> &'static str {
        match self {
            ExperienceLevel::Internship => "Internship",
            ExperienceLevel::EntryLevel => "Entry level",
            ExperienceLevel::Associate => "Associate",
            ExperienceLevel::MidSeniorLevel => "Mid-Senior level",
            ExperienceLevel::Director => "Director",
            ExperienceLevel::Executive => "Executive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Remote {
    #[serde(rename = "On-site")]
    OnSite,
    #[serde(rename = "Remote")]
    Remote,
    #[serde(rename = "Hybrid")]
    Hybrid,
}

impl Remote {
    pub fn as_brightdata_str(self) -> &'static str {
        match self {
            Remote::OnSite => "On-site",
            Remote::Remote => "Remote",
            Remote::Hybrid => "Hybrid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndeedDatePosted {
    #[serde(rename = "Last 24 hours")]
    Last24Hours,
    #[serde(rename = "Last 3 days")]
    Last3Days,
    #[serde(rename = "Last 7 days")]
    Last7Days,
    #[serde(rename = "Last 14 days")]
    Last14Days,
}

impl IndeedDatePosted {
    pub fn as_brightdata_str(self) -> &'static str {
        match self {
            IndeedDatePosted::Last24Hours => "Last 24 hours",
            IndeedDatePosted::Last3Days => "Last 3 days",
            IndeedDatePosted::Last7Days => "Last 7 days",
            IndeedDatePosted::Last14Days => "Last 14 days",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedinDiscoverInput {
    #[serde(default)]
    pub location: String,

    #[serde(default)]
    pub keyword: String,

    #[serde(default)]
    pub country: String,

    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub time_range: Option<TimeRange>,

    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub job_type: Option<JobType>,

    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub experience_level: Option<ExperienceLevel>,

    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub remote: Option<Remote>,

    #[serde(default)]
    pub selective_search: String,

    #[serde(default)]
    pub company: String,

    #[serde(default)]
    pub location_radius: String,
}

impl LinkedinDiscoverInput {
    pub fn into_brightdata_payload(self) -> serde_json::Value {
        serde_json::json!({
            "location": self.location,
            "keyword": self.keyword,
            "country": self.country,
            "time_range": self
                .time_range
                .map(|v| v.as_brightdata_str())
                .unwrap_or(""),
            "job_type": self.job_type.map(|v| v.as_brightdata_str()).unwrap_or(""),
            "experience_level": self
                .experience_level
                .map(|v| v.as_brightdata_str())
                .unwrap_or(""),
            "remote": self.remote.map(|v| v.as_brightdata_str()).unwrap_or(""),
            "selective_search": self.selective_search,
            "company": self.company,
            "location_radius": self.location_radius,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndeedDiscoverInput {
    #[serde(default)]
    pub country: String,

    #[serde(default)]
    pub domain: String,

    #[serde(default, alias = "keyword_search")]
    pub keyword: String,

    #[serde(default)]
    pub location: String,

    #[serde(default, deserialize_with = "deserialize_indeed_date_posted")]
    pub date_posted: Option<IndeedDatePosted>,

    #[serde(default)]
    pub posted_by: String,

    #[serde(default)]
    pub location_radius: String,

    #[serde(default)]
    pub pay: Option<u32>,
}

impl IndeedDiscoverInput {
    pub fn into_brightdata_payload(self) -> serde_json::Value {
        let mut payload = serde_json::json!({
            "country": self.country,
            "domain": self.domain,
            "keyword_search": self.keyword,
            "location": self.location,
            "date_posted": self
                .date_posted
                .map(|v| v.as_brightdata_str())
                .unwrap_or(""),
            "posted_by": self.posted_by,
            "location_radius": self.location_radius,
            "pay": self.pay,
        });

        if payload.get("pay").is_some_and(|v| v.is_null()) {
            if let Some(obj) = payload.as_object_mut() {
                obj.remove("pay");
            }
        }

        payload
    }
}
