use colored::Colorize;
use std::process::Command;

pub enum Builder {
    Cargo,
    Cross,
}

pub fn build_target(builder: Builder, target: &str) {
    let mut command = match builder {
        Builder::Cargo => Command::new("cargo"),
        Builder::Cross => Command::new("cargo-cross"),
    };

    let status = command
        .args(["build", "--release", "--target", target])
        .status();

    match status {
        Ok(s) if s.success() => println!("{}", format!("OK: {}", target).green()),
        Ok(_) => eprintln!("{}", format!("FAIL: {}", target).red()),
        Err(e) => eprintln!("{}", format!("ERROR: {} ({})", target, e).red()),
    }
}
