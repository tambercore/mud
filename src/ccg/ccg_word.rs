use crate::brill::wordclass::Wordclass;

#[derive(Debug, Clone)]
pub struct CCGWord {
    pub text: String,
    pub tag: Wordclass
}