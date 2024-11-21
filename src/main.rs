mod ccg;
mod brill;
mod lambda;
mod wordnet;

use std::io::{Write};

fn main() {
    use wordnet::interface::*;
    init_wordnet();
    let a = get_meanings("wank");
    println!("{:?}", a);

    let b = get_meanings("murder");
    println!("{:?}", b);
    return;
}