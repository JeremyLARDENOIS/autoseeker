use crate::{
    actors::driving::cli::{
        commands::{Cli, Commands, JobsCommands, SnapshotCommands},
        snapshot_table::SnapshotTable,
    },
    app::ports::driving::{ForHandlingJobs, ForHandlingSnapshot},
    app::ports::types::LinkedinDiscoverInput,
};
use anyhow::Result;
use std::fs;

#[derive(serde::Deserialize)]
struct InputsFile {
    input: Vec<LinkedinDiscoverInput>,
}

fn load_inputs_from_file(path: &std::path::Path) -> Result<Vec<LinkedinDiscoverInput>> {
    let body = fs::read_to_string(path)?;
    let parsed: InputsFile = serde_json::from_str(&body)?;
    if parsed.input.is_empty() {
        anyhow::bail!("inputs file contains an empty 'input' array");
    }
    Ok(parsed.input)
}

fn build_single_input(
    args: &crate::actors::driving::cli::commands::TriggerArgs,
) -> LinkedinDiscoverInput {
    LinkedinDiscoverInput {
        location: args
            .location
            .clone()
            .unwrap_or_else(|| "New York".to_string()),
        keyword: args
            .keyword
            .clone()
            .unwrap_or_else(|| "Engineer".to_string()),
        country: args.country.clone(),
        time_range: args.time_range.map(Into::into),
        job_type: args.job_type.map(Into::into),
        experience_level: args.experience_level.map(Into::into),
        remote: args.remote.map(Into::into),
        selective_search: args.selective_search.clone(),
        company: args.company.clone(),
        location_radius: args.location_radius.clone(),
    }
}

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
                    let inputs = if let Some(path) = &args.inputs_file {
                        load_inputs_from_file(path)?
                    } else {
                        vec![build_single_input(&args)]
                    };
                    self.job_fetcher_service
                        .get_jobs(inputs, args.limit_per_input)
                        .await?;
                }
            },
            Commands::Snapshot(snapshot_command) => match snapshot_command {
                SnapshotCommands::Trigger(args) => {
                    let inputs = if let Some(path) = &args.inputs_file {
                        load_inputs_from_file(path)?
                    } else {
                        vec![build_single_input(&args)]
                    };
                    let result = self
                        .snapshot_manager
                        .trigger_fetching_jobs(inputs, args.limit_per_input)
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
