mod ccg;
mod brill;

use std::io::{self, Write};
use std::process::Command;
use ccg::language_parser::english_to_ccg;

fn main() -> io::Result<()> {
    let ccg = english_to_ccg().unwrap();
    println!("{:?}", ccg);

    Ok(())
}
