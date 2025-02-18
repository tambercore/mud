


/* Enumeration to denote Simple & Function types in Agda */
pub enum AgdaType {
    iden(String),
    func(Box<AgdaType>, Box<AgdaType>)
}



/* Structure for Postulate Entry */
pub struct PostulateEntry {
    iden: String,
    _typ: AgdaType
}



/* Structure for the Entire Agda File */
pub struct AgdaFile {
    postulate: Vec<PostulateEntry>
}

