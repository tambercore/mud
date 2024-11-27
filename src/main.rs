mod ccg;
mod brill;
mod montague;
mod wordnet;
mod lingo;

use std::io::{self, Write};

fn main() {
    let verbs = vec!["eat", "play", "write", "try", "see", "dance"];

    for verb in verbs {
        println!("{} -> {}", verb, crate::lingo::past_participle::get_past_participle(verb.parse().unwrap()));
    }
}