use clap::{Args, Parser, Subcommand};

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
    #[arg(long, default_value = "New York")]
    pub location: String,

    /// Search keyword (e.g., Engineer, Rust)
    #[arg(short = 'k', long, default_value = "Engineer")]
    pub keyword: String,

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
