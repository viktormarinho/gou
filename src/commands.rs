use clap::Subcommand;

use crate::{fix::Fix, refactor::Refactor, feat::Feat};

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Fix(Fix),
    Refactor(Refactor),
    Feat(Feat),
    //Chore,
    //Test,
    //Docs,
    //Style,
    //Ci,
    //Build,
    //Revert,
    //Perf,
    //Config,
    //Init,
    //Release,
    //Bump,
    // blablabla
}