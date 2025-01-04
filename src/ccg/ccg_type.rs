use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CCGType {
    ForwardsFunctor(Box<CCGType>, Box<CCGType>),
    BackwardsFunctor(Box<CCGType>, Box<CCGType>),
    Conjunction,
    ConjunctionTag,
    Noun,
    NounPhrase,
    PrepositionalPhrase,
    Punctuation,
    Sentence,
    Empty
}

impl fmt::Display for CCGType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CCGType::ForwardsFunctor(left, right) => write!(f, "({} / {})", left, right),
            CCGType::BackwardsFunctor(left, right) => write!(f, "({} \\ {})", left, right),
            CCGType::Conjunction => write!(f, "Conjunction"),
            CCGType::Noun => write!(f, "Noun"),
            CCGType::NounPhrase => write!(f, "Noun Phrase"),
            CCGType::PrepositionalPhrase => write!(f, "Prepositional Phrase"),
            CCGType::Punctuation => write!(f, "Punctuation"),
            CCGType::Sentence => write!(f, "Sentence"),
            CCGType::Empty => write!(f, "Empty Type")
        }
    }
}
