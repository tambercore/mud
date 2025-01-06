use std::fmt;
use serde::{Deserialize, Deserializer, de::Error};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize)]
pub enum CCGRule {
    #[serde(rename = "BA")]
    BackwardApplication,                    // "BA"

    #[serde(rename = "BC")]
    BackwardComposition,                    // "BC"

    #[serde(rename = "BX")]
    BackwardCrossedComposition,             // "BX"

    #[serde(rename = "BTR")]
    BackwardTypeRaising,                    // "BTR"

    #[serde(rename = "CONJ")]
    Conjunction,                            // "CONJ"

    #[serde(rename = "FA")]
    ForwardApplication,                     // "FA"

    #[serde(rename = "FC")]
    ForwardComposition,                     // "FC"

    #[serde(rename = "FX")]
    ForwardCrossedComposition,              // "FX"

    #[serde(rename = "FTR")]
    ForwardTypeRaising,                     // "FTR"

    #[serde(rename = "GBC")]
    GeneralizedBackwardComposition,         // "GBC"

    #[serde(rename = "GBX")]
    GeneralizedBackwardCrossedComposition,  // "GBX"

    #[serde(rename = "GFC")]
    GeneralizedForwardComposition,          // "GFC"

    #[serde(rename = "GFX")]
    GeneralizedForwardCrossedComposition,   // "GFX"

    #[serde(rename = "L")]
    Lexical,                                // "L"

    #[serde(rename = "LP")]
    RemovePunctuationLeft,                  // "LP"

    #[serde(rename = "RP")]
    RemovePunctuationRight,                 // "RP"

    #[serde(rename = "U")]
    Unary,                                  // "U"

    #[serde(rename = "UNK")]
    Unknown,                                // "UNK"
}


impl CCGRule {
    pub fn as_str(&self) -> &'static str {
        match self {
            CCGRule::BackwardApplication => "BA",
            CCGRule::BackwardComposition => "BC",
            CCGRule::BackwardCrossedComposition => "BX",
            CCGRule::BackwardTypeRaising => "BTR",
            CCGRule::Conjunction => "CONJ",
            CCGRule::ForwardApplication => "FA",
            CCGRule::ForwardComposition => "FC",
            CCGRule::ForwardCrossedComposition => "FX",
            CCGRule::ForwardTypeRaising => "FTR",
            CCGRule::GeneralizedBackwardComposition => "GBC",
            CCGRule::GeneralizedBackwardCrossedComposition => "GBX",
            CCGRule::GeneralizedForwardComposition => "GFC",
            CCGRule::GeneralizedForwardCrossedComposition => "GFX",
            CCGRule::Lexical => "L",
            CCGRule::RemovePunctuationLeft => "LP",
            CCGRule::RemovePunctuationRight => "RP",
            CCGRule::Unary => "U",
            CCGRule::Unknown => "UNK",
        }
    }
}


impl fmt::Display for CCGRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

