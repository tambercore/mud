use std::fmt;
use serde::Deserialize;



/// Structure to represent rules in the CCG, matching `lambeq`'s CCGRule Structure.
/// For more information, see: https://docs.quantinuum.com/lambeq/root-api.html#lambeq.CCGRule.
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize)]
    pub enum CCGRule {
        #[serde(rename = "BA")]
        BackwardApplication,

        #[serde(rename = "BC")]
        BackwardComposition,

        #[serde(rename = "BX")]
        BackwardCrossedComposition,

        #[serde(rename = "BTR")]
        BackwardTypeRaising,

        #[serde(rename = "CONJ")]
        Conjunction,

        #[serde(rename = "FA")]
        ForwardApplication,

        #[serde(rename = "FC")]
        ForwardComposition,

        #[serde(rename = "FX")]
        ForwardCrossedComposition,

        #[serde(rename = "FTR")]
        ForwardTypeRaising,

        #[serde(rename = "GBC")]
        GeneralizedBackwardComposition,

        #[serde(rename = "GBX")]
        GeneralizedBackwardCrossedComposition,

        #[serde(rename = "GFC")]
        GeneralizedForwardComposition,

        #[serde(rename = "GFX")]
        GeneralizedForwardCrossedComposition,

        #[serde(rename = "L")]
        Lexical,

        #[serde(rename = "LP")]
        RemovePunctuationLeft,

        #[serde(rename = "RP")]
        RemovePunctuationRight,

        #[serde(rename = "U")]
        Unary,

        #[serde(rename = "UNK")]
        Unknown,
    }



/// Implementation of pretty print for `CCGRule`.
impl fmt::Display for CCGRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rule_str = match self {
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
        };
        write!(f, "{}", rule_str)
    }
}


