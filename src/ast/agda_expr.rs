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
///
/// This enum represents various types of expressions in the Agda language. Each variant corresponds to a specific kind of expression in Agda. The variants include:
/// - `Term`: Represents a basic term (e.g., variable or constant).
/// - `UnOp`: Represents a unary operation (e.g., necessity or possibility).
/// - `BinOp`: Represents a binary operation (e.g., equality, product).
/// - `App`: Represents a function application.
/// - `Abs`: Represents an abstraction (e.g., lambda expression).
/// - `Quant`: Represents a quantification (e.g., universal or existential quantifier).
/// - `FunType`: Represents a function type (e.g., `A → B`).
/// - `DepFun`: Represents a dependent function (e.g., a function dependent on a variable's type).
/// - `RecProj`: Represents a record projection (e.g., accessing a field in a record).
/// - `QuestionMark`: Represents a placeholder or an unknown expression.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
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



/// A macro for creating a `Term` variant of `AgdaExpr`.
/// This macro takes a string identifier and returns an `AgdaExpr::Term` variant that represents a basic term in Agda.
#[macro_export]
macro_rules! term {
    ($iden:expr) => {
        Term(String::from($iden))
    };
}



/// Helper function that prints an `AgdaExpr` with awareness of operator precedence.
///
/// This function generates a string representation of the given Agda expression, ensuring proper formatting by taking into account operator precedence.
/// Lower precedence values indicate looser binding, and the function wraps expressions in parentheses when the inner expression binds more tightly than the outer one.
///
/// # Parameters
/// - `agda_type`: The Agda expression to format.
/// - `prec`: The current precedence level used to determine whether parentheses are needed.
///
/// # Returns
/// A string representing the formatted Agda expression.
pub fn format_agda_type_prec(agda_type: &AgdaExpr, prec: u8) -> String {
    match agda_type {
        AgdaExpr::Term(s) => {s.clone()}
        AgdaExpr::UnOp(unop) => {
            match unop.op {
                // The necessity operator (□) is formatted with the inner expression.
                Operator::Necessity => {format!("□ {}", format_agda_type_prec(&*unop.expr, prec))}
                // The possibility operator (◇) is formatted similarly.
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

        // If it's an application, format the left-hand side (function) and right-hand side (argument).
        AgdaExpr::App(app) => {
            // Function application binds tighter than the function arrow.
            let my_prec = 2;
            let func_str = format_agda_type_prec(&*app.lhs, my_prec);
            // The argument is printed in an even tighter context.
            let arg_str = format_agda_type_prec(&*app.rhs, my_prec + 1);
            let s = format!("{} {}", func_str, arg_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        // If it's a function type (→), format the left-hand side and right-hand side.
        AgdaExpr::FunType(func) => {
            // Function arrow (→) has precedence level 1.
            let my_prec = 1;
            // Use a tighter context for the left-hand side.
            let from_str = format_agda_type_prec(&*func.lhs, my_prec + 1);
            let to_str = format_agda_type_prec(&*func.rhs, my_prec);
            let s = format!("{} → {}", from_str, to_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        // If it's a dependent function, format it with the bound variable and expression.
        AgdaExpr::DepFun(dfun) => {
            let rest_str = format_agda_type_prec(&*dfun.expr, prec);
            format!("({} : {}) → {}", dfun.bound_var.iden, format_agda_type_prec(&*dfun.bound_var._type, prec), rest_str)
        }

        // If it's a record projection (.), format the left-hand side (record) and right-hand side (field).
        AgdaExpr::RecProj(proj) => {
            // Record projection (.) binds very tightly.
            let my_prec = 3;
            let rec_str = format_agda_type_prec(&Term(proj.clone().lhs), my_prec);

            // Projection field is usually atomic; use a higher precedence.
            let proj_str = format_agda_type_prec(&proj.clone().rhs, 4);
            let s = format!("{}.{}", rec_str, proj_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        AgdaExpr::QuestionMark => {format!("?")}

        // If it's an abstraction (λ expression), format the variable and body of the abstraction.
        AgdaExpr::Abs(abs) => {
            let _prec = 1;
            let body_str = format_agda_type_prec(&*abs.expr, _prec);
            let s = format!("λ {} → {}", abs.var, body_str);
            if _prec < prec { format!("({})", s) } else { s }
        }

        // If it's a quantification (e.g., ∀ or ∃), format the quantifier and variables.
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



/// The public function that prints an Agda expression.
///
/// This function acts as a wrapper around `format_agda_type_prec`, starting the printing process with a base precedence of 0.
///
/// # Parameters
/// - `agda_type`: The Agda expression to format.
///
/// # Returns
/// A string representing the formatted Agda expression.
pub fn format_agda_type(agda_type: &AgdaExpr) -> String {
    format_agda_type_prec(agda_type, 0)
}
