use std::fmt;
use serde::{Deserialize, Deserializer};
use crate::ccg::type_parser::parse_category;



#[derive(Debug, Clone, PartialEq)]
pub enum CCGType {
    ForwardsFunctor(Box<CCGType>, Box<CCGType>),
    BackwardsFunctor(Box<CCGType>, Box<CCGType>),
    Conjunction,
    ConjunctionTag,
    Noun,
    NounPhrase,
    PrepositionalPhrase,
    Punctuation,
    Sentence,
    Empty
}



/// Implementation for `Deserialize` to parse CCG categories from JSON strings.
impl<'de> Deserialize<'de> for CCGType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(parse_category(value.as_str()).unwrap().1)
    }
}


impl CCGType {
    pub fn to_string(&self) -> String {
        match self {
            CCGType::ForwardsFunctor(left, right) => format!("({} / {})", left.to_string(), right.to_string()),
            CCGType::BackwardsFunctor(left, right) => format!("({} \\ {})", left.to_string(), right.to_string()),
            CCGType::Conjunction => "CONJ".to_string(),
            CCGType::ConjunctionTag => "[CONJ]".to_string(),
            CCGType::Noun => "N".to_string(),
            CCGType::NounPhrase => "NP".to_string(),
            CCGType::PrepositionalPhrase => "P".to_string(),
            CCGType::Punctuation => "PUNC".to_string(),
            CCGType::Sentence => "S".to_string(),
            CCGType::Empty => "NONE".to_string(),
        }
    }
}

impl fmt::Display for CCGType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}