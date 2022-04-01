use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use crate::errors::RError;
use std::io::Write;
use chrono;

pub fn save_file(data: Vec<String>, path: String, domain: &str) -> Result<(), RError> {
    let stamp = chrono::offset::Local::now().date();
    let new_file = File::create(Path::new(format!("{}/{}-{}.txt", path, domain, stamp).as_str()))?;
    let writer = BufWriter::new(new_file);
    write_to_file(data, writer)?;
    Ok(())
}

fn write_to_file<T: Write>(
    data: Vec<String>,
    mut writer: T,
) -> Result<(), RError> {
    for d in data.iter() {
        let line = format!("{}\n", d);
        writer.write_all(line.as_bytes())?;
    }
    let totals = format!("\nindexed count: {}", data.len());
    writer.write_all(totals.as_bytes())?;
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
