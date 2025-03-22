use crate::ast::abstraction::Abstraction;
use crate::ast::application::Application;
use crate::ast::binary_op::BinOperator;
use crate::ast::dependent_function::DependentFunction;
use crate::ast::function_type::FunctionType;
use crate::ast::quantification::Quantification;
use crate::ast::record_projection::RecordProjection;
use crate::ast::unary_op::UnOperator;


/// An enumeration of possible Expressions in Agda.
#[derive(PartialEq)]
pub enum AgdaExpr {
    Term(String),
    UnOp(UnOperator),
    BinOp(BinOperator),
    App(Application),
    Abs(Abstraction),
    Quant(Quantification),
    FunType(FunctionType),
    DepFun(DependentFunction),
    RecProj(RecordProjection),
    QuestionMark
}

#[macro_export]
macro_rules! term {
    ($iden:expr) => {
        Box::from(Term(String::from($iden)))
    };
}
