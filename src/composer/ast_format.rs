use crate::composer::structures::{AgdaType};
use crate::composer::ast::AgdaAst;

/// Helper function that prints an Agda AST node with proper parentheses based on precedence.
fn format_agda_ast_prec(ast: &AgdaAst, prec: u8) -> String {
    match ast {
        AgdaAst::Term(name) => name.clone(),

        AgdaAst::Lambda{param, body} => {
            let my_prec = 1;
            let body_str = format_agda_ast_prec(body, my_prec);
            let s = format!("λ ({}) → {}", param, body_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        AgdaAst::Application { func, arg } => {
            let my_prec = 2;
            let func_str = format_agda_ast_prec(func, my_prec);
            let arg_str = format_agda_ast_prec(arg, my_prec); // <-- use my_prec here instead of my_prec + 1
            let s = format!("{} {}", func_str, arg_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }
    }
}

/// Public function that initiates formatting with base precedence of 0.
pub fn format_agda_ast(ast: &AgdaAst) -> String {
    format_agda_ast_prec(ast, 0)
}

// You may reuse the existing function to format types
fn format_agda_type(typ: &AgdaType) -> String {
    crate::composer::agdaify::format_agda_type(typ)
}