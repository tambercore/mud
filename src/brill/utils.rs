use std::collections::HashMap;
use crate::brill::wordclass::Wordclass;
use std::sync::OnceLock;

pub static TAG_MAPPING: OnceLock<Vec<(String, Vec<Wordclass>, Wordclass)>> = OnceLock::new();

/* Merge the mappings of words to possible tags, words to chosen tags, to a single vector containing a tuple of a word, its possible tags, and its chosen tag. */
pub fn create_tag_mapping(possible_tags: Vec<(String, Vec<Wordclass>)>, chosen_tags: Vec<(String, Wordclass)>) -> &'static Vec<(String, Vec<Wordclass>, Wordclass)> {
    TAG_MAPPING.get_or_init(|| {
        let possible_map: HashMap<String, Vec<Wordclass>> = possible_tags.into_iter().collect();
        let chosen_map: HashMap<String, Wordclass> = chosen_tags.into_iter().collect();

        let mut merged: Vec<(String, Vec<Wordclass>, Wordclass)> = Vec::new();

        for (word, chosen_tag) in chosen_map {
            if let Some(possible) = possible_map.get(&word) {
                merged.push((word, possible.clone(), chosen_tag));
            }
        }
        merged
    })
}
