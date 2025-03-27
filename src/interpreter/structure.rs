use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::hash::Hash;

// Define a global LazyLock for our List
static INTERPRETATIONS: OnceLock<RwLock<Vec<Interpretation>>> = OnceLock::new();

fn get_interpretations() -> &'static RwLock<Vec<Interpretation>> {
    INTERPRETATIONS.get_or_init(|| RwLock::new(Vec::new()))
}

// Insert an Interpretation into the global Vec
pub fn insert_interpretation(interpretation: Interpretation) {
    let mut vec = get_interpretations().write().unwrap();
    vec.push(interpretation);
}

// Retrieve all Interpretations from the global Vec
fn get_all() -> Vec<Interpretation> {
    let vec = get_interpretations().read().unwrap();
    vec.clone() // Clone the entire Vec, or you could return a reference if needed
}

pub fn print_interpretations() {
    /* Access the global Vec of Interpretations */
    let interpretations = get_interpretations().read().unwrap();

    /* Iterate through each Interpretation and print in the desired format */
    for (index, interpretation) in interpretations.iter().enumerate() {
        println!("{}. {} ({})", index + 1, interpretation.statement, interpretation.source);
    }
}

/// A type to denote Natural Language Interpretations.
#[derive(Debug, Clone)]
pub struct Interpretation {
    pub statement: String,
    pub source: String,
}
