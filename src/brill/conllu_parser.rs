use std::fs::File;
use std::io::{self, BufRead};

/// Enum representing Universal POS tags.
#[derive(Debug, Clone, PartialEq)]
pub enum UPOS {
    ADJ,      // Adjective
    ADP,      // Adposition
    ADV,      // Adverb

    // In the Penn Treebank tag set, verbs are split across several tags (VB, VBD, VBP, VBZ, etc.)
    // which do not translate to the UPOS tag set (treat AUX as VERB)
    //AUX,      // Auxiliary

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

    // the tag for IN (which includes both prepositions and subordinating conjunctions) may
    // cause issues if the model does not accurately differentiate between these two usages.
    // This could lead to mistags where SCONJ is being mapped to ADP incorrectly.
    SCONJORADP,


}

/// Struct representing a token parsed from a CoNLL-U file.
#[derive(Debug, Clone)]
pub struct Token {
    pub id: String,
    pub form: String,
    pub lemma: String,
    pub upos: Option<UPOS>, // Optional UPOS field
    pub xpos: Option<String>, // Optional XPOS field
    pub feats: Option<String>, // Optional features
    pub head: Option<String>, // Optional head index
    pub deprel: Option<String>, // Optional dependency relation
    pub start: usize, // Start character index
    pub end: usize, // End character index
}

/// Function to parse a CoNLL-U file and return a vector of sentences (each sentence is a vector of tokens).
pub fn parse_conllu_file(filepath: &str) -> Result<Vec<Vec<Token>>, io::Error> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);
    let mut sentences = Vec::new();
    let mut current_sentence = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if !current_sentence.is_empty() {
                sentences.push(current_sentence);
                current_sentence = Vec::new();
            }
            continue; // Skip empty lines
        }

        // Split the line by tabs (CoNLL-U format)
        let fields: Vec<&str> = line.split('\t').collect();

        // Skip lines with a multi-word token ID, indicated by a range like '17-18'
        if fields[0].contains('-') {
            continue; // Skip MWT lines
        }

        if fields.len() >= 10 { // Ensure there are enough fields
            let id = fields[0].to_string();
            let form = fields[1].to_string();
            let lemma = fields[2].to_string();
            let upos = match fields[3] {
                "ADJ" => Some(UPOS::ADJ),
                "ADP" => Some(UPOS::SCONJORADP),
                "ADV" => Some(UPOS::ADV),
                "AUX" => Some(UPOS::VERB),
                "CCONJ" => Some(UPOS::CCONJ),
                "DET" => Some(UPOS::DET),
                "INTJ" => Some(UPOS::INTJ),
                "NOUN" => Some(UPOS::NOUN),
                "NUM" => Some(UPOS::NUM),
                "PART" => Some(UPOS::PART),
                "PRON" => Some(UPOS::PRON),
                "PROPN" => Some(UPOS::PROPN),
                "PUNCT" => Some(UPOS::PUNCT),
                "SCONJ" => Some(UPOS::SCONJORADP),
                "SYM" => Some(UPOS::SYM),
                "VERB" => Some(UPOS::VERB),
                _ => None, // Handle any other cases
            };

            // Create a new Token and push it to the current sentence
            let token = Token {
                id,
                form,
                lemma,
                upos,
                xpos: fields.get(4).map(|s| s.to_string()), // Optional field
                feats: fields.get(5).map(|s| s.to_string()), // Optional field
                head: fields.get(6).map(|s| s.to_string()), // Optional field
                deprel: fields.get(7).map(|s| s.to_string()), // Optional field
                start: 0, // Add actual start/end indices if needed
                end: 0,
            };
            current_sentence.push(token);
        }
    }

    // Push the last sentence if not empty
    if !current_sentence.is_empty() {
        sentences.push(current_sentence);
    }

    Ok(sentences)
}
