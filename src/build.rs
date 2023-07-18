use std::process::Command;

use pretty_log::{log, PrettyError};

use crate::config::CONFIG;



pub fn build_if_configured() {
    CONFIG.build_command.as_ref().map(|cmd| {
        log::info(&format!("[gou] Running build command: {}", cmd));

        let mut args = cmd.split_whitespace();
        let mut cmd = Command::new(args.nth(0).unwrap());

        for arg in args {
            cmd.arg(arg);
        }

        cmd.spawn()
            .expect_p("[gou] Failed to run build command")
            .wait()
            .expect_p("[gou] Failed to wait for build command");
    });
}