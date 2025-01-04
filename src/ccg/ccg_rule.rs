use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CCGRule {
    BackwardApplication,                    // "BA"
    BackwardComposition,                    // "BC"
    BackwardCrossedComposition,             // "BX"
    BackwardTypeRaising,                    // "BTR"
    Conjunction,                            // "CONJ"
    ForwardApplication,                     // "FA"
    ForwardComposition,                     // "FC"
    ForwardCrossedComposition,              // "FX"
    ForwardTypeRaising,                     // "FTR"
    GeneralizedBackwardComposition,         // "GBC"
    GeneralizedBackwardCrossedComposition,  // "GBX"
    GeneralizedForwardComposition,          // "GFC"
    GeneralizedForwardCrossedComposition,   // "GFX"
    Lexical,                                // "L"
    RemovePunctuationLeft,                  // "LP"
    RemovePunctuationRight,                 // "RP"
    Unary,                                  // "U"
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

fn main() {
    // Example usage
    let rule = CCGRule::ForwardApplication;
    println!("The rule {:?} has code '{}'.", rule, rule.as_str());
}