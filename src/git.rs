use std::{io::Error, process::Command};

use pretty_log::{log, PrettyError};

use crate::config::CONFIG;

enum Operation {
    Stash,
    StashPop,
    CheckoutCreate(String),
    Checkout(String),
    Add,
    Commit(String),
    PushSetUpstream(String),
    Pull,
}

pub struct GitTransaction {
    operations: Vec<Operation>,
    done: Vec<Operation>,
    start: String
}

impl GitTransaction {
    pub fn new() -> Self {
        GitTransaction {
            operations: Vec::new(),
            done: Vec::new(),
            start: String::new()
        }
    }

    pub fn stash(mut self) -> Self {
        self.operations.push(Operation::Stash);
        self
    }

    pub fn stash_pop(mut self) -> Self {
        self.operations.push(Operation::StashPop);
        self
    }

    pub fn checkout_create(mut self, branch_name: &str) -> Self {
        self.operations
            .push(Operation::CheckoutCreate(branch_name.to_string()));
        self
    }

    pub fn checkout(mut self, branch_name: &str) -> Self {
        self.operations
            .push(Operation::Checkout(branch_name.to_string()));
        self
    }

    pub fn add(mut self) -> Self {
        self.operations.push(Operation::Add);
        self
    }

    pub fn commit(mut self, message: &str) -> Self {
        self.operations.push(Operation::Commit(message.to_string()));
        self
    }

    pub fn push_set_upstream(mut self, branch_name: &str) -> Self {
        self.operations
            .push(Operation::PushSetUpstream(branch_name.to_string()));
        self
    }

    pub fn pull(mut self) -> Self {
        self.operations.push(Operation::Pull);
        self
    }

    pub fn execute(mut self) {
        self.start = chrono::Local::now().time().to_string().split(".").nth(0).unwrap().to_string();
        log::info(&format!("Starting git transaction at {}.", &self.start));

        for operation in &self.operations {
            match operation {
                Operation::Stash => match Git::stash() {
                    Ok(_) => {
                        self.done.push(Operation::Stash);
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to stash changes: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
                Operation::StashPop => match Git::stash_pop() {
                    Ok(_) => {
                        self.done.push(Operation::StashPop);
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to stash pop changes: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
                Operation::CheckoutCreate(branch_name) => match Git::checkout_create(&branch_name) {
                    Ok(_) => {
                        self.done
                            .push(Operation::CheckoutCreate(branch_name.to_string()));
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to checkout create branch: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
                Operation::Checkout(branch_name) => match Git::checkout(&branch_name) {
                    Ok(_) => {
                        self.done
                            .push(Operation::Checkout(branch_name.to_string()));
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to checkout branch: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
                Operation::Add => match Git::add() {
                    Ok(_) => {
                        self.done.push(Operation::Add);
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to add changes: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
                Operation::Commit(message) => match Git::commit(&message) {
                    Ok(_) => {
                        self.done.push(Operation::Commit(message.to_string()));
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to commit changes: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
                Operation::PushSetUpstream(branch_name) => match Git::push_set_upstream(&branch_name)
                {
                    Ok(_) => {
                        self.done
                            .push(Operation::PushSetUpstream(branch_name.to_string()));
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to push changes: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
                Operation::Pull => match Git::pull() {
                    Ok(_) => {
                        self.done.push(Operation::Pull);
                    }
                    Err(error) => {
                        log::error(&format!(
                            "Failed to pull changes: {}\nRolling back git changes.",
                            error
                        ));
                        self.rollback();
                    }
                },
            }
        }
    }

    fn rollback(&self) {
        Git::reset_hard(
            &format!("HEAD@{{{}}}", self.start.as_str())
        ).expect_p("Error rolling back git changes.");
        return;
        for operation in self.done.iter().rev() {
            match operation {
                Operation::Stash => {
                    Git::stash_pop().expect_p("Error rolling back stash changes.");
                }
                Operation::StashPop => {
                    Git::stash().expect_p("Error rolling back stash changes.");
                }
                Operation::CheckoutCreate(_) => {
                    Git::checkout(&CONFIG.main_branch)
                        .expect_p("Error rolling back checkout create.");
                }
                Operation::Checkout(_) => {
                    Git::checkout(&CONFIG.main_branch).expect_p("Error rolling back checkout.");
                }
                Operation::Add => {
                    Git::reset().expect_p("Error rolling back git add (running git reset).");
                }
                Operation::Commit(_) => {
                    Git::reset_soft("HEAD~1").expect_p(
                        "Error rolling back git commit (running git reset --soft HEAD~1).",
                    );
                }
                Operation::PushSetUpstream(branch_name) => {
                    Git::delete_remote_branch(&branch_name).expect_p(&format!(
                        "Error rolling back git push set upstream (running git push origin -d {}).",
                        &branch_name
                    ));
                }
                Operation::Pull => {
                }
            }
        }
        log::info("Rolled back git changes successfully.");
    }
}

pub struct Git;

type GitResult = Result<(), Error>;

impl Git {
    pub fn transaction() -> GitTransaction {
        GitTransaction::new()
    }

    pub fn stash() -> GitResult {
        Command::new("git").arg("stash").spawn()?.wait()?;

        Ok(())
    }

    pub fn stash_pop() -> GitResult {
        Command::new("git")
            .arg("stash")
            .arg("pop")
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn checkout_create(branch_name: &str) -> GitResult {
        let status = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(branch_name)
            .spawn()?
            .wait()?;
            
        if !status.success() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to checkout create branch: {}", status),
            ))
        }
        
        Ok(())
    }

    pub fn checkout(branch_name: &str) -> GitResult {
        Command::new("git")
            .arg("checkout")
            .arg(branch_name)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn add() -> GitResult {
        Command::new("git").arg("add").arg(".").spawn()?.wait()?;

        Ok(())
    }

    pub fn commit(message: &str) -> GitResult {
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn push_set_upstream(branch_name: &str) -> GitResult {
        Command::new("git")
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg(&branch_name)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn pull() -> GitResult {
        Command::new("git").arg("pull").spawn()?.wait()?;

        Ok(())
    }

    pub fn reset() -> GitResult {
        Command::new("git").arg("reset").spawn()?.wait()?;

        Ok(())
    }

    pub fn reset_soft(target: &str) -> GitResult {
        Command::new("git")
            .arg("reset")
            .arg("--soft")
            .arg(target)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn delete_remote_branch(target: &str) -> GitResult {
        Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("-d")
            .arg(target)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn reset_hard(target: &str) -> GitResult {
        Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg(target)
            .spawn()?
            .wait()?;

        Ok(())
    }
}
