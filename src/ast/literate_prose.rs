use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{CommentSegment, RecordDecl, TheoremDecl, VariableDecl};

/// A type to denote literate segments in an AgdaFile.
/// LaTeX segments will be parsed as Strings.
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Literate {pub content : String}

#[macro_export]
macro_rules! literate {
    ($tt:tt) => {
        TDeclaration::LiterateProse(Literate { content: $tt })
    };
}

impl Literate {
    pub(crate) fn agdaify(&self) -> String {
        format!("\\end{{code}} \n\n {} \n\n \\begin{{code}}", self.content)
    }
}