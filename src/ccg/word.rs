use std::fmt;
use serde::{Deserialize, Deserializer};
use crate::brill::wordclass::Wordclass;

#[derive(Debug, Clone, Hash, Eq)]
/// Structure to represent a word with its associated part of speech.
pub struct CCGWord {
    pub text: String,
    pub tag: Wordclass,
}



impl PartialEq for CCGWord {
    /// Function to check if two `CCGWord` instances are equal based on text and tag.
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.tag == other.tag
    }
}



/// Implementation of `Display` for `CCGWord`, formatting it as "text (tag)".
impl fmt::Display for CCGWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.text, self.tag)
    }
}



impl<'de> Deserialize<'de> for CCGWord {
    /// Function to deserialize a `CCGWord` from a string, defaulting the tag to `Wordclass::ANY`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(CCGWord {
            text: value,
            tag: Wordclass::ANY,
        })
    }
}
