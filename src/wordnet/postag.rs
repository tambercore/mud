use std::fmt;

/// Enumeration to represent different parts of speech (POS) tags.
/// These tags are used to classify words according to their syntactic roles.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WordnetTag {
    SatelliteAdjective,
    Noun,
    Adjective,
    Verb,
    Adverb,
}



/// Implement the Display trait for the WordnetTag enum.
/// This allows WordnetTag to be printed as a human-readable string.
impl fmt::Display for WordnetTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /* Match the WordnetTag variant and write its corresponding string representation. */
        let pos_str = match self {
            WordnetTag::SatelliteAdjective => "Satellite Adjective",
            WordnetTag::Noun => "Noun",
            WordnetTag::Adjective => "Adjective",
            WordnetTag::Verb => "Verb",
            WordnetTag::Adverb => "Adverb",
        };
        write!(f, "{}", pos_str)
    }
}



/// Implement the From trait for converting a string slice into a WordnetTag.
/// This enables the conversion from string-based POS tags to WordnetTag enum variants.
impl From<&str> for WordnetTag {
    fn from(s: &str) -> Self {
        /* Match the string input and return the corresponding WordnetTag variant. */
        match s {
            "s" => WordnetTag::SatelliteAdjective,
            "n" => WordnetTag::Noun,
            "a" => WordnetTag::Adjective,
            "v" => WordnetTag::Verb,
            "r" => WordnetTag::Adverb,
            _ => panic!("Unknown POS tag: {}", s),
        }
    }
}




