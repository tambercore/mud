use crate::ast::agda_expr::AgdaExpr;
use crate::ast::program::Program;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{RecordDecl, VariableDecl};
use crate::interpreter::structure::{get_interpretation, INTERPRETATIONS};

/// Function to interpret Agsy's proof of a conclusion in natural language.
pub fn interpret_proof(expr: AgdaExpr, program: &Program) -> Vec<String> {
    let mut derivations = vec![];

    add_assumptions(program, &mut derivations);

    _interpret_proof(expr, &derivations);

    derivations
}

/// Function to gather assumptions from the knowledge base and
/// add them as natural language assumptions.
pub fn add_assumptions(program: &Program, derivations: &Vec<String>) {
    
}

pub fn _interpret_proof(expr: AgdaExpr, derivations: &Vec<String>) {
    match expr {
        AgdaExpr::Term(term) => {}
        AgdaExpr::App(app) => {}
        AgdaExpr::Abs(abs) => {
            /* This should be the assumptions. */
        }
        AgdaExpr::RecProj(rec_proj) => {}
        _ => unimplemented!()
    }
}