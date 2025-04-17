use std::collections::{HashSet};
use std::fs::read_to_string;
use std::io;
use std::io::Error;
use super::lex_rulespec_id::{map_lexical_rule_id, LexicalRulespec};
use super::wordclass::{map_pos_tag, Wordclass};

/// Parses a lexical ruleset from a file into a vector of `LexicalRulespec` structs.
///
/// Lexical rules are Brill-style transformation templates that define how a token’s tag
/// may change based on affix or substring information. Each line in the file represents
/// a rule with a specific format. This parser is designed to tolerate minor variations
/// in rule syntax by identifying the rulestring and target tag heuristically.
pub fn parse_lexical_ruleset(path: &str) -> Result<Vec<LexicalRulespec>, io::Error>
{
    let mut result: Vec<LexicalRulespec> = Vec::new();
    for line in read_to_string(path)?.lines() {

        let parts: Vec<&str> = line.split_whitespace().collect();

        // Brill's original lexical rules come in a (somewhat weird) variety of forms, with each rule varying in syntactic structure.
        // The only common attributes are the `rulestring` and `target_tag`, as some rules are source-tag ambiguous. This processes it.

        // Firstly, we will obtain the rulestring attribute, which is a primary identifier for the given lexical rule. This is either
        // the 2nd or 3rd token (meaning indexed at 1 or 2). If there is not a valid rulestring at either, then the something is wrong.
        let (ruleset_id, rulestring_index) = if let Some(maybe_rulestring) = parts.get(1) {
            match map_lexical_rule_id(maybe_rulestring) {
                Ok(rulespec) => (rulespec, 1), // Found at index 1
                Err(_) => {
                    if let Some(maybe_rulestring) = parts.get(2) {
                        match map_lexical_rule_id(maybe_rulestring) {
                            Ok(rulespec) => (rulespec, 2), // Found at index 2
                            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid rulestring")),
                        }
                    } else { return Err(io::Error::new(io::ErrorKind::InvalidData, "Couldn't grab 3rd token.")); }
                }
            }
        } else { return Err(io::Error::new(io::ErrorKind::InvalidData, "Couldn't grab 1st token.")); };

        // The `target_tag` is always the second to final token in the vector. We can find this by searching the
        // position at `len(parts) - 2`. If it's not here, something is wrong.
        const TARGET_TAG_INSET: i8 = 2;
        let target_tag_index: usize = parts.len() - TARGET_TAG_INSET as usize;
        let target_tag: Wordclass = if let Some(target_tag_str) = parts.get(target_tag_index) {
            map_pos_tag(target_tag_str)
                .ok_or_else(|| Error::new(io::ErrorKind::InvalidInput, "invalid target tag"))?

        } else { return Err(io::Error::new(io::ErrorKind::InvalidData, "Target tag not found.")); };

        // Finally, any additional parameters are collected, before the structure is added to the vector.
        // Parameters should be everything else, so copy each token over unless it's index is in the list {target_tag_index, rulestring_index}
        // Collect parameters by excluding the `rulestring` and `target_tag` tokens.
        let excluded_indices: HashSet<usize> = [rulestring_index, target_tag_index].iter().cloned().collect();
        let parameters: Vec<String> = parts .iter()
            .enumerate().filter_map(|(index, token)| {
            if !excluded_indices.contains(&index) { Some(token.to_string())}
            else { None } } ).collect();

        // Encapsulate the rule in the `LexicalRulespec` type, and push to the result vector.
        let new_rulespec = LexicalRulespec {
            ruleset_id, target_tag, parameters
        };
        result.push(new_rulespec);
    }
    Ok(result)
}

#[test]
pub fn test_parse_lexical_rules() {
    // Call parse_lexical_ruleset and handle the Result directly.
    let lexical_rules = parse_lexical_ruleset("data/rulefile_lexical.txt");

    // Assert that the result is Ok and contains the expected data.
    match lexical_rules {
        Ok(rules) => {
            // You can assert that the map is not empty, or check for specific keys/values
            assert!(!rules.is_empty(), "The lexical rules should not be empty.");

            // You could also check specific values inside the HashMap.
            for rule_specs in rules {
                // Example assertions: you might want to assert that specific Wordclasses
                // exist and that there are valid rule specs.
                println!("Rule: {:?}", rule_specs); // Optional printing for debugging.
            }
        },
        Err(e) => {
            // If the function returns Err, you can fail the test with a message.
            panic!("Failed to parse lexical rules: {}", e);
        }
    }
}