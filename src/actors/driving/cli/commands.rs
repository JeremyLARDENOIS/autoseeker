use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::app::ports::types::{ExperienceLevel, IndeedDatePosted, JobType, Remote, TimeRange};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliProvider {
    #[value(name = "all", alias = "both")]
    All,
    #[value(name = "linkedin")]
    Linkedin,
    #[value(name = "indeed")]
    Indeed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliIndeedDatePosted {
    #[value(
        name = "Last 24 hours",
        alias = "1",
        alias = "last-24-hours",
        alias = "last_24_hours"
    )]
    Last24Hours,
    #[value(
        name = "Last 3 days",
        alias = "3",
        alias = "last-3-days",
        alias = "last_3_days"
    )]
    Last3Days,
    #[value(
        name = "Last 7 days",
        alias = "7",
        alias = "last-7-days",
        alias = "last_7_days"
    )]
    Last7Days,
    #[value(
        name = "Last 14 days",
        alias = "14",
        alias = "last-14-days",
        alias = "last_14_days"
    )]
    Last14Days,
}

impl From<CliIndeedDatePosted> for IndeedDatePosted {
    fn from(value: CliIndeedDatePosted) -> Self {
        match value {
            CliIndeedDatePosted::Last24Hours => IndeedDatePosted::Last24Hours,
            CliIndeedDatePosted::Last3Days => IndeedDatePosted::Last3Days,
            CliIndeedDatePosted::Last7Days => IndeedDatePosted::Last7Days,
            CliIndeedDatePosted::Last14Days => IndeedDatePosted::Last14Days,
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
        "domain",
        "date_posted",
        "posted_by",
        "pay",
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

    /// Provider filter when using --inputs-file.
    /// Use this to run only one provider even if the file contains both.
    /// In flat-flag mode (no --inputs-file), this selects which provider to run.
    #[arg(long, value_enum)]
    pub provider: Option<CliProvider>,

    /// Country code (e.g., FR)
    #[arg(long)]
    pub country: Option<String>,

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
    #[arg(long)]
    pub selective_search: Option<String>,

    /// Company name
    #[arg(long)]
    pub company: Option<String>,

    /// Location radius
    #[arg(long)]
    pub location_radius: Option<String>,

    // --- Indeed-only flat flags ---
    /// Indeed: domain (e.g., fr.indeed.com)
    #[arg(long)]
    pub domain: Option<String>,

    /// Indeed: date_posted
    #[arg(long, value_enum)]
    pub date_posted: Option<CliIndeedDatePosted>,

    /// Indeed: posted_by
    #[arg(long)]
    pub posted_by: Option<String>,

    /// Indeed: pay
    #[arg(long)]
    pub pay: Option<u32>,

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
