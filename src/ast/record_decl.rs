use crate::ast::agda_expr::format_agda_type;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::VariableDecl;
use crate::ast::var_declaration::VarDecl;
/// A type to denote Records in Agda.
/// Consists of Record name, Constructor name, a list of record fields, and an optional comment.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Record {
    pub record_iden : String,
    pub constructor_iden : String,
    pub fields : Vec<VarDecl>,
    pub comment : Option<String>}

impl Record {
    pub fn agdaify(&self) -> String {
        let mut code = String::new();
        if let Some(comment) = &self.comment {
            code.push_str(&format!("-- Record declaration for '{}'\n", comment));
        }
        // Start the record declaration with its name and type.
        code.push_str(&format!("record {} : Set where\n", self.record_iden));
        // Specify the constructor.
        code.push_str(&format!("  constructor {}\n", self.constructor_iden));
        // Begin the field section.
        code.push_str("  field\n");
        // Iterate over each field and add it to the code.
        for field in &self.fields {
            let type_str = format_agda_type(&field._type);
            code.push_str(&format!("    {} : {}\n", field.iden, type_str));

        }
        code
    }
}

#[macro_export]
macro_rules! record {
    ($record_iden:expr, $constructor_iden:expr, $field:expr,$comment:expr) => {
        TDeclaration::RecordDecl(
        Record {
            record_iden: $record_iden.to_string(),
            constructor_iden: $constructor_iden.to_string(),
            fields: $field,
            comment: $comment
        })
    };
}