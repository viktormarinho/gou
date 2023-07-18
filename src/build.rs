use std::process::Command;

use pretty_log::{log, PrettyError};

use crate::config::{BuildMode, CONFIG};

pub fn build_if_configured() {
    if let Some(cfg) = &CONFIG.build {
        match &cfg.mode {
            BuildMode::Local => {
                let mut args = cfg.cmd.split_whitespace();
                let mut cmd = Command::new(args.nth(0).expect_p("[gou] Build command is empty"));
    
                for arg in args {
                    cmd.arg(arg);
                }
    
                log::info(&format!("[gou] Running build command: {}", cfg.cmd));
                cmd.spawn()
                    .expect_p("[gou] Failed to run build command")
                    .wait()
                    .expect_p("[gou] Failed to wait for build command");
            },
            BuildMode::Remote => {
                log::warn("[gou] Remote build selected - not implemented yet. Skipping build step");
            }
        }
        return;
    }

    log::warn("[gou] No build configured, skipping build step");
}
