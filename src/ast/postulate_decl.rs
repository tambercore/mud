use crate::ast::top_decl::TDeclaration;
use crate::ast::var_declaration::VarDecl;

/// A type to denote postulate in Agda.
/// Consists of a list of fields and an optional Comment.
#[derive(Eq, Hash, PartialEq, Clone)]
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
        code.push_str("postulate\n");

        for field in self.fields {
            let field_code = field.agdaify();
            for line in field_code.lines() {
                code.push_str(&format!("    {}\n", line));
            }
        }

        code
    }
}
