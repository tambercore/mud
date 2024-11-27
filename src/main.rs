mod ccg;
mod brill;
mod montague;
mod wordnet;
mod lingo;

use std::io::{self, Write};
use crate::lingo::past_participle::get_past_participle;

fn main() {
    // Example usage of the get_past_participle function
    let verbs = vec![
        "eat", "play", "write", "try", "see", "dance", "lick",
        "begin", "take", "do", "stop", "hop", "run", "cry",
        "fly", "jump", "lie", "study", "apologize", "fax", "mix",
        "tickle", "pickle", "push", "kick"
    ];

    for verb in verbs {
        println!("{} -> {}", verb, get_past_participle(verb.to_string()));
    }
}