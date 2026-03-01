use anyhow::Result;
use autoseeker::{
    actors::{
        driven::{brightdata::client::BrightDataClient, file_saver::FileSaver},
        driving::cli::handler::CLIHandler,
    },
    adapters::driven::{
        indeed_brightdata::BrightDataIndeedAdapter, linkedin_brightdata::BrightDataLinkedinAdapter,
    },
    app::services::{
        indeed_job::IndeedJobFetcherService, indeed_snapshot::IndeedSnapshotService,
        job::JobFetcherService, linkedin_snapshot::LinkedinSnapshotService,
    },
};
use clap::Parser;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cli = autoseeker::actors::driving::cli::commands::Cli::parse();

    let brightdata_token =
        std::env::var("BRIGHTDATA_TOKEN").expect("BRIGHTDATA_TOKEN not set in environment");
    let client = BrightDataClient::new(brightdata_token)?;

    let linkedin_adapter = BrightDataLinkedinAdapter::new(&client);
    let indeed_adapter = BrightDataIndeedAdapter::new(&client);
    let job_saver = FileSaver::new();
    let linkedin_job_fetcher_service = JobFetcherService::new(&linkedin_adapter, &job_saver);
    let linkedin_snapshot_handler_service = LinkedinSnapshotService::new(&linkedin_adapter);

    let indeed_job_fetcher_service = IndeedJobFetcherService::new(&indeed_adapter, &job_saver);
    let indeed_snapshot_handler_service = IndeedSnapshotService::new(&indeed_adapter);

    let cli_handler = CLIHandler::new(
        linkedin_job_fetcher_service,
        linkedin_snapshot_handler_service,
        indeed_job_fetcher_service,
        indeed_snapshot_handler_service,
    );
    cli_handler.run(cli).await?;

    Ok(())
}
