use crate::ast::abstraction::Abstraction;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::application::Application;
use crate::ast::binary_op::BinOperator;
use crate::ast::dependent_function::DependentFunction;
use crate::ast::function_type::FunctionType;
use crate::ast::operator::Operator;
use crate::ast::operator::Operator::PropEq;
use crate::ast::quantification::Quantification;
use crate::ast::record_projection::RecordProjection;
use crate::ast::unary_op::UnOperator;

/// An enumeration of possible Expressions in Agda.
#[derive(PartialEq, Clone, Debug)]
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
        Term(String::from($iden))
    };
}



/// Helper function that prints an AgdaType with awareness of operator precedence.
/// Lower numbers indicate looser binding; we wrap in parentheses when the inner
/// expression's binding (my_prec) is less than the context (prec).
pub fn format_agda_type_prec(agda_type: &AgdaExpr, prec: u8) -> String {
    match agda_type {
        AgdaExpr::Term(s) => {s.clone()}
        AgdaExpr::UnOp(unop) => {
            match unop.op {
                Operator::Necessity => {format!("□ {}", format_agda_type_prec(&*unop.expr, prec))}
                Operator::Possibility => {format!("◇ {}", format_agda_type_prec(&*unop.expr, prec))}
                _ => panic!("Expected Unary Operator, found {:?}", unop.op)
            }
        }
        AgdaExpr::BinOp(binop) => {
            match binop.symbol {
                PropEq => {format!("{} ≡ {}", format_agda_type_prec(&*binop.lhs, prec), format_agda_type_prec(&*binop.rhs, prec))}
                Operator::Product => {format!("{} × {}", format_agda_type_prec(&*binop.lhs, prec), format_agda_type_prec(&*binop.rhs, prec))}
                _ => panic!("Expected Binary Operator, found {:?}", binop.symbol)
            }
        }
        AgdaExpr::App(app) => {
            // Function application binds tighter than the function arrow.
            let my_prec = 2;
            let func_str = format_agda_type_prec(&*app.lhs, my_prec);
            // The argument is printed in an even tighter context.
            let arg_str = format_agda_type_prec(&*app.rhs, my_prec + 1);
            let s = format!("{} {}", func_str, arg_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }
        AgdaExpr::FunType(func) => {
            // Function arrow (→) has precedence level 1.
            let my_prec = 1;
            // Use a tighter context for the left-hand side.
            let from_str = format_agda_type_prec(&*func.lhs, my_prec + 1);
            let to_str = format_agda_type_prec(&*func.rhs, my_prec);
            let s = format!("{} → {}", from_str, to_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }
        AgdaExpr::DepFun(dfun) => {
            let rest_str = format_agda_type_prec(&*dfun.expr, prec);
            format!("({} : {}) → {}", dfun.bound_var.iden, format_agda_type_prec(&*dfun.bound_var._type, prec), rest_str)
        }
        AgdaExpr::RecProj(proj) => {
            // Record projection (.) binds very tightly.
            let my_prec = 3;
            let rec_str = format_agda_type_prec(&*proj.lhs, my_prec);

            // Projection field is usually atomic; use a higher precedence.
            let proj_str = format_agda_type_prec(&*proj.rhs, 4);
            let s = format!("{}.{}", rec_str, proj_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }
        AgdaExpr::QuestionMark => {format!("?")}

        AgdaExpr::Abs(abs) => {
            let _prec = 1;
            let body_str = format_agda_type_prec(&*abs.expr, _prec);
            let s = format!("λ {} → {}", abs.var, body_str);
            if _prec < prec { format!("({})", s) } else { s }
        }
        AgdaExpr::Quant(quant) => {

            let my_prec = 3;

            let mut vars = String::new();
            for var in quant.vars.clone() {
                if let Term(t) = *var._type {
                    vars.push_str(format!("{{ {} : {} }}", var.iden, t).as_str());
                }
            }

            let body_str = format_agda_type_prec(&*quant.expr, my_prec);
            let s = format!("{} {} → {}", quant.symbol, vars, body_str);
            if my_prec < prec { format!("({})", s) } else { s }

        }


    }
}



/// The public function that prints an AgdaType.
/// It starts the printing process with a base precedence of 0.
pub fn format_agda_type(agda_type: &AgdaExpr) -> String {
    format_agda_type_prec(agda_type, 0)
}
