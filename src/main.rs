mod ccg;
mod brill;
mod montague;
mod wordnet;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    use wordnet::interface::get_meanings;

    let b = get_meanings("murder");
    println!("{:?}", b);

    return Ok(());
}