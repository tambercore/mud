use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::lingo::consonants::*;


// Static variable to hold the irregular verb mappings
static IRREGULAR_VERBS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let file = File::open("data/irregular_verbs.txt").expect("File not found");
    let reader = io::BufReader::new(file);

    // Read each line from the file and process it
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            let (normal, _, past_participle) = (parts[0], parts[1], parts[2]);
            map.insert(normal.to_string(), past_participle.to_string());
        }
    }

    Mutex::new(map) // Wrapping HashMap in a Mutex for thread-safety
});



// Function to get the past participle of a verb
pub
fn get_past_participle(verb: String) -> String {
    // Step 1: Check if the verb is irregular and return the past participle from the static map
    let map = IRREGULAR_VERBS.lock().expect("Failed to lock map");
    if let Some(past_participle) = map.get(verb.as_str()) {
        return past_participle.clone();
    }

    // Step 2: Rule-based system for regular verbs

    // Rule 1: Verbs ending in "e" simply add "d"
    if verb.ends_with('e') {
        return format!("{}d", verb);
    }

    // Rule 2: Verbs ending in consonant + "y" (excluding some exceptions) change "y" to "ied"
    if verb.ends_with("y") && !verb.ends_with("ay") && !verb.ends_with("ey") && !verb.ends_with("iy") && !verb.ends_with("oy") && !verb.ends_with("uy") {
        return format!("{}ied", &verb[..verb.len() - 1]);
    }

    // Rule 3: Verbs with a CVC pattern (Consonant-Vowel-Consonant) double the final consonant
    if verb.len() >= 3 {
        let chars: Vec<char> = verb[verb.len() - 3..].chars().collect();
        if chars.len() == 3 {
            let (first, second, third) = (chars[0], chars[1], chars[2]);

            // Check if the pattern is CVC and the final consonant is not "p" or "t"
            if first.is_consonant() && second.is_vowel() && third.is_consonant() && !matches!(third, 'p' | 't') {
                return format!("{}{}ed", &verb[..verb.len() - 1], third);
            }
        }
    }


    // Rule 4: Verbs ending in "c" add "ked"
    if verb.ends_with("c") {
        return format!("{}ked", verb);
    }

    // Rule 5: Verbs ending with specific consonants add "ped"
    if verb.ends_with("p") || verb.ends_with("t") || verb.ends_with("d") {
        return format!("{}ped", verb);
    }

    // Default rule: Add "ed" for other regular verbs
    format!("{}ed", verb)
}
