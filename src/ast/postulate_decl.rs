use crate::ast::theorem_decl::Agdaify;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{CommentSegment, ImportDecl, InfixDecl, TheoremDecl, VariableDecl};
use crate::ast::var_declaration::VarDecl;

/// A type to denote postulate in Agda.
/// Consists of a list of fields and an optional Comment.
#[derive(PartialEq, Clone)]
pub struct Postulate {pub fields : Vec<TDeclaration>, pub comment : Option<String>}

#[macro_export]
macro_rules! postulate {
    ($fields:expr, $comment:expr) => {
        TDeclaration::PostulateDecl(Postulate {fields: $fields, comment: $comment})
    };
}

impl Postulate {
    pub fn agdaify(self) -> String {

        let mut code = String::new();

        if self.fields.len() == 0 {return code}

        code.push_str(&format!("postulate\n"));

        for entry in self.fields {
            match entry {
                ImportDecl(import) => code.push_str( &format!("   {}\n", import.agdaify())),
                InfixDecl(infix) => code.push_str( &format!("   {}\n", infix.agdaify())),

                /* tab each line of the theorem */
                TheoremDecl(theorem) => {
                    let indented_theorem = theorem.agdaify()
                        .lines()
                        .map(|line| format!("   {}", line))
                        .collect::<Vec<_>>()
                        .join("\n");
                    code.push_str(&format!("{}\n", indented_theorem));
                }
                VariableDecl(var) => code.push_str( &format!("   {}\n", var.agdaify())),
                CommentSegment(comment) => code.push_str( &format!("\n   -- {}\n", comment)),
                _ => panic!("Unexpected entry in postulate.")
            };
        }

        code
    }
}