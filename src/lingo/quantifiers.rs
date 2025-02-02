use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_quantifiers() -> io::Result<Vec<String>> {
    let path = Path::new("data/every_quantifiers.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}