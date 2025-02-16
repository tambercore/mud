use crate::type_theory::utility::Agdaify;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldType {
    pub id: String,
    pub ty: String,
}

impl Agdaify for FieldType {
    fn to_agda(&self) -> String {
        format!("  {} : {}", self.id, self.ty)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordType {
    pub id: String,
    pub base_typ: String,
    pub constructor: String,
    pub fields: Vec<FieldType>,
}

impl Agdaify for RecordType {
    fn to_agda(&self) -> String {
        let mut agda_code = format!("record {} : {} where\n    constructor {}\n    field\n",
                                    self.id, self.base_typ, self.constructor);
        for field in &self.fields {
            agda_code.push_str(&format!("      {}\n", field.to_agda()));
        }
        agda_code
    }
}
