use crate::ast::import_decl::Import;
use crate::ast::infix_decl::Infix;
use crate::ast::literate_prose::Literate;
use crate::ast::postulate_decl::Postulate;
use crate::ast::record_decl::Record;
use crate::ast::theorem_decl::Theorem;
use crate::ast::var_declaration::VarDecl;

/// Enum to describe a top-level declaration in an AgdaFile.
/// These can be literate (tex segments), or programmatic.
#[derive(PartialEq, Clone)]
pub enum TDeclaration {
    ImportDecl(Import),
    InfixDecl(Infix),
    PostulateDecl(Postulate),
    TheoremDecl(Theorem),
    VariableDecl(VarDecl),
    RecordDecl(Record),
    LiterateProse(Literate),
    CommentSegment(String)
}

