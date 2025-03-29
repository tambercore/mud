use crate::ast::agda_expr::AgdaExpr;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{RecordDecl, VariableDecl};
use crate::interpreter::structure::{get_interpretation, INTERPRETATIONS};

/// Function to interpret Agsy's proof of a conclusion in natural language.
pub fn interpret_proof(expr: AgdaExpr, knowledge_base: TDeclaration, conclusion: TDeclaration) -> Vec<String> {
    let mut derivations = vec![];

    add_assumptions(&knowledge_base, &mut derivations);

    _interpret_proof(expr, &derivations);

    derivations
}

/// Function to gather assumptions from the knowledge base and
/// add them as natural language assumptions.
pub fn add_assumptions(knowledge_base: &TDeclaration, derivations: &mut Vec<String>) {
    if let RecordDecl(knowledge) = knowledge_base {
        for (idx, field) in knowledge.fields.iter().enumerate() {

            /* Retrieve the natural language interpretation of each field. */
            let interpretation = get_interpretation(&field.clone()).expect(format!("Expecting field to have an interpretation :{:?}", field).as_str());
            derivations.push(format!("A{}: {}", idx, interpretation));
        }

    } else {panic!("Expecting knowledge base to be a record.")}
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