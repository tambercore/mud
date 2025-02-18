


/* Enumeration to denote Simple & Function types in Agda */
#[derive(Clone, Debug, PartialEq)]
pub enum AgdaType {
    Simple(String),
    Function(Box<AgdaType>, Box<AgdaType>),
    Application(Box<AgdaType>, Box<AgdaType>),
}


