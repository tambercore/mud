use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use once_cell::sync::Lazy;

/// Static variable to hold the list of universal quantifiers read from a file.
pub static UNIVERSAL_QUANTIFIERS: Lazy<Vec<String>> = Lazy::new(|| {
    read_universal_quantifiers().unwrap()
});

/// Static variable to hold the list of existential quantifiers read from a file.
pub static EXISTENTIAL_QUANTIFIERS: Lazy<Vec<String>> = Lazy::new(|| {
    read_existential_quantifiers().unwrap()
});

/// Function to read universal quantifiers from a file and return them as a vector of strings.
fn read_universal_quantifiers() -> io::Result<Vec<String>> {
    let path = Path::new("data/universal_quantifiers.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    /* Read lines from the file and collect them into a vector of strings */
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

/// Function to read existential quantifiers from a file and return them as a vector of strings.
fn read_existential_quantifiers() -> io::Result<Vec<String>> {
    let path = Path::new("data/existential_quantifiers.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    /* Read lines from the file and collect them into a vector of strings */
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}
