
/// Macro to create a terminal `SemanticTree` node from a string.
#[macro_export]
macro_rules! tToken {
    ($s:expr) => {
        Box::from(SemanticTree::Terminal($s))
    };
}



/// Macro to create a non-terminal (relation) `SemanticTree` node from a tuple.
#[macro_export]
macro_rules! tRelation {
    ($s:expr) => {
        Box::from(SemanticTree::NonTerminal($s))
    };
}


/// Macro to create a conjunction `SemanticTree` node from a pair of nodes.
#[macro_export]
macro_rules! tJoin {
    ($s:expr) => {
        Box::from(SemanticTree::Conj($s))
    };
}
