use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Deserializer, Serialize};

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
            "time_range": self.time_range.map(|v| v.as_brightdata_str()).unwrap_or(""),
            "job_type": self.job_type.map(|v| v.as_brightdata_str()).unwrap_or(""),
            "experience_level": self.experience_level.map(|v| v.as_brightdata_str()).unwrap_or(""),
            "remote": self.remote.map(|v| v.as_brightdata_str()).unwrap_or(""),
            "selective_search": self.selective_search,
            "company": self.company,
            "location_radius": self.location_radius,
        })
    }
}
