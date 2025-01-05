use std::fmt;
use serde::{Deserialize, Deserializer};
use crate::brill::wordclass::Wordclass;
use crate::ccg::ccg_type::CCGType;
use crate::ccg::ccg_type_parser::parse_category;

#[derive(Debug, Clone)]
pub struct CCGWord {
    pub text: String,
    pub tag: Wordclass
}

// Implement Display for CCGWord
impl fmt::Display for CCGWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.text, self.tag)
    }
}

impl<'de> Deserialize<'de> for CCGWord {
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