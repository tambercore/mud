use std::fmt;


/*
pub enum UPOS {
    ADJ,      // Adjective
    ADP,      // Adposition
    ADV,      // Adverb
    AUX,      // Auxiliary
    CCONJ,    // Coordinating conjunction
    DET,      // Determiner
    INTJ,     // Interjection
    NOUN,     // Noun
    NUM,      // Numeral
    PART,     // Particle
    PRON,     // Pronoun
    PROPN,    // Proper noun
    PUNCT,    // Punctuation
    SCONJ,    // Subordinating conjunction
    SYM,      // Symbol
    VERB,     // Verb
    X,        // Other
}
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Wordclass {
    CC,     // Coordinating conjunction
    CD,     // Cardinal number
    DT,     // Determiner
    EX,     // Existential there
    FW,     // Foreign word
    IN,     // Preposition or subordinating conjunction
    JJ,     // Adjective
    JJR,    // Adjective, comparative
    JJS,    // Adjective, superlative
    LS,     // List item marker
    MD,     // Modal
    NN,     // Noun, singular or mass
    NNS,    // Noun, plural
    NNP,    // Proper noun, singular
    NNPS,   // Proper noun, plural
    PDT,    // Predeterminer
    POS,    // Possessive ending
    PRPE,   // Personal pronoun
    PRPO,   // Possessive pronoun
    RB,     // Adverb
    RBR,    // Adverb, comparative
    RBS,    // Adverb, superlative
    RP,     // Particle
    SYM,    // Symbol
    TO,     // to
    UH,     // Interjection
    VB,     // Verb, base form
    VBD,    // Verb, past tense
    VBG,    // Verb, gerund or present participle
    VBN,    // Verb, past participle
    VBP,    // Verb, non-3rd person singular present
    VBZ,    // Verb, 3rd person singular present
    WDT,    // Wh-determiner
    WPR,    // Wh-pronoun
    WPO,    // Possessive wh-pronoun
    WRB,    // Wh-adverb
    PUNC,   // Punctuation
    ANY,    // Any, used in contextual rules.
    NUM,
}

impl fmt::Display for Wordclass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wordclass_str = match self {
            Wordclass::CC => "Coordinating conjunction",
            Wordclass::CD => "Cardinal number",
            Wordclass::DT => "Determiner",
            Wordclass::EX => "Existential there",
            Wordclass::FW => "Foreign word",
            Wordclass::IN => "Preposition or subordinating conjunction",
            Wordclass::JJ => "Adjective",
            Wordclass::JJR => "Adjective (comparative)",
            Wordclass::JJS => "Adjective (superlative)",
            Wordclass::LS => "List item marker",
            Wordclass::MD => "Modal",
            Wordclass::NN => "Noun (singular or mass)",
            Wordclass::NNS => "Noun (plural)",
            Wordclass::NNP => "Proper noun (singular)",
            Wordclass::NNPS => "Proper noun (plural)",
            Wordclass::PDT => "Predeterminer",
            Wordclass::POS => "Possessive ending",
            Wordclass::PRPE => "Personal pronoun",
            Wordclass::PRPO => "Possessive pronoun",
            Wordclass::RB => "Adverb",
            Wordclass::RBR => "Adverb (comparative)",
            Wordclass::RBS => "Adverb (superlative)",
            Wordclass::RP => "Particle",
            Wordclass::SYM => "Symbol",
            Wordclass::TO => "to",
            Wordclass::UH => "Interjection",
            Wordclass::VB => "Verb (base form)",
            Wordclass::VBD => "Verb (past tense)",
            Wordclass::VBG => "Verb (gerund or present participle)",
            Wordclass::VBN => "Verb (past participle)",
            Wordclass::VBP => "Verb (non-3rd person singular present)",
            Wordclass::VBZ => "Verb (3rd person singular present)",
            Wordclass::WDT => "Wh-determiner",
            Wordclass::WPR => "Wh-pronoun",
            Wordclass::WPO => "Possessive wh-pronoun",
            Wordclass::WRB => "Wh-adverb",
            Wordclass::PUNC => "Punctuation",
            Wordclass::NUM => "Numeric",
            Wordclass::ANY => "Any!",

        };
        write!(f, "{}", wordclass_str)
    }
}

/// Function to map strings representing POS tags to their enum representation.
pub fn map_pos_tag(tag: &str) -> Option<Wordclass> {
    match tag {
        "CC" => Some(Wordclass::CC),
        "CD" => Some(Wordclass::CD),
        "DT" => Some(Wordclass::DT),
        "EX" => Some(Wordclass::EX),
        "FW" => Some(Wordclass::FW),
        "IN" => Some(Wordclass::IN),
        "JJ" => Some(Wordclass::JJ),
        "JJR" => Some(Wordclass::JJR),
        "JJS" => Some(Wordclass::JJS),
        "LS" => Some(Wordclass::LS),
        "MD" => Some(Wordclass::MD),
        "NN" => Some(Wordclass::NN),
        "NNS" => Some(Wordclass::NNS),
        "NNP" => Some(Wordclass::NNP),
        "NNPS" => Some(Wordclass::NNPS),
        "PDT" => Some(Wordclass::PDT),
        "POS" => Some(Wordclass::POS),
        "PRP" => Some(Wordclass::PRPE),
        "PRP$" => Some(Wordclass::PRPO),
        "RB" => Some(Wordclass::RB),
        "RBR" => Some(Wordclass::RBR),
        "RBS" => Some(Wordclass::RBS),
        "RP" => Some(Wordclass::RP),
        "SYM" => Some(Wordclass::SYM),
        "TO" => Some(Wordclass::TO),
        "UH" => Some(Wordclass::UH),
        "VB" => Some(Wordclass::VB),
        "VBD" => Some(Wordclass::VBD),
        "VBG" => Some(Wordclass::VBG),
        "VBN" => Some(Wordclass::VBN),
        "VBP" => Some(Wordclass::VBP),
        "VBZ" => Some(Wordclass::VBZ),
        "WDT" => Some(Wordclass::WDT),
        "WP" => Some(Wordclass::WPR),
        "WP$" => Some(Wordclass::WPO),
        "WRB" => Some(Wordclass::WRB),
        "." => Some(Wordclass::PUNC),
        "," => Some(Wordclass::PUNC),
        "!" => Some(Wordclass::PUNC),
        ";" => Some(Wordclass::PUNC),
        "(" => Some(Wordclass::PUNC),
        ")" => Some(Wordclass::PUNC),
        "-" => Some(Wordclass::PUNC),
        ":" => Some(Wordclass::PUNC),
        tag if tag.contains("|") => Some(Wordclass::ANY),
        _ => None,
    }
}
