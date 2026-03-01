use crate::{
    actors::driving::cli::{
        commands::{Cli, Commands, JobsCommands, SnapshotCommands},
        snapshot_table::SnapshotTable,
    },
    app::ports::driving::{
        ForHandlingIndeedJobs, ForHandlingIndeedSnapshot, ForHandlingLinkedinJobs,
        ForHandlingLinkedinSnapshot,
    },
    app::ports::types::{IndeedDiscoverInput, LinkedinDiscoverInput},
};
use anyhow::Result;
use std::fs;

fn apply_provider_filter(
    provider: crate::actors::driving::cli::commands::CliProvider,
    linkedin_inputs: &mut Vec<LinkedinDiscoverInput>,
    indeed_inputs: &mut Vec<IndeedDiscoverInput>,
) {
    match provider {
        crate::actors::driving::cli::commands::CliProvider::All => {}
        crate::actors::driving::cli::commands::CliProvider::Linkedin => {
            indeed_inputs.clear();
        }
        crate::actors::driving::cli::commands::CliProvider::Indeed => {
            linkedin_inputs.clear();
        }
    }
}

fn provider_for_inputs_file(
    args: &crate::actors::driving::cli::commands::TriggerArgs,
) -> crate::actors::driving::cli::commands::CliProvider {
    args.provider
        .unwrap_or(crate::actors::driving::cli::commands::CliProvider::All)
}

fn provider_for_flat_args(
    args: &crate::actors::driving::cli::commands::TriggerArgs,
) -> crate::actors::driving::cli::commands::CliProvider {
    args.provider
        .unwrap_or(crate::actors::driving::cli::commands::CliProvider::Linkedin)
}

fn has_nonempty(opt: &Option<String>) -> bool {
    opt.as_ref().is_some_and(|s| !s.trim().is_empty())
}

fn validate_flat_args_for_linkedin(
    args: &crate::actors::driving::cli::commands::TriggerArgs,
) -> Result<()> {
    if has_nonempty(&args.domain)
        || args.date_posted.is_some()
        || has_nonempty(&args.posted_by)
        || args.pay.is_some()
    {
        anyhow::bail!(
            "Indeed-specific flags provided but provider is linkedin. Use --provider indeed or remove Indeed flags."
        );
    }
    Ok(())
}

