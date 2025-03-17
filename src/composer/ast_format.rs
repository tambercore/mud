use crate::composer::ast::AgdaAst;



/// Function to format and display an Agda AST with precedence. Handles rendering
/// of λ-term, λ-abs and λ-application with brackets where necessary.
fn format_agda_ast_prec(ast: &AgdaAst, prec: u8) -> String {
    match ast {
        /* Render a singular, primitive term in the λ-calculus */
        AgdaAst::Term(name) => name.clone(),

        /* Render Lambda Abstractions in Agda-Notation `λ {} → {}` */
        AgdaAst::LambdaAbstraction {param, body} => {
            let _prec = 1;
            let body_str = format_agda_ast_prec(body, _prec);
            let s = format!("λ {} → {}", param, body_str);
            if _prec < prec { format!("({})", s) } else { s }
        }

        /* Render Lambda Applications (or function applications) */
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