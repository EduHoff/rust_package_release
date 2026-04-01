use std::{fs, path::Path};

use cargo_toml::Manifest;
use chrono::Local;

pub fn get_project_name(root_path: &Path) -> String {
    let generic_name = Local::now()
        .format("generic_name_%Y-%m-%dT%Hh%Mm%Ss%:z")
        .to_string();

    let content = match fs::read(root_path.join("Cargo.toml")) {
        Ok(c) => c,
        Err(_) => return generic_name,
    };

    let manifest = match Manifest::from_slice(&content) {
        Ok(m) => m,
        Err(_) => return generic_name,
    };

    manifest.package.map(|pkg| pkg.name).unwrap_or(generic_name)
}

pub fn get_license_file(root_path: &Path) -> Option<String> {
    let prefixes = ["license", "licence", "copying", "copyright", "unlicense"];

    let entries = fs::read_dir(root_path).ok()?;

    for entry in entries.flatten() {
        if let Some(file_name) = entry.file_name().to_str() {
            let lower_file_name = file_name.to_lowercase();

            if prefixes.iter().any(|&p| lower_file_name.starts_with(p)) {
                return Some(file_name.to_string());
            }
        }
    }
    None
}
