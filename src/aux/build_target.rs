use colored::Colorize;
use std::process::Command;

use crate::aux::write_log::write_log;

pub enum Builder {
    Cargo,
    Cross,
}

pub fn build_target(builder: Builder, target: &str, path: &str) {
    let mut command = match builder {
        Builder::Cargo => Command::new("cargo"),
        Builder::Cross => Command::new("cargo-cross"),
    };

    let status = command
        .args(["build", "--release", "--target", target])
        .status();

    match status {
        Ok(s) if s.success() => {
            let msg = format!("OK: {}", target);
            println!("{}", msg.green());
            let _ = write_log(&msg, path);
        }

        Ok(_) => {
            let msg = format!("FAIL: {}", target);
            eprintln!("{}", msg.red());
            let _ = write_log(&msg, path);
        }

        Err(e) => {
            let msg = format!("ERROR: {} ({})", target, e);
            eprintln!("{}", msg.red());
            let _ = write_log(&msg, path);
        }
    }
}
