use std::process::{Command, Stdio};

pub enum DependencyError {
    NotFound,
    ExecutionError,
}

pub fn check_dependency(cmd: &str, args: &[&str]) -> Result<(), DependencyError> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match output {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(DependencyError::ExecutionError),
        Err(_) => Err(DependencyError::NotFound),
    }
}
