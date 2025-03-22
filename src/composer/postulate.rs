
/* Structure for Postulate Entry */
use crate::ast::program::Program;
use crate::composer::function_def::FunctionDefinition;
use crate::composer::record::RecordDefinition;
use crate::composer::structures::AgdaType;

#[derive(Clone, Debug, PartialEq)]
pub struct PostulateEntry(pub String, pub AgdaType);

#[derive(Clone, Debug, PartialEq)]
pub enum AgdaStructure {
    RecordDef(RecordDefinition),
    FunctionDef(FunctionDefinition)
}


pub fn initialise_agda_file() -> Program {
    let mut f = Program{
        file_name: "test".to_string(),
        declarations: vec!(),
    };

    /* Add `Entity : Set` as a declaration */
    f.postulate.push(PostulateEntry("Entity".to_string(), AgdaType::Simple("Set".to_string())));

    f
}



/* Trait to insert a postulate entry into an AgdaFile */
pub trait PostulateInserter {
    fn insert_postulate(&mut self, entry: PostulateEntry);
}



/* Implement the trait for AgdaFile */
impl PostulateInserter for AgdaFile {
    fn insert_postulate(&mut self, entry: PostulateEntry) {
        if !self.postulate.contains(&entry) {
            self.postulate.push(entry);
        }
    }
}



pub trait DefinitionInserter {
    fn insert_definition(&mut self, entry: AgdaStructure);
}

impl DefinitionInserter for AgdaFile {
    fn insert_definition(&mut self, entry: AgdaStructure) {
        if !self.definitions.contains(&entry) {
            self.definitions.push(entry);
        }
    }
}