use crate::{
    actors::driving::cli::{
        commands::{Cli, Commands, JobsCommands, SnapshotCommands},
        snapshot_table::SnapshotTable,
    },
    app::ports::driving::{ForHandlingJobs, ForHandlingSnapshot},
};
use anyhow::Result;
use std::fs;

pub struct CLIHandler<HJ, HS>
where
    HJ: ForHandlingJobs,
    HS: ForHandlingSnapshot,
{
    job_fetcher_service: HJ,
    snapshot_manager: HS,
}

impl<HJ, HS> CLIHandler<HJ, HS>
where
    HJ: ForHandlingJobs,
    HS: ForHandlingSnapshot,
{
    pub fn new(job_fetcher_service: HJ, snapshot_manager: HS) -> Self {
        Self {
            job_fetcher_service,
            snapshot_manager,
        }
    }

    pub async fn run(&self, cli: Cli) -> Result<()> {
        match cli.command {
            Commands::Jobs(jobs_command) => match jobs_command {
                JobsCommands::Get(args) => {
                    self.job_fetcher_service
                        .get_jobs(args.location, args.keyword, args.limit_per_input)
                        .await?;
                }
            },
            Commands::Snapshot(snapshot_command) => match snapshot_command {
                SnapshotCommands::Trigger(args) => {
                    let result = self
                        .snapshot_manager
                        .trigger_fetching_jobs(args.location, args.keyword, args.limit_per_input)
                        .await?;
                    println!("Triggered fetching jobs: {}", result);
                }
                SnapshotCommands::List => {
                    let snapshots = self.snapshot_manager.list_snapshots().await?;
                    print!("{}", SnapshotTable::new(snapshots));
                }
                SnapshotCommands::Download(args) => {
                    let snapshot_data = self
                        .snapshot_manager
                        .download_snapshot(&args.snapshot_id)
                        .await?;

                    let path = args
                        .output
                        .unwrap_or_else(|| std::path::PathBuf::from("snapshot.json"));
                    fs::write(&path, snapshot_data)?;
                    eprintln!("Saved to {}", path.display());
                }
            },
        }
        Ok(())
    }
}
