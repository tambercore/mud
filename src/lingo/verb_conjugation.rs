use crate::lingo::consonants::CharExtensions;

/// Function to apply conjugation rules to convert regular verbs to their past participles, this
/// should be expanded in the future if we encounter missing cases.
pub fn apply_conjugation_rules(verb: String) -> String {

    /* Handle verbs ending in "e" simply add "d" */
    if verb.ends_with('e') {
        return format!("{}d", verb);
    }

    /* Handle verbs ending in consonant + "y" (excluding exceptions) change "y" to "ied" */
    let exceptions = vec!["ay", "ey", "iy", "oy", "uy"];
    if verb.ends_with('y') && !exceptions.iter().any(|&suffix| verb.ends_with(suffix)) {
        return format!("{}ied", &verb[..verb.len() - 1]);
    }

    /* Handle verbs with a CVC pattern (Consonant-Vowel-Consonant) double the final consonant */
    if verb.len() >= 3 {
        let chars: Vec<char> = verb[verb.len() - 3..].chars().collect();
        if chars.len() == 3 {
            let (first, second, third) = (chars[0], chars[1], chars[2]);

            /* Check if the pattern is CVC and the final consonant is not "p" or "t" */
            if first.is_consonant() && second.is_vowel() && third.is_consonant() && !matches!(third, 'p' | 't') {
                return format!("{}{}ed", &verb[..verb.len() - 1], third);
            }
        }
    }

    /* Handle verbs ending in "c" add "ked" */
    if verb.ends_with("c") {
        return format!("{}ked", verb);
    }

    /* Handle verbs with a CVC pattern (Consonant-Vowel-Consonant) double the final consonant */
    if verb.len() >= 3 {
        let chars: Vec<char> = verb[verb.len() - 3..].chars().collect();
        if chars.len() == 3 {
            let (first, second, third) = (chars[0], chars[1], chars[2]);

            /* Check if the pattern is CVC and the final consonant should be doubled */
            // Exclude certain consonants (e.g., "p" from "jump")
            if first.is_consonant() && second.is_vowel() && third.is_consonant() {
                /* Check for doubling consonant rule: avoid doubling if final consonant is "p" or specific exclusions */
                if !matches!(third, 'p') {
                    return format!("{}{}ed", &verb[..verb.len() - 1], third);
                }
            }
        }
    }

    /* Handle verbs ending with specific consonants (p, t, d) add "ped" */
    if verb.ends_with("p") || verb.ends_with("t") || verb.ends_with("d") {
        return format!("{}ped", verb);
    }

    /* Default rule is to add "ed" for other regular verbs */
    format!("{}ed", verb)
}
