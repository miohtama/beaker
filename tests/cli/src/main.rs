use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use protostar_cw::CW;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    config: Option<PathBuf>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manipulating and interacting with CosmWasm contract
    Cw {
        #[clap(subcommand)]
        cmd: CW,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Cw { cmd } => CW::execute(&cmd),
    }
}