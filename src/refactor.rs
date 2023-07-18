use crate::{config::CONFIG, build, git::Git, github::Github};



#[derive(Debug, Clone, clap::Parser)]
pub struct Refactor {
    #[clap(value_delimiter = ' ', num_args = 1..)]
    message: Vec<String>,
}

impl Refactor {
    pub fn run(self) {
        let message = self.message.join(" ");
        let branch_name = format!("refactor/{}", message.to_lowercase().replace(" ", "-"));
        let target_branch = CONFIG.main_branch.clone();

        build::build_if_configured();

        Git::add();
        Git::stash();
        Git::checkout_create(&branch_name);
        Git::stash_pop();
        Git::add();
        Git::commit(&format!("refactor: {}", message));
        Git::push_set_upstream(&branch_name);

        Github::create_pr(&format!("refactor: {}", message), &target_branch);

        Git::checkout(&target_branch);
    }
}