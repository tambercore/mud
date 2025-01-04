use crate::brill::wordclass::Wordclass;

#[derive(Debug, Clone)]
pub struct CCGWord {
    pub text: Option<String>,
    pub tag: Option<Wordclass>
}