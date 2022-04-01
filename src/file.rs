use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use crate::errors::RError;
use std::error::Error;
use std::io::Write;
use std::fs::OpenOptions;
use chrono;

pub fn save_file(data: Vec<String>, host: String) -> Result<(), RError> {
    let stamp = chrono::offset::Local::now().date();
    let file_name = format!("./tmp/{}-{}.txt", host, stamp);
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(file_name)?;

    let writer = BufWriter::new(file);
    write_to_file(data, writer)?;
    Ok(())
}

fn write_to_file<T: Write>(data: Vec<String>, mut writer: T) -> Result<(), RError> {
    for d in data.iter() {
        println!("writing... {}", d);
        let line = format!("{}\n", d);
        writer.write(line.as_bytes())?;
    }
    let count = format!("\nindexed count: {}", data.len());
    writer.write(count.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_to_file() {
        let mut indexed = vec![String::from("result1 - OK"), String::from("result2 - OK")];
        let mut buf = Vec::new();

        assert!(write_to_file(indexed, &mut buf).is_ok());
        assert_eq!(
            "result1 - OK\nresult2 - OK\n\nindexed count: 2",
            String::from_utf8(buf).unwrap()
        );
    }
}
