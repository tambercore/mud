use crate::ast::agda_expr::AgdaExpr;
use crate::ast::top_decl::TDeclaration;

/// Function to interpret Agsy's proof of a conclusion in natural language.
pub fn interpret_proof(expr: AgdaExpr, conclusion: TDeclaration) -> Vec<String> {
    let derivations = vec![];

    add_assumptions(&derivations);

    _interpret_proof(expr, &derivations);

    derivations
}

/// Function to gather assumptions from the knowledge base and
/// add them as natural language assumptions.
pub fn add_assumptions(derivations : &Vec<String>) {

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