
/* Singular Field */
use crate::composer::structures::AgdaType;

pub struct RecordField {
    field_name: String,
    field_type: AgdaType,
}



/* Entire Record Definition */
pub struct RecordDefinition {
    record_name: String,
    constructor_name: String,
    fields: Vec<RecordField>
}
