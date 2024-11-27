mod ccg;
mod brill;
mod montague;
mod wordnet;

use std::io::{self, Write};

fn main() {

    use crate::wordnet::interface::*;

    // Initialize WordNet
    if let Err(e) = init_wordnet() {
        eprintln!("Error initializing WordNet: {}", e);
        return;
    }

    // Retrieve meanings for "dog"
    if let Some(meanings) = get_meanings("good") {
        for meaning in meanings {
            println!("{}", meaning);
        }
    } else {
        println!("No meanings found for 'dog'.");
    }
}
