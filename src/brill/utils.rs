use std::collections::HashMap;
use crate::brill::wordclass::Wordclass;
use std::sync::OnceLock;


/// Lazily initialized static containing a merged mapping of words to their
/// possible and chosen tags.
///
/// Each entry is a tuple `(word, possible_tags, chosen_tag)`.
/// This is initialized once and reused for subsequent calls to `create_tag_mapping`.
pub static TAG_MAPPING: OnceLock<Vec<(String, Vec<Wordclass>, Wordclass)>> = OnceLock::new();



/// Constructs a global mapping of words to their possible and chosen part-of-speech tags.
///
/// This function merges two input vectors:
/// - A vector mapping words to possible `Wordclass` tags (e.g. from a lexicon).
/// - A vector mapping words to their selected `Wordclass` tag (e.g. from a gold standard).
///
/// The result is a vector of tuples for each word with both its possible tags and chosen tag,
/// returned as a static reference. Only entries found in both maps are included.
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
