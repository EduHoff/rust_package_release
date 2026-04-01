use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn write_log(msg: &str, path: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new().create(true).append(true).open(path)?;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", msg)?;

    writer.flush()?;
    Ok(())
}
