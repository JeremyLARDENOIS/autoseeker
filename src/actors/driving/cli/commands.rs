use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::app::ports::types::{ExperienceLevel, JobType, Remote, TimeRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliTimeRange {
    #[value(
        name = "Past 24 hours",
        alias = "past-24-hours",
        alias = "past_24_hours"
    )]
    Past24Hours,
    #[value(name = "Past week", alias = "past-week", alias = "past_week")]
    PastWeek,
    #[value(name = "Past month", alias = "past-month", alias = "past_month")]
    PastMonth,
    #[value(name = "Any time", alias = "any-time", alias = "any_time")]
    AnyTime,
}

impl From<CliTimeRange> for TimeRange {
    fn from(value: CliTimeRange) -> Self {
        match value {
            CliTimeRange::Past24Hours => TimeRange::Past24Hours,
            CliTimeRange::PastWeek => TimeRange::PastWeek,
            CliTimeRange::PastMonth => TimeRange::PastMonth,
            CliTimeRange::AnyTime => TimeRange::AnyTime,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliJobType {
    #[value(name = "Full-time", alias = "full-time", alias = "full_time")]
    FullTime,
    #[value(name = "Part-time", alias = "part-time", alias = "part_time")]
    PartTime,
    #[value(name = "Contract", alias = "contract")]
    Contract,
    #[value(name = "Temporary", alias = "temporary")]
    Temporary,
    #[value(name = "Volunteer", alias = "volunteer")]
    Volunteer,
}

impl From<CliJobType> for JobType {
    fn from(value: CliJobType) -> Self {
        match value {
            CliJobType::FullTime => JobType::FullTime,
            CliJobType::PartTime => JobType::PartTime,
            CliJobType::Contract => JobType::Contract,
            CliJobType::Temporary => JobType::Temporary,
            CliJobType::Volunteer => JobType::Volunteer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliExperienceLevel {
    #[value(name = "Internship", alias = "internship")]
    Internship,
    #[value(name = "Entry level", alias = "entry-level", alias = "entry_level")]
    EntryLevel,
    #[value(name = "Associate", alias = "associate")]
    Associate,
    #[value(
        name = "Mid-Senior level",
        alias = "mid-senior-level",
        alias = "mid_senior_level"
    )]
    MidSeniorLevel,
    #[value(name = "Director", alias = "director")]
    Director,
    #[value(name = "Executive", alias = "executive")]
    Executive,
}

impl From<CliExperienceLevel> for ExperienceLevel {
    fn from(value: CliExperienceLevel) -> Self {
        match value {
            CliExperienceLevel::Internship => ExperienceLevel::Internship,
            CliExperienceLevel::EntryLevel => ExperienceLevel::EntryLevel,
            CliExperienceLevel::Associate => ExperienceLevel::Associate,
            CliExperienceLevel::MidSeniorLevel => ExperienceLevel::MidSeniorLevel,
            CliExperienceLevel::Director => ExperienceLevel::Director,
            CliExperienceLevel::Executive => ExperienceLevel::Executive,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliRemote {
    #[value(
        name = "On-site",
        alias = "on-site",
        alias = "on_site",
        alias = "onsite"
    )]
    OnSite,
    #[value(name = "Remote", alias = "remote")]
    Remote,
    #[value(name = "Hybrid", alias = "hybrid")]
    Hybrid,
}

impl From<CliRemote> for Remote {
    fn from(value: CliRemote) -> Self {
        match value {
            CliRemote::OnSite => Remote::OnSite,
            CliRemote::Remote => Remote::Remote,
            CliRemote::Hybrid => Remote::Hybrid,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Autoseeker CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Job-related commands
    #[command(subcommand)]
    Jobs(JobsCommands),
    /// Snapshot-related commands
    #[command(subcommand)]
    Snapshot(SnapshotCommands),
}

#[derive(Subcommand, Debug)]
pub enum SnapshotCommands {
    /// Trigger job fetching from Bright Data
    Trigger(TriggerArgs),
    /// List available snapshots
    List,
    /// Download a specific snapshot
    Download(DownloadArgs),
}

#[derive(Subcommand, Debug)]
pub enum JobsCommands {
    /// Get jobs
    Get(TriggerArgs),
}

#[derive(Parser, Debug)]
pub struct TriggerArgs {
    /// Job location (city or region)
    #[arg(long)]
    pub location: Option<String>,

    /// Search keyword (e.g., Engineer, Rust)
    #[arg(short = 'k', long)]
    pub keyword: Option<String>,

    /// Read Bright Data inputs from a JSON file (format: {"input": [ ... ]}); forbids flat criteria flags.
    #[arg(long, value_name = "FILE", conflicts_with_all = [
        "location",
        "keyword",
        "country",
        "time_range",
        "job_type",
        "experience_level",
        "remote",
        "selective_search",
        "company",
        "location_radius",
    ])]
    pub inputs_file: Option<PathBuf>,

    /// Country code (e.g., FR)
    #[arg(long, default_value = "")]
    pub country: String,

    /// Time range
    #[arg(long, value_enum)]
    pub time_range: Option<CliTimeRange>,

    /// Job type
    #[arg(long, value_enum)]
    pub job_type: Option<CliJobType>,

    /// Experience level
    #[arg(long, value_enum)]
    pub experience_level: Option<CliExperienceLevel>,

    /// Remote mode
    #[arg(long, value_enum)]
    pub remote: Option<CliRemote>,

    /// Selective search
    #[arg(long, default_value = "")]
    pub selective_search: String,

    /// Company name
    #[arg(long, default_value = "")]
    pub company: String,

    /// Location radius
    #[arg(long, default_value = "")]
    pub location_radius: String,

    /// Limit per input for Bright Data discovery
    #[arg(long = "limit")]
    pub limit_per_input: Option<u32>,
}

#[derive(Args, Debug)]
pub struct DownloadArgs {
    /// Snapshot ID to download (positional)
    pub snapshot_id: String,
    /// Write output to file instead of stdout
    #[arg(short, long, default_value = "snapshot.json")]
    pub output: Option<std::path::PathBuf>,
}
