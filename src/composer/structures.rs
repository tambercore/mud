


/* Enumeration to denote Simple & Function types in Agda */
pub enum AgdaType {
    Simple(String),
    Function(Box<AgdaType>, Box<AgdaType>)
}



/* Structure for Postulate Entry */
type PostulateEntry = (String, AgdaType);



/* Structure for the Entire Agda File */
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
    f.postulate.push(("Entity".to_string(), AgdaType::Simple("Set".to_string())));

    f
}