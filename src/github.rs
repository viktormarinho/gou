use std::process::Command;

use pretty_log::PrettyError;

pub struct Github;

impl Github {
    pub fn create_pr(message: &str, target_branch: &str) {
        Command::new("gh")
            .arg("pr")
            .arg("create")
            .arg("--title")
            .arg(message)
            .arg("--body")
            .arg("")
            .arg("--base")
            .arg(target_branch)
            .spawn()
            .expect_p("Failed to create PR")
            .wait()
            .expect_p("Failed to wait for create PR");
    }
}