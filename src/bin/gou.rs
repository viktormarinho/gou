use std::time::{SystemTime, UNIX_EPOCH};

use gou_git::commands::Commands;
use clap::Parser;
use pretty_log::log;


#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

fn main() {
    let args = Args::parse();
    log::init("[gou]");

    match &args.command {
        Commands::Fix(args) => args.to_owned().run(),
        Commands::Refactor(args) => args.to_owned().run(),
        Commands::Feat(args) => args.to_owned().run(),
    }
}