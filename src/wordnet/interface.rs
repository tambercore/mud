use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::wordnet::wordnode::Wordnode;

// Define a static singleton for word meanings
static WORD_MEANINGS: Lazy<Mutex<HashMap<String, Vec<Wordnode>>>> = Lazy::new(|| Mutex::new(HashMap::new()));


// Initialize the word meanings from the JSON file
pub fn init_wordnet() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("data/wordnet.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Value = serde_json::from_str(&contents)?;
    let synsets = data.get("synset").and_then(|s| s.as_object()).ok_or("Invalid JSON structure")?;

    let mut word_meanings = HashMap::new();
    let empty_array: Vec<Value> = Vec::new();

    // Iterate over the synsets to extract meanings, POS, IDs, and synonyms
    for (id, synset) in synsets {
        let words = synset.get("word").and_then(|w| w.as_array()).unwrap_or(&empty_array);
        let gloss = synset.get("gloss").and_then(|g| g.as_str()).unwrap_or("");
        let pos = synset.get("pos").and_then(|p| p.as_str()).unwrap_or("");

        // Split the gloss by semicolon and take the part before the first semicolon
        let meaning = gloss.split(';')
            .next()            // Take the first part before the semicolon (preprocess the definition)
            .unwrap_or(gloss)  // If there's no semicolon, use the whole gloss
            .trim()            // Trim any surrounding whitespace
            .to_string();

        // Collect all the synonyms (other words in the same synset)
        let synonyms: Vec<String> = words.iter()
            .filter_map(|w| w.as_str())
            .map(|w| w.to_string())
            .collect();

        // Create Wordnodes for each word in the synset
        for word in words.iter().filter_map(|w| w.as_str()) {
            // Remove the current word from its synonyms
            let filtered_synonyms: Vec<String> = synonyms
                .iter()
                .filter(|&synonym| synonym != word)
                .cloned()
                .collect();

            // Create the Wordnode
            let word_definition = Wordnode {
                meaning: meaning.clone(),
                pos: pos.to_string(),
                id: id.to_string(),
                synonyms: filtered_synonyms, // Add the filtered synonyms here
            };

            // Insert the Wordnode into the vector for each word
            word_meanings
                .entry(word.to_string())
                .or_insert_with(Vec::new)
                .push(word_definition);
        }
    }

    let mut meanings = WORD_MEANINGS.lock().unwrap();
    *meanings = word_meanings;

    Ok(())
}



// Access the meanings from the singleton
pub fn get_meanings(word: &str) -> Option<Vec<Wordnode>> {
    let meanings = WORD_MEANINGS.lock().unwrap();
    meanings.get(word).cloned()
}
