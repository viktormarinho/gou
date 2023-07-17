use std::process::Command;

use crate::config::CONFIG;

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

        Command::new("git")
            .arg("stash")
            .spawn()
            .expect("Failed to stash changes")
            .wait()
            .expect("Failed to wait for stash changes");


        Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(&branch_name)
            .spawn()
            .expect("Failed to create feat branch")
            .wait()
            .expect("Failed to wait for feat branch");

        Command::new("git")
            .arg("stash")
            .arg("pop")
            .spawn()
            .expect("Failed to pop stash")
            .wait()
            .expect("Failed to wait for pop stash");

        Command::new("git")
            .arg("add")
            .arg(".")
            .spawn()
            .expect("Failed to add changes")
            .wait()
            .expect("Failed to wait for add changes");

        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(format!("feat: {}", message))
            .spawn()
            .expect("Failed to commit changes")
            .wait()
            .expect("Failed to wait for commit changes");

        Command::new("git")
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg(&branch_name)
            .spawn()
            .expect("Failed to push changes")
            .wait()
            .expect("Failed to wait for push changes");

        Command::new("gh")
            .arg("pr")
            .arg("create")
            .arg("--title")
            .arg(format!("feat: {}", message))
            .arg("--body")
            .arg("")
            .arg("--base")
            .arg(target_branch)
            .spawn()
            .expect("Failed to create PR")
            .wait()
            .expect("Failed to wait for create PR");

        println!("Feat");
    }
}
