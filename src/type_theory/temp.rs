use crate::type_theory::function_type::FunctionType;
use crate::type_theory::record_type::{FieldType, RecordType};
use crate::type_theory::utility::Agdaify;

pub fn print_agda_examples() {
    // Create a RecordType
    let record = RecordType {
        id: "Person".to_string(),
        base_typ: "Set".to_string(),
        constructor: "mkPerson".to_string(),
        fields: vec![
            FieldType {
                id: "name".to_string(),
                ty: "String".to_string(),
            },
            FieldType {
                id: "age".to_string(),
                ty: "Nat".to_string(),
            },
        ],
    };

    println!("Agda Representation of RecordType:\n{}\n", record.to_agda());

    // Create a FunctionType
    let function = FunctionType {
        params: vec!["Nat".to_string(), "Nat".to_string()],
        return_type: "Bool".to_string(),
    };

    println!("Agda Representation of FunctionType:\n{}\n", function.to_agda());
}