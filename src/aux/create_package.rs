use colored::Colorize;
use std::{
    fs::File,
    io::{self, Error, ErrorKind},
    path::Path,
};

use crate::aux::{targets::BuildTarget, write_log::write_log};

pub fn create_package(
    root_path: &Path,
    target: &BuildTarget,
    project_name: &str,
    license_file: Option<&str>,
    file_log_name: &str,
) -> io::Result<()> {
    let out_dir = root_path.join("release_packages");
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir)?;
    }

    let mut bin_filename = project_name.to_string();
    if target.name.contains("windows") {
        bin_filename.push_str(".exe");
    }

    let src_path = root_path
        .join("target")
        .join(target.name)
        .join("release")
        .join(&bin_filename);

    if !src_path.exists() {
        let msg = format!("Binary not found in {:?}", src_path);
        eprintln!("{}", msg.red());
        let _ = write_log(&msg, root_path, file_log_name);
        return Err(Error::new(ErrorKind::NotFound, msg));
    }

    let is_zip = target.name.contains("windows") || target.name.contains("apple");
    let extension = if is_zip { "zip" } else { "tar.gz" };
    let archive_name = format!("{}-{}.{}", project_name, target.label, extension);

    let dest_path = out_dir.join(archive_name); // PRECISO VERIFICAR ISSO AINDA
    let file = File::create(&dest_path)?;

    if is_zip {
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        zip.start_file(&bin_filename, options)?;
        let bin_data = std::fs::read(&src_path)?;
        std::io::Write::write_all(&mut zip, &bin_data)?;

        if let Some(lic_name) = license_file {
            let lic_path = root_path.join(lic_name);
            if lic_path.exists() {
                zip.start_file(lic_name, options)?;
                let lic_data = std::fs::read(lic_path)?;
                std::io::Write::write_all(&mut zip, &lic_data)?;
            }
        }
        zip.finish()?;
    } else {
        let enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut tar = tar::Builder::new(enc);

        tar.append_path_with_name(&src_path, &bin_filename)?;

        if let Some(lic_name) = license_file {
            let lic_path = root_path.join(lic_name);
            if lic_path.exists() {
                tar.append_path(lic_name)?;
            }
        }
        tar.finish()?;
    }

    let success_msg = format!("Created: {}", dest_path.display());
    println!("{}", success_msg.green());
    let _ = write_log(&success_msg, root_path, file_log_name);

    Ok(())
}
