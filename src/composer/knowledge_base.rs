use crate::composer::postulate::{AgdaFile, AgdaStructure, DefinitionInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::AgdaType;
use crate::composer::structures::AgdaType::Simple;
use crate::monty::fresh_variable::to_unicode_subscript;

pub type KnowledgeBase = Vec<(String, AgdaType)>;

pub fn compose_kb(kb: KnowledgeBase, f: &mut AgdaFile) -> (String, AgdaType) {

    /* KB is a vector of records, each representing a premesis in the system */
    let mut assumptions: Vec<RecordField> = vec![];
    let mut assumtion_index = 1;

    for (rec_name, rec_type) in kb {
        assumptions.push(RecordField(
            format!("j{}", to_unicode_subscript(assumtion_index)),
            Simple(rec_name)
        ));
        assumtion_index = assumtion_index + 1;
    }

    let rec = AgdaStructure::RecordDef(RecordDefinition {
        record_name: "KnowledgeBaseᵣ".parse().unwrap(),
        constructor_name: "KnowledgeBase꜀".parse().unwrap(),
        fields: assumptions,
    });

    f.insert_definition(rec);
    ("KnowledgeBaseᵣ".parse().unwrap(), Simple("KnowledgeBaseᵣ".parse().unwrap()))
}