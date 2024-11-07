use std::fmt;
use std::io::{Error, ErrorKind};

/// RulespecID Enumeration
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum RulespecID {
    PREVTAG,
    PREVWD,
    PREV1OR2TAG,
    PREV1OR2OR3TAG,
    NEXT1OR2OR3TAG,
    WDAND2TAGAFT,
    WDAND2AFT,
    PREV1OR2WD,
    NEXT1OR2TAG,
    NEXTTAG,
    PREV2TAG,
    NEXTWD,
    WDNEXTTAG,
    SURROUNDTAG,
    WDAND2TAGBFR,
    RBIGRAM,
    PREVBIGRAM,
    CURWD,
    WDPREVTAG,
    NEXTBIGRAM,
    NEXT2TAG,
    LBIGRAM,
}



/// Implementation to display RulespecIDs in CLI.
impl fmt::Display for RulespecID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            RulespecID::PREVTAG => "Previous Tag",
            RulespecID::PREVWD => "Previous Word",
            RulespecID::PREV1OR2TAG => "Previous 1 or 2 Tags",
            RulespecID::PREV1OR2OR3TAG => "Previous 1, 2, or 3 Tags",
            RulespecID::NEXT1OR2OR3TAG => "Next 1, 2, or 3 Tags",
            RulespecID::WDAND2TAGAFT => "Word and 2 Tags After",
            RulespecID::WDAND2AFT => "Word and 2 After",
            RulespecID::PREV1OR2WD => "Previous 1 or 2 Words",
            RulespecID::NEXT1OR2TAG => "Next 1 or 2 Tags",
            RulespecID::NEXTTAG => "Next Tag",
            RulespecID::PREV2TAG => "Previous 2 Tags",
            RulespecID::NEXTWD => "Next Word",
            RulespecID::WDNEXTTAG => "Word and Next Tag",
            RulespecID::SURROUNDTAG => "Surrounding Tag",
            RulespecID::WDAND2TAGBFR => "Word and 2 Tags Before",
            RulespecID::RBIGRAM => "Right Bigram",
            RulespecID::PREVBIGRAM => "Previous Bigram",
            RulespecID::CURWD => "Current Word",
            RulespecID::WDPREVTAG => "Word and Previous Tag",
            RulespecID::NEXTBIGRAM => "Next Bigram",
            RulespecID::NEXT2TAG => "Next 2 Tags",
            RulespecID::LBIGRAM => "Left Bigram",
        };
        write!(f, "{}", name)
    }
}



pub fn map_rulespec_id(string: &str) -> Result<RulespecID, Error> {
    match string {
        "PREVTAG" => Ok(RulespecID::PREVTAG),
        "PREVWD" => Ok(RulespecID::PREVWD),
        "PREV1OR2TAG" => Ok(RulespecID::PREV1OR2TAG),
        "PREV1OR2OR3TAG" => Ok(RulespecID::PREV1OR2OR3TAG),
        "WDAND2TAGAFT" => Ok(RulespecID::WDAND2TAGAFT),
        "WDAND2AFT" => Ok(RulespecID::WDAND2AFT),
        "PREV1OR2WD" => Ok(RulespecID::PREV1OR2WD),
        "NEXT1OR2TAG" => Ok(RulespecID::NEXT1OR2TAG),
        "NEXTTAG" => Ok(RulespecID::NEXTTAG),
        "RBIGRAM" => Ok(RulespecID::RBIGRAM),
        "PREV2TAG" => Ok(RulespecID::PREV2TAG),
        "NEXTWD" => Ok(RulespecID::NEXTWD),
        "WDNEXTTAG" => Ok(RulespecID::WDNEXTTAG),
        "SURROUNDTAG" => Ok(RulespecID::SURROUNDTAG),
        "NEXT2TAG" => Ok(RulespecID::NEXT2TAG),
        "NEXT1OR2OR3TAG" => Ok(RulespecID::NEXT1OR2OR3TAG),
        "WDAND2TAGBFR" => Ok(RulespecID::WDAND2TAGBFR),
        "PREVBIGRAM" => Ok(RulespecID::PREVBIGRAM),
        "WDPREVTAG" => Ok(RulespecID::WDPREVTAG),
        "NEXTBIGRAM" => Ok(RulespecID::NEXTBIGRAM),
        "LBIGRAM" => Ok(RulespecID::LBIGRAM),
        "CURWD" => Ok(RulespecID::CURWD),
        _ => Err(Error::new(ErrorKind::InvalidData, format!("Invalid RulespecID Identifier: {}", string))),
    }
}