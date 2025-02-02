use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use once_cell::sync::Lazy;

pub static UNIVERSAL_QUANTIFIERS: Lazy<Vec<String>> = Lazy::new(|| {
    read_universal_quantifiers().unwrap()
});
fn read_universal_quantifiers() -> io::Result<Vec<String>> {
    let path = Path::new("data/universal_quantifiers.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}
