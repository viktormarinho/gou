use std::process::Command;

use crate::{config::CONFIG, git::Git, github::Github};

#[derive(Debug, Clone, clap::Parser)]
pub struct Feat {
    #[clap(value_delimiter = ' ', num_args = 1..)]
    message: Vec<String>,
}

impl Feat {
    pub fn run(self) {
        let message = self.message.join(" ");
        let branch_name = format!("feat/{}", message.to_lowercase().replace(" ", "-"));
        let target_branch = CONFIG.main_branch.clone().unwrap_or(String::from("main"));

        CONFIG.build_command.as_ref().map(|cmd| {
            println!("Running build command: {}", cmd);

            let mut args = cmd.split_whitespace();
            let mut cmd = Command::new(args.nth(0).unwrap());

            for arg in args {
                cmd.arg(arg);
            }

            cmd.spawn()
                .expect("Failed to run build command")
                .wait()
                .expect("Failed to wait for build command");
        });

        Git::add();
        Git::stash();
        Git::checkout_create(&branch_name);
        Git::stash_pop();
        Git::add();
        Git::commit(&format!("feat: {}", message));
        Git::push_set_upstream(&branch_name);

        Github::create_pr(&format!("feat: {}", message), &target_branch);

        Git::checkout(&target_branch);
    }
}
