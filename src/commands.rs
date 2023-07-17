use clap::Subcommand;

use crate::fix::Fix;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Fix(Fix),
    Refactor,
    Feat,
    Chore,
    Test,
    Docs,
    Style,
    Ci,
}