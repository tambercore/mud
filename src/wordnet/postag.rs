use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WordnetTag {
    SatelliteAdjective,
    Noun,
    Adjective,
    Verb,
    Adverb,
}

impl fmt::Display for WordnetTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl From<&str> for WordnetTag {
    fn from(s: &str) -> Self {
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
