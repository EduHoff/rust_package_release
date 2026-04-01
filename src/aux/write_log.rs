use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn write_log(msg: &str, root_path: &Path, file_log_name: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(root_path.join(file_log_name))?;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", msg)?;

    writer.flush()?;
    Ok(())
}
