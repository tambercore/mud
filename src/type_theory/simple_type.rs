use uuid::fmt::Simple;
use crate::type_theory::utility::Agdaify;

pub struct SimpleType {
    iden: String
}

impl Agdaify for SimpleType {
    fn to_agda(&self) -> String {
        format!("{}", self.iden)
    }
}