#[macro_export]
macro_rules! tToken {
    ($s:expr) => {
        Box::from(SemanticTree::Terminal($s))
    };
}
#[macro_export]
macro_rules! tRelation {
    ($s:expr) => {
        Box::from(SemanticTree::NonTerminal($s))
    };
}
#[macro_export]
macro_rules! tJoin {
    ($s:expr) => {
        Box::from(SemanticTree::Conj($s))
    };
}
