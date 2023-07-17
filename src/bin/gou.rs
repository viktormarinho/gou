use gou_git::commands::Commands;
use clap::Parser;


#[derive(Parser, Debug, Clone)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Fix(args) => args.to_owned().run(),
        Commands::Refactor(args) => args.to_owned().run(),
        Commands::Feat(args) => args.to_owned().run(),
    }
}