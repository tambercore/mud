
/* Structure for Postulate Entry */
use crate::composer::structures::AgdaType;

#[derive(Clone, Debug, PartialEq)]
pub struct PostulateEntry(pub String, pub AgdaType);



/* Structure for the Entire Agda File */
#[derive(Clone, Debug, PartialEq)]
pub struct AgdaFile {
    pub filename: String,
    pub postulate: Vec<PostulateEntry>
}



pub fn initialise_agda_file() -> AgdaFile {
    let mut f = AgdaFile{
        filename: "test".to_string(),
        postulate: vec!(),
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
        self.postulate.push(entry);
    }
}