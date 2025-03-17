use crate::composer::structures::{AgdaType};
use crate::composer::ast::AgdaAst;



/// Function to format and display an Agda AST with precedence. Handles rendering
/// of λ-term, λ-abs and λ-application with brackets where necessary.
fn format_agda_ast_prec(ast: &AgdaAst, prec: u8) -> String {
    match ast {
        AgdaAst::Term(name) => name.clone(),

        AgdaAst::LambdaAbstraction {param, body} => {
            let my_prec = 1;
            let body_str = format_agda_ast_prec(body, my_prec);
            let s = format!("λ ({}) → {}", param, body_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        AgdaAst::LambdaApplication { func, arg } => {
            let _prec = 2;
            let func_str = format_agda_ast_prec(func, _prec);
            let arg_str = format_agda_ast_prec(arg, _prec);
            let s = format!("{} {}", func_str, arg_str);
            if _prec < prec { format!("({})", s) } else { s }
        }
    }
}



/// Function to format and display an Agda AST. This calls the internal [`format_agda_ast_prec`]
/// with a starting precedence of zero.
pub fn format_agda_ast(ast: &AgdaAst) -> String {
    format_agda_ast_prec(ast, 0)
}