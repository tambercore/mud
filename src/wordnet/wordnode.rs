use std::fmt;
use crate::wordnet::postag::WordnetTag;

#[derive(Debug, Clone)]
pub struct Wordnode {
    pub(crate) meaning: String,
    pub(crate) pos: WordnetTag,
    pub(crate) id: String,
    pub(crate) synonyms: Vec<String>,
}

// Implement fmt::Display for WordDefinition
impl fmt::Display for Wordnode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Meaning: {}\nPOS: {}\nID: {}\nSynonyms: {}", self.meaning, self.pos, self.id,
            if self.synonyms.is_empty() { "None".to_string() } else { self.synonyms.join(", ") }
        )
    }
}