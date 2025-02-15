
#[derive(Clone, Debug, PartialEq)]
pub struct FieldType {
    id : String,
    ty : String
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordType {
    id: String,
    base_typ: String,
    constructor: String,
    fields: Vec<FieldType>
}