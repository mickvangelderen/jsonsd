use std::path::Path;

use serde::de::DeserializeOwned;

pub fn read_json_file<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
) -> Result<T, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