fn validate_flat_args_for_indeed(
    args: &crate::actors::driving::cli::commands::TriggerArgs,
) -> Result<()> {
    if args.time_range.is_some()
        || args.job_type.is_some()
        || args.experience_level.is_some()
        || args.remote.is_some()
        || has_nonempty(&args.selective_search)
        || has_nonempty(&args.company)
    {
        anyhow::bail!(
            "LinkedIn-specific flags provided but provider is indeed. Remove LinkedIn flags (e.g., --keyword/--time-range/--remote/...)."
        );
    }

    if !has_nonempty(&args.domain) {
        anyhow::bail!("Indeed flat mode requires --domain (e.g., fr.indeed.com)");
    }
    if !has_nonempty(&args.keyword) {
        anyhow::bail!("Indeed flat mode requires --keyword");
    }
    if !has_nonempty(&args.country) {
        anyhow::bail!("Indeed flat mode requires --country (e.g., FR)");
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct InputsFile {
    input: Vec<InputsEntry>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum InputsEntry {
    Tagged(TaggedInput),
    LinkedinLegacy(LinkedinDiscoverInput),
}

#[derive(serde::Deserialize)]
#[serde(tag = "provider", rename_all = "snake_case")]
enum TaggedInput {
    Linkedin(LinkedinDiscoverInput),
    Indeed(IndeedDiscoverInput),
}

fn split_inputs(
    entries: Vec<InputsEntry>,
) -> (Vec<LinkedinDiscoverInput>, Vec<IndeedDiscoverInput>) {
    let mut linkedin = Vec::new();
    let mut indeed = Vec::new();

    for entry in entries {
        match entry {
            InputsEntry::Tagged(tagged) => match tagged {
                TaggedInput::Linkedin(input) => linkedin.push(input),
                TaggedInput::Indeed(input) => indeed.push(input),
            },
            InputsEntry::LinkedinLegacy(input) => linkedin.push(input),
        }
    }

    (linkedin, indeed)
}

fn load_inputs_from_file(
    path: &std::path::Path,
) -> Result<(Vec<LinkedinDiscoverInput>, Vec<IndeedDiscoverInput>)> {
    let body = fs::read_to_string(path)?;
    let parsed: InputsFile = serde_json::from_str(&body)?;
    if parsed.input.is_empty() {
        anyhow::bail!("inputs file contains an empty 'input' array");
    }
    Ok(split_inputs(parsed.input))
}

fn build_single_linkedin_input(
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
        country: args.country.clone().unwrap_or_default(),
        time_range: args.time_range.map(Into::into),
        job_type: args.job_type.map(Into::into),
        experience_level: args.experience_level.map(Into::into),
        remote: args.remote.map(Into::into),
        selective_search: args.selective_search.clone().unwrap_or_default(),
        company: args.company.clone().unwrap_or_default(),
        location_radius: args.location_radius.clone().unwrap_or_default(),
    }
}

fn build_single_indeed_input(
    args: &crate::actors::driving::cli::commands::TriggerArgs,
) -> IndeedDiscoverInput {
    IndeedDiscoverInput {
        country: args.country.clone().unwrap_or_default(),
        domain: args.domain.clone().unwrap_or_default(),
        keyword: args.keyword.clone().unwrap_or_default(),
        location: args.location.clone().unwrap_or_default(),
        date_posted: args.date_posted.map(Into::into),
        posted_by: args.posted_by.clone().unwrap_or_default(),
        location_radius: args.location_radius.clone().unwrap_or_default(),
        pay: args.pay,
    }
}

pub struct CLIHandler<LJ, LS, IJ, IS>
where
    LJ: ForHandlingLinkedinJobs,
    LS: ForHandlingLinkedinSnapshot,
    IJ: ForHandlingIndeedJobs,
    IS: ForHandlingIndeedSnapshot,
{
    linkedin_job_fetcher: LJ,
    linkedin_snapshot_manager: LS,
    indeed_job_fetcher: IJ,
    indeed_snapshot_manager: IS,
}

impl<LJ, LS, IJ, IS> CLIHandler<LJ, LS, IJ, IS>
where
    LJ: ForHandlingLinkedinJobs,
    LS: ForHandlingLinkedinSnapshot,
    IJ: ForHandlingIndeedJobs,
    IS: ForHandlingIndeedSnapshot,
{
    pub fn new(
        linkedin_job_fetcher: LJ,
        linkedin_snapshot_manager: LS,
        indeed_job_fetcher: IJ,
        indeed_snapshot_manager: IS,
    ) -> Self {
        Self {
            linkedin_job_fetcher,
            linkedin_snapshot_manager,
            indeed_job_fetcher,
            indeed_snapshot_manager,
        }
    }

    pub async fn run(&self, cli: Cli) -> Result<()> {
        match cli.command {
            Commands::Jobs(jobs_command) => match jobs_command {
                JobsCommands::Get(args) => {
                    if let Some(path) = &args.inputs_file {
                        let (mut linkedin_inputs, mut indeed_inputs) = load_inputs_from_file(path)?;
                        apply_provider_filter(
                            provider_for_inputs_file(&args),
                            &mut linkedin_inputs,
                            &mut indeed_inputs,
                        );

                        if linkedin_inputs.is_empty() && indeed_inputs.is_empty() {
                            anyhow::bail!(
                                "no inputs left after applying provider filter (provider={:?})",
                                args.provider
                            );
                        }

                        if !linkedin_inputs.is_empty() {
                            self.linkedin_job_fetcher
                                .get_jobs(linkedin_inputs, args.limit_per_input)
                                .await?;
                        }

                        if !indeed_inputs.is_empty() {
                            self.indeed_job_fetcher
                                .get_jobs(indeed_inputs, args.limit_per_input)
                                .await?;
                        }
                    } else {
                        match provider_for_flat_args(&args) {
                            crate::actors::driving::cli::commands::CliProvider::All => {
                                anyhow::bail!(
                                    "flat-flag mode does not support provider=all; choose --provider linkedin|indeed or use --inputs-file"
                                );
                            }
                            crate::actors::driving::cli::commands::CliProvider::Linkedin => {
                                validate_flat_args_for_linkedin(&args)?;
                                self.linkedin_job_fetcher
                                    .get_jobs(
                                        vec![build_single_linkedin_input(&args)],
                                        args.limit_per_input,
                                    )
                                    .await?;
                            }
                            crate::actors::driving::cli::commands::CliProvider::Indeed => {
                                validate_flat_args_for_indeed(&args)?;
                                self.indeed_job_fetcher
                                    .get_jobs(
                                        vec![build_single_indeed_input(&args)],
                                        args.limit_per_input,
                                    )
                                    .await?;
                            }
                        }
                    }
                }
            },
            Commands::Snapshot(snapshot_command) => match snapshot_command {
                SnapshotCommands::Trigger(args) => {
                    if let Some(path) = &args.inputs_file {
                        let (mut linkedin_inputs, mut indeed_inputs) = load_inputs_from_file(path)?;
                        apply_provider_filter(
                            provider_for_inputs_file(&args),
                            &mut linkedin_inputs,
                            &mut indeed_inputs,
                        );

                        if linkedin_inputs.is_empty() && indeed_inputs.is_empty() {
                            anyhow::bail!(
                                "no inputs left after applying provider filter (provider={:?})",
                                args.provider
                            );
                        }

                        let mut outputs: Vec<String> = Vec::new();

                        if !linkedin_inputs.is_empty() {
                            let id = self
                                .linkedin_snapshot_manager
                                .trigger_fetching_jobs(linkedin_inputs, args.limit_per_input)
                                .await?;
                            outputs.push(format!("LinkedIn: {id}"));
                        }

                        if !indeed_inputs.is_empty() {
                            let id = self
                                .indeed_snapshot_manager
                                .trigger_fetching_jobs(indeed_inputs, args.limit_per_input)
                                .await?;
                            outputs.push(format!("Indeed: {id}"));
                        }

                        println!("Triggered fetching jobs:\n{}", outputs.join("\n"));
                    } else {
                        match provider_for_flat_args(&args) {
                            crate::actors::driving::cli::commands::CliProvider::All => {
                                anyhow::bail!(
                                    "flat-flag mode does not support provider=all; choose --provider linkedin|indeed or use --inputs-file"
                                );
                            }
                            crate::actors::driving::cli::commands::CliProvider::Linkedin => {
                                validate_flat_args_for_linkedin(&args)?;
                                let id = self
                                    .linkedin_snapshot_manager
                                    .trigger_fetching_jobs(
                                        vec![build_single_linkedin_input(&args)],
                                        args.limit_per_input,
                                    )
                                    .await?;
                                println!("Triggered fetching jobs: LinkedIn: {id}");
                            }
                            crate::actors::driving::cli::commands::CliProvider::Indeed => {
                                validate_flat_args_for_indeed(&args)?;
                                let id = self
                                    .indeed_snapshot_manager
                                    .trigger_fetching_jobs(
                                        vec![build_single_indeed_input(&args)],
                                        args.limit_per_input,
                                    )
                                    .await?;
                                println!("Triggered fetching jobs: Indeed: {id}");
                            }
                        }
                    }
                }
                SnapshotCommands::List => {
                    let snapshots = self.linkedin_snapshot_manager.list_snapshots().await?;
                    print!("{}", SnapshotTable::new(snapshots));
                }
                SnapshotCommands::Download(args) => {
                    let snapshot_data = self
                        .linkedin_snapshot_manager
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
