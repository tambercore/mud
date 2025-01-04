use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::lingo::verb_conjugation::apply_conjugation_rules;

/// Static variable to hold the irregular verb mappings, parsed from `data/irregular_verbs.txt`.
static IRREGULAR_VERBS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let file = File::open("data/irregular_verbs.txt").expect("File not found");
    let reader = io::BufReader::new(file);

    // Parse the file
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



/// Function to get the past participle of a verb
pub fn get_past_participle(verb: String) -> String {

    // Checking if the word is irregular, as the rules don't apply to irregular verbs.
    let map = IRREGULAR_VERBS.lock().expect("Failed to lock map");
    if let Some(past_participle) = map.get(verb.as_str()) {
        return past_participle.clone();
    }

    // The verb is regular, so we can apply generic conjugation rules.
    return apply_conjugation_rules(verb);
}



#[test]
fn test_get_past_participle() {
    // Create a vector of test cases with verbs and their expected past participles
    let test_cases = vec![
        ("eat", "eaten"),
        ("play", "played"),
        ("write", "written"),
        ("try", "tried"),
        ("see", "seen"),
        ("dance", "danced"),
        ("lick", "licked"),
        ("begin", "begun"),
        ("take", "taken"),
        ("do", "done"),
        ("stop", "stopped"),
        ("hop", "hopped"),
        ("run", "run"),
        ("cry", "cried"),
        ("fly", "flown"),
        ("jump", "jumped"),
        ("lie", "lain"),
        ("study", "studied"),
        ("apologize", "apologized"),
        ("fax", "faxed"),
        ("mix", "mixed"),
        ("tickle", "tickled"),
        ("pickle", "pickled"),
        ("push", "pushed"),
        ("kick", "kicked"),
    ];

    // Iterate through the test cases and assert the expected result
    for (verb, expected_participle) in test_cases {
        assert_eq!(get_past_participle(verb.to_string()), expected_participle);
    }
}
