


/* Enumeration to denote Simple & Function types in Agda */
#[derive(Clone, Debug, PartialEq)]
pub enum AgdaType {
    Simple(String),
    Function(Box<AgdaType>, Box<AgdaType>),
    DepFunc(String, Box<AgdaType>, Box<AgdaType>),
    Application(Box<AgdaType>, Box<AgdaType>),
    RecordProj(Box<AgdaType>, Box<AgdaType>),
    Product(Box<AgdaType>, Box<AgdaType>)
}


#[macro_export]
macro_rules! τSimp {
    ($s:expr) => {
        Box::from(AgdaType::Simple($s))
    };
}

#[macro_export]
macro_rules! τFunc {
    ($from:expr, $to:expr) => {
        Box::from(AgdaType::Function($from, $to))
    };
}

#[macro_export]
macro_rules! τApp {
    ($func:expr, $arg:expr) => {
        Box::from(AgdaType::Application($func, $arg))
    };
}

#[macro_export]
macro_rules! τRecProj {
    ($proj:expr, $arg:expr) => {
        Box::from(AgdaType::RecordProj($proj, $arg))
    };
}

#[macro_export]
macro_rules! τDepFunc {
    ($iden:expr, $_type:expr, $rest:expr) => {
        Box::from(AgdaType::DepFunc($iden, $_type, $rest))
    };
}

#[macro_export]
macro_rules! τProduct {
    ($item1:expr, $item2:expr) => {
        Box::from(AgdaType::Product($item1, $item2))
    };
}