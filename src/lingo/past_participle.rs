use std::collections::HashMap;

pub(crate) fn get_past_participle(verb: String) -> String {
    // Step 1: Irregular verbs
    let mut irregular_verbs = HashMap::new();
    irregular_verbs.insert("eat", "eaten");
    irregular_verbs.insert("go", "gone");
    irregular_verbs.insert("write", "written");
    irregular_verbs.insert("see", "seen");

    // Check if the verb is irregular
    if let Some(&past_participle) = irregular_verbs.get(verb.as_str()) {
        return past_participle.to_string();
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

    // Rule 3: Verbs with a single vowel followed by a single consonant (CVC) double the final consonant before adding "ed"
    let len = verb.len();
    if len >= 3 {
        let (penultimate, last) = verb[len - 2..].chars().collect::<Vec<char>>()[..2];
        if penultimate.is_vowel() && last.is_consonant() && verb[len - 3..].chars().last().unwrap().is_consonant() {
            return format!("{}{}ed", &verb[..len - 1], last);
        }
    }

    // Rule 4: Verbs ending with "c" add "ked" (e.g., panic -> panicked)
    if verb.ends_with("c") {
        return format!("{}ked", verb);
    }

    // Rule 5: Verbs ending in "e" followed by consonants double the final consonant before adding "ed" (e.g., "hop" -> "hopped")
    if verb.ends_with("p") || verb.ends_with("t") || verb.ends_with("d") {
        return format!("{}ped", verb);
    }

    // Default rule: Add "ed" for other regular verbs
    format!("{}ed", verb)
}
