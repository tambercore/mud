use crate::ast::program::Program;

use std::fs;
use std::io::{self, Read};

pub fn parse_agda(filepath: String) -> Program {
    let mut program = Program { filepath: filepath.clone(), declarations: vec![] };

    // Read the file from the given path as a string
    let agda = fs::read_to_string(&filepath).expect("Something went wrong reading the file");

    println!("Agda: {}", agda);

    program
}
