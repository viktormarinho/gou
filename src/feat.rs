use crate::{config::CONFIG, git::Git, github::Github, build};

#[derive(Debug, Clone, clap::Parser)]
pub struct Feat {
    #[clap(value_delimiter = ' ', num_args = 1..)]
    message: Vec<String>,
}

impl Feat {
    pub fn run(self) {
        let message = self.message.join(" ");
        let branch_name = format!("feat/{}", message.to_lowercase().replace(" ", "-"));
        let target_branch = CONFIG.main_branch.clone();

        build::build_if_configured();

        Git::transaction()
            .add()
            .stash()
            .checkout_create(&branch_name)
            .stash_pop()
            .add()
            .commit(&format!("feat: {}", message))
            .push_set_upstream(&branch_name)
            .execute();

        // Git::add();
        // Git::stash();
        // Git::checkout_create(&branch_name);
        // Git::stash_pop();
        // Git::add();
        // Git::commit(&format!("feat: {}", message));
        // Git::push_set_upstream(&branch_name);

        Github::create_pr(&format!("feat: {}", message), &target_branch);

        Git::transaction()
            .checkout(&target_branch);
    }
}
