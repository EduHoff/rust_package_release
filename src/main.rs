use std::{collections::HashMap, env, path::PathBuf};

use chrono::Local;
use clearscreen::clear;
use colored::Colorize;
use rust_package_release::aux::{
    build_target::{Builder, build_target},
    check_dependency::{DependencyError, check_dependency},
    create_package::create_package,
    dependencies::DEPENDENCIES,
    project_data::{get_license_file, get_project_name},
    targets::{MAC_TARGETS, TARGETS},
    write_log::write_log,
};

fn main() {
    clear().expect("Error: clear failed");

    let root_path: PathBuf =
        env::current_dir().expect("Error: Could not determine current directory");

    let file_log_name = Local::now()
        .format("log_%Y-%m-%dT%Hh%Mm%Ss%:z.txt")
        .to_string();

    let has_cargo = root_path.join("Cargo.toml").exists();
    let has_src = root_path.join("src/main.rs").exists() || root_path.join("src/lib.rs").exists();

    if !has_cargo || !has_src {
        let msg = "Error: Not a valid Rust project (missing Cargo.toml or src files).";
        eprintln!("{}", msg.red());
        let _ = write_log(msg, &root_path, &file_log_name);

        #[cfg(windows)]
        let _ = std::process::Command::new("cmd")
            .arg("/c")
            .arg("pause")
            .status();
        return;
    }

    let project_name = get_project_name(&root_path);
    let license_file = get_license_file(&root_path);

    let mut status_dependencies_map: HashMap<&str, bool> = HashMap::new();
    for dep in DEPENDENCIES {
        let result = check_dependency(dep.name, dep.args);

        let status = result.is_ok();
        status_dependencies_map.insert(dep.label, status);

        match result {
            Ok(()) => {
                let msg = format!("{} {}", dep.label, "is installed and running");
                eprintln!("{}", msg.green());
                let _ = write_log(&msg, &root_path, &file_log_name);
            }
            Err(DependencyError::NotFound) => {
                let msg = format!("{} {}", dep.label, "is not installed");
                eprintln!("{}", msg.red());
                let _ = write_log(&msg, &root_path, &file_log_name);
            }
            Err(DependencyError::ExecutionError) => {
                let msg = format!(
                    "{} {}",
                    dep.label, "installed, but it's not working correctly"
                );
                eprintln!("{}", msg.red());
                let _ = write_log(&msg, &root_path, &file_log_name);
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
        let _ = write_log(&msg, &root_path, &file_log_name);

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
            build_target(Builder::Cross, tag.name, &root_path, &file_log_name);
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

    if x86_64_apple_ok || aarch64_apple_ok {
        for tag in MAC_TARGETS {
            build_target(Builder::Cargo, tag.name, &root_path, &file_log_name);
        }
    }

    if !(docker_ok && cargo_cross_ok || x86_64_apple_ok && aarch64_apple_ok) {
        let msg = "You must at least have docker and cargo-cross or target some apple".red();
        eprintln!("{}", msg);
        let _ = write_log(&msg, &root_path, &file_log_name);

        #[cfg(windows)]
        let _ = std::process::Command::new("cmd")
            .arg("/c")
            .arg("pause")
            .status();

        return;
    }

    if docker_ok && cargo_cross_ok {
        for tag in TARGETS {
            if let Err(e) = create_package(
                &root_path,
                tag,
                &project_name,
                license_file.as_deref(),
                &file_log_name,
            ) {
                let msg = format!("Error creating package: {}", e);
                eprintln!("{}", msg.red());
                let _ = write_log(&msg, &root_path, &file_log_name);
            }
        }
    }

    if x86_64_apple_ok || aarch64_apple_ok {
        for tag in MAC_TARGETS {
            if let Err(e) = create_package(
                &root_path,
                tag,
                &project_name,
                license_file.as_deref(),
                &file_log_name,
            ) {
                let msg = format!("Error creating package: {}", e);
                eprintln!("{}", msg.red());
                let _ = write_log(&msg, &root_path, &file_log_name);
            }
        }
    }

    #[cfg(windows)]
    let _ = std::process::Command::new("cmd")
        .arg("/c")
        .arg("pause")
        .status();
}
