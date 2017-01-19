use std::fs::File;
use std::io::Read;
use std::error::Error;

pub fn read_file(path: &str) -> Result<String, Box<Error>> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}
