mod ccg;
mod brill;
mod montague;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let b = get_meanings("murder");
    println!("{:?}", b);
    return;
}