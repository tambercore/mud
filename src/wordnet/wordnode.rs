use std::fmt;

#[derive(Debug, Clone)]
pub struct Wordnode {
    pub(crate) meaning: String,
    pub(crate) pos: String,
    pub(crate) id: String,
}

// Implement fmt::Display for WordDefinition
impl fmt::Display for Wordnode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Meaning: {}\nPOS: {}\nID: {}",
            self.meaning, self.pos, self.id
        )
    }
}
