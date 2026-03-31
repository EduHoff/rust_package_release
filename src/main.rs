use std::collections::HashMap;

use clearscreen::clear;
use colored::Colorize;
use rust_package_release::aux::{
    build_target::{Builder, build_target},
    check_dependency::{DependencyError, check_dependency},
    dependencies::DEPENDENCIES,
    targets::{MAC_TARGETS, TARGETS},
};

fn main() {
    clear().expect("Error: clear failed");

    let mut status_dependencies_map: HashMap<&str, bool> = HashMap::new();
    for dep in DEPENDENCIES {
        let result = check_dependency(dep.name, dep.args);

        let status = result.is_ok();
        status_dependencies_map.insert(dep.label, status);

        match result {
            Ok(()) => {
                let msg = "is installed and running".green();
                println!("{} {}", dep.label.green(), msg);
            }
            Err(DependencyError::NotFound) => {
                let msg = "is not installed".red();
                eprintln!("{} {}", dep.label.red(), msg);
            }
            Err(DependencyError::ExecutionError) => {
                let msg = "installed, but it's not working correctly".red();
                eprintln!("{} {}", dep.label.red(), msg);
            }
        }
    }

    let rustc_ok = status_dependencies_map
        .get("rustc")
        .copied()
        .unwrap_or(false);
    let cargo_ok = status_dependencies_map
        .get("cargo")
        .copied()
        .unwrap_or(false);

    if !rustc_ok || !cargo_ok {
        let msg = "You must have rustc and cargo installed!".red();
        eprintln!("{}", msg);

        #[cfg(windows)]
        let _ = std::process::Command::new("cmd")
            .arg("/c")
            .arg("pause")
            .status();
        return;
    }

    let docker_ok = status_dependencies_map
        .get("docker")
        .copied()
        .unwrap_or(false);

    let cargo_cross_ok = status_dependencies_map
        .get("cargo-cross")
        .copied()
        .unwrap_or(false);

    if docker_ok && cargo_cross_ok {
        for tag in TARGETS {
            build_target(Builder::Cross, tag);
        }
    }

    let x86_64_apple_ok = status_dependencies_map
        .get("macOS target (x86_64)")
        .copied()
        .unwrap_or(false);

    let aarch64_apple_ok = status_dependencies_map
        .get("macOS target (aarch64)")
        .copied()
        .unwrap_or(false);

    if x86_64_apple_ok && aarch64_apple_ok {
        for tag in MAC_TARGETS {
            build_target(Builder::Cargo, tag);
        }
    }

    #[cfg(windows)]
    let _ = std::process::Command::new("cmd")
        .arg("/c")
        .arg("pause")
        .status();
}
