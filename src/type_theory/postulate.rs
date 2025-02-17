use crate::type_theory::master_type::Type;
use crate::type_theory::record_type::FieldType;

pub struct Postulate {
    elements: [PostulateEntry]
}

pub struct PostulateEntry {
    iden : String,
    typ : Type,
}