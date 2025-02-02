use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use once_cell::sync::Lazy;

pub static UNIVERSAL_QUANTIFIERS: Lazy<Vec<String>> = Lazy::new(|| {
    read_quantifiers().unwrap()
});

pub fn read_quantifiers() -> io::Result<Vec<String>> {
    let path = Path::new("data/every_quantifiers.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}