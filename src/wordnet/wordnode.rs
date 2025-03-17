use std::fmt;
use crate::wordnet::postag::WordnetTag;

/// Structure to represent a word in the Wordnet system.
/// Contains information about the word's meaning, part of speech, ID, and synonyms.
#[derive(Debug, Clone)]
pub struct Wordnode {
    pub(crate) meaning: String,
    pub(crate) pos: WordnetTag,
    pub(crate) id: String,
    pub(crate) synonyms: Vec<String>,
}



/// Implement fmt::Display for Wordnode.
/// This allows Wordnode to be printed as a human-readable string, showing its meaning, POS, ID, and synonyms.
impl fmt::Display for Wordnode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /* Format the Wordnode as a string showing its details such as meaning, part of speech, ID, and synonyms. */
        write!(f, "Meaning: {}\nPOS: {}\nID: {}\nSynonyms: {}", self.meaning, self.pos, self.id,
               if self.synonyms.is_empty() { "None".to_string() } else { self.synonyms.join(", ") }
        )
    }
}



