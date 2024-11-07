use std::collections::HashMap;
use std::fs;
use serde::{Deserialize};
use serde_json;
use std::io;

/// Struct used to help parse contractions in `load_contractions`.
#[derive(Deserialize, Debug)]
struct Contractions {
    #[serde(flatten)]
    contractions: HashMap<String, Vec<String>>,
}


/// Function to load `data/contractions.json` as a hashmap of contractions to their expansions.
fn load_contractions() -> Result<HashMap<String, Vec<String>>, io::Error> {
    let data = fs::read_to_string("data/contractions.json")?;
    let contractions: Contractions = serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(contractions.contractions)
}


/// Function to expand a contraction `input` according to the `contractions_map`.
fn expand_contraction(input: String, contractions_map: &HashMap<String, Vec<String>>) -> Option<Vec<String>> {
    contractions_map.get(&input).map(|expansion| expansion.clone())
}


/// Function to find contractions for a given `input`
pub fn find_contractions(input: String) -> Result<Vec<String>, String> {
    let contractions_map = load_contractions().map_err(|e| format!("Error loading contractions: {}", e))?;

    // Map the `input` to its corresponding contraction
    let mut result: Vec<String> = Vec::new();
    if let Some(expansion) = expand_contraction(input.clone().to_lowercase(), &contractions_map) {
        match expansion.get(0) {
            Some(first_expansion) => {
                result = first_expansion
                    .to_string()
                    .split_whitespace()
                    .map(|s| s.to_string())  // Convert each &str to String
                    .collect();                                          // Collect the results into a Vec<String>

                let output = input.clone();
                if let Some(first_char) = output.chars().next() {
                    if first_char.is_uppercase() {
                        // Access the first element of result mutably using indexing, not `get()`
                        result[0].replace_range(0..first_char.len_utf8(), &first_char.to_string().as_str());
                    }
                } else { return Err(String::from("Input is an empty string.")); }
            }
            None => return Err("Empty contraction vector.".to_string()),  // Error if empty
        }
    }

    // If no contractions are found, return the original input as a single-element vector
    if result.is_empty() { Ok(vec![input]) } else { Ok(result) }
}


/// Test that the `expand_contraction` function correct expands out contractions.
#[test]
fn test_expand_contraction() {
    // Test expansion of "you're" to "you" and "are".
    let result = find_contractions("you're".to_string());
    assert_eq!(result, Ok(vec!["you".to_string(), "are".to_string()]));

    // Test expansion of "it's" to "it" and "is".
    let result = find_contractions("it's".to_string());
    assert_eq!(result, Ok(vec!["it".to_string(), "is".to_string()]));

    // Test expansion of "It's" to "it" and "is".
    let result = find_contractions("It's".to_string());
    assert_eq!(result, Ok(vec!["It".to_string(), "is".to_string()]));

    // Test expansion of "chocolate" (it shouldn't expand so should just return ['chocolate'].
    let result = find_contractions("chocolate".to_string());
    assert_eq!(result, Ok(vec!["chocolate".to_string()]));

    // Ensuring `Chocolate` remains capitalised on output.
    let result = find_contractions("Chocolate".to_string());
    assert_eq!(result, Ok(vec!["Chocolate".to_string()]));
}
