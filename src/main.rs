use anyhow::Result;
use autoseeker::{
    actors::{
        driven::{brightdata::client::BrightDataClient, file_saver::FileSaver},
        driving::cli::handler::CLIHandler,
    },
    adapters::driven::linkedin_brightdata::BrightDataLinkedinAdapter,
    app::services::{job::JobFetcherService, snapshot::SnapshotService},
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
    let job_saver = FileSaver::new();
    let job_fetcher_service = JobFetcherService::new(&linkedin_adapter, job_saver);
    let snapshot_handler_service = SnapshotService::new(&linkedin_adapter);

    let cli_handler = CLIHandler::new(job_fetcher_service, snapshot_handler_service);
    cli_handler.run(cli).await?;

    Ok(())
}
