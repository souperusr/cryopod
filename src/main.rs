mod cryopod;
mod error;
mod backend;
mod constants;
mod podman;

use crate::podman::PodmanBackend;

use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    Enter,
    Develop(DevelopArgs)
}

#[derive(Args)]
pub struct DevelopArgs {
    project: Option<PathBuf>
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setups error printing
    color_eyre::install()?;

    let cli = Cli::parse();

    let mut backend = PodmanBackend::new().await?;
    let _ = backend.run().await?;

    match &cli.command {
        Some(Commands::Enter) => {
            //cryopod.develop();
        }
        Some(Commands::Develop(develop_args)) => {
            //cryopod.develop();
        }
        None => {
            println!("No subcommand specified. See `--help` for command information.");
        }
    };
    Ok(())
}
