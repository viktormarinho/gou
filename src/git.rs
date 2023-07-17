use std::process::Command;

pub struct Git;

impl Git {
    pub fn stash() {
        Command::new("git")
            .arg("stash")
            .spawn()
            .expect("Failed to stash changes")
            .wait()
            .expect("Failed to wait for stash changes");
    }

    pub fn stash_pop() {
        Command::new("git")
            .arg("stash")
            .arg("pop")
            .spawn()
            .expect("Failed to pop stash")
            .wait()
            .expect("Failed to wait for pop stash");
    }

    pub fn checkout_create(branch_name: &str) {
        Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(branch_name)
            .spawn()
            .expect("Failed to create feat branch")
            .wait()
            .expect("Failed to wait for feat branch");
    }

    pub fn checkout(branch_name: &str) {
        Command::new("git")
            .arg("checkout")
            .arg(branch_name)
            .spawn()
            .expect("Failed to checkout feat branch")
            .wait()
            .expect("Failed to wait for checkout feat branch");
    }

    pub fn add() {
        Command::new("git")
            .arg("add")
            .arg(".")
            .spawn()
            .expect("Failed to add changes")
            .wait()
            .expect("Failed to wait for add changes");
    }

    pub fn commit(message: &str) {
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .spawn()
            .expect("Failed to commit changes")
            .wait()
            .expect("Failed to wait for commit changes");
    }

    pub fn push_set_upstream(branch_name: &str) {
        Command::new("git")
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg(&branch_name)
            .spawn()
            .expect("Failed to push changes")
            .wait()
            .expect("Failed to wait for push changes");
    }

    pub fn pull() {
        Command::new("git")
            .arg("pull")
            .spawn()
            .expect("Failed to pull changes")
            .wait()
            .expect("Failed to wait for pull changes");
    }
}