use crate::{config::CONFIG, git::Git, github::Github};
use std::process::Command;


#[derive(Debug, Clone, clap::Parser)]
pub struct Fix {
    #[clap(value_delimiter = ' ', num_args = 1..)]
    message: Vec<String>,
}

impl Fix {
    pub fn run(self) {
        let message = self.message.join(" ");
        let branch_name = format!("fix/{}", message.to_lowercase().replace(" ", "-"));
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
        Git::commit(&format!("fix: {}", message));
        Git::push_set_upstream(&branch_name);

        Github::create_pr(&format!("fix: {}", message), &target_branch);

        Git::checkout(&target_branch);
    }
}