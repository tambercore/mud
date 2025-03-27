use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::ast::agda_expr::AgdaExpr;
use crate::wordnet::wordnode::Wordnode;

/// Static singleton to store interpretation mappings.
/// This is initialized lazily and is protected by a Mutex for thread safety.
static INTERPRETATIONS: Lazy<Mutex<HashMap<String, Interpretation>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// A type to denote Natural Language Interpretations
#[derive(Debug, Clone)]
pub struct Interpretation
{
    /* The natural language statement, e.g. "John likes Gouda" */
    pub statement: String,

    /* The source of the derivation, e.g. "WordNet" */
    pub source: String
}

