
use std::{fmt};
use std::io::{Error, ErrorKind};
use super::wordclass::{Wordclass};

/// Represents a lexical rule used in transformation-based tagging.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct LexicalRulespec {
    pub ruleset_id: LexicalRuleID,
    pub target_tag: Wordclass,
    pub parameters: Vec<String>,
}

/// LexicalRuleID Enumeration
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum LexicalRuleID {
    FHASSUF,
    FCHAR,
    HASSUF,
    ADDSUF,
    FGOODRIGHT,
    DELETEPREF,
    FGOODLEFT,
    GOODLEFT,
    GOODRIGHT,
    FDELETESUF,
    CHAR,
    FDELETEPREF,
    FADDSUF,
    FHASPREF,
    DELETESUF,
}

/// Implementation to display LexicalRuleID in CLI.
impl fmt::Display for LexicalRuleID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            LexicalRuleID::FHASSUF => "Final Has Suffix",
            LexicalRuleID::FCHAR => "Final Character",
            LexicalRuleID::HASSUF => "Has Suffix",
            LexicalRuleID::ADDSUF => "Add Suffix",
            LexicalRuleID::FGOODRIGHT => "Final Good Right",
            LexicalRuleID::DELETEPREF => "Delete Prefix",
            LexicalRuleID::FGOODLEFT => "Final Good Left",
            LexicalRuleID::GOODLEFT => "Good Left",
            LexicalRuleID::GOODRIGHT => "Good Right",
            LexicalRuleID::FDELETESUF => "Final Delete Suffix",
            LexicalRuleID::CHAR => "Character",
            LexicalRuleID::FDELETEPREF => "Final Delete Prefix",
            LexicalRuleID::FADDSUF => "Final Add Suffix",
            LexicalRuleID::FHASPREF => "Final Has Prefix",
            LexicalRuleID::DELETESUF => "Delete Suffix",
        };
        write!(f, "{}", name)
    }
}

/// Function to map string to LexicalRuleID
pub fn map_lexical_rule_id(string: &str) -> Result<LexicalRuleID, Error> {
    match string {
        "fhassuf" => Ok(LexicalRuleID::FHASSUF),
        "fchar" => Ok(LexicalRuleID::FCHAR),
        "hassuf" => Ok(LexicalRuleID::HASSUF),
        "addsuf" => Ok(LexicalRuleID::ADDSUF),
        "fgoodright" => Ok(LexicalRuleID::FGOODRIGHT),
        "deletepref" => Ok(LexicalRuleID::DELETEPREF),
        "fgoodleft" => Ok(LexicalRuleID::FGOODLEFT),
        "goodleft" => Ok(LexicalRuleID::GOODLEFT),
        "goodright" => Ok(LexicalRuleID::GOODRIGHT),
        "fdeletesuf" => Ok(LexicalRuleID::FDELETESUF),
        "char" => Ok(LexicalRuleID::CHAR),
        "fdeletepref" => Ok(LexicalRuleID::FDELETEPREF),
        "faddsuf" => Ok(LexicalRuleID::FADDSUF),
        "fhaspref" => Ok(LexicalRuleID::FHASPREF),
        "deletesuf" => Ok(LexicalRuleID::DELETESUF),
        _ => Err(Error::new(ErrorKind::InvalidData, format!("Invalid LexicalRuleID Identifier: {}", string))),
    }
}
