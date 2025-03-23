
/* Singular Field */
/* use crate::ast::var_declaration::VarDecl;
use crate::composer::agdaify::format_agda_type;
use crate::composer::structures::AgdaType;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordField(pub String, pub AgdaType);



/* Entire Record Definition */
#[derive(Clone, Debug, PartialEq)]
pub struct RecordDefinition {
    pub(crate) record_name: String,
    pub(crate) constructor_name: String,
    pub(crate) fields: Vec<RecordField>
}


impl RecordDefinition {
    /// Converts the RecordDefinition into valid Agda code.
    pub fn agdaify(&self) -> String {
        let mut code = String::new();
        // Start the record declaration with its name and type.
        code.push_str(&format!("record {} : Set where\n", self.record_name));
        // Specify the constructor.
        code.push_str(&format!("  constructor {}\n", self.constructor_name));
        // Begin the field section.
        code.push_str("  field\n");
        // Iterate over each field and add it to the code.

        for VarDecl{iden: field_name, _type: field_type} in &self.fields {
            let type_str = format_agda_type(field_type);
            code.push_str(&format!("    {} : {}\n", field_name, type_str));
        }
        code
    }
} */