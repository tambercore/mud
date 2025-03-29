use crate::ast::agda_expr::AgdaExpr;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::program::Program;
use crate::ast::record_decl::Record;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{PostulateDecl, RecordDecl, VariableDecl};
use crate::ast::var_declaration::VarDecl;
use crate::interpreter::structure::{get_interpretation, INTERPRETATIONS};
use crate::lambda::variable::Variable;

/// Function to interpret Agsy's proof of a conclusion in natural language.
pub fn interpret_proof(expr: AgdaExpr, program: &Program) -> Vec<String> {
    let mut derivations = vec![];

    _interpret_proof(expr, program, &mut derivations);

    println!("derivations: {:?}", derivations);

    derivations
}

/// Function to gather assumptions from the knowledge base and
/// add them as natural language assumptions.
pub fn add_assumptions(program: &Program, derivations: &mut Vec<String>) {
    let kb = find_record(String::from("KnowledgeBaseᵣ"), program).expect("Expected record.");
    for (idx, field) in kb.fields.iter().enumerate() {
        if let Term(field_iden) = *field.clone()._type {
            let field_record = RecordDecl(find_record(field_iden, program).expect("Expected record."));
            let interpretation = get_interpretation(&field_record).expect(format!("Expected record to have interpretation: {:?}", field_record.clone()).as_str());
            derivations.push(format!("A{} : {}", idx, interpretation));
        } else {panic!("Expected KB field to contain a term.")}

    }
}

pub fn find_record(iden: String, program: &Program) -> Option<Record> {
    for decl in &program.declarations {
        if let RecordDecl(record) = decl {
            if record.record_iden == iden {
                return Some(record.clone());
            }
        }
    }
    None
}

pub fn find_variable(iden: String, program: &Program) -> VarDecl {
    for decl in &program.declarations {
        if let PostulateDecl(postulate) = decl {
            for post in postulate.fields.clone() {
                if let VariableDecl(var_decl) = post {
                    if var_decl.iden == iden {
                        return var_decl.clone();
                    }
                }
            }
        }
    }
    panic!("Could not find Variable");
}

pub fn interpret_record_field(field: &VarDecl) -> String {
    get_interpretation(&VariableDecl(field.clone())).expect(format!("Missing interpretation: {:?}", field).as_str())
}
pub fn _interpret_proof(expr: AgdaExpr, program: &Program, derivations: &mut Vec<String>) {
    match expr {
        AgdaExpr::Term(term) => {
            /* Handle record constructions. */
            if term.ends_with("꜀") {
                /* Find the corresponding record. */
                let record = find_record(term.replace("꜀", "ᵣ"), program).expect("Expected record.");
                let interpretable_record = get_interpretation(&RecordDecl(record.clone())).expect(format!("Expected record to be interpretable: {:?}", record.clone()).as_str());

                /* Count the number of fields. */
                let num_fields = record.fields.len();

                /* Interpret the fields. */
                let interpretable_fields: Vec<String> = record.fields.iter()
                    .map(interpret_record_field)
                    .collect();

                /* Add a summary of the fields. */
                let interpreted_string = format!("To know that {}, it must be known that {}", interpretable_record, interpretable_fields.join(", "));
                derivations.push(interpreted_string);

                /* Expect that many applications. */

            }
            /* Handle normal terms. */
            else {

            }
        }
        AgdaExpr::App(app) => {
            _interpret_proof(*app.lhs, program, derivations);
            _interpret_proof(*app.rhs, program, derivations);

        }
        AgdaExpr::Abs(abs) => {
            /* For now, assume this is KB. */
            add_assumptions(program, derivations);
            _interpret_proof(*abs.expr, program, derivations);

        }
        AgdaExpr::RecProj(rec_proj) => {

            /* Only consider records (terms come as a result). */
            if let Some(record) = find_record(rec_proj.lhs.clone(), program) {
                /* Ignore the knowledge base (for now?). */
                if rec_proj.lhs.clone() != "KnowledgeBaseᵣ" {
                    /* Find the record field whose name is rec_proj.rhs */
                    for field in record.fields {

                    }
                }
            }

            /* Continue with the rest of the expression. */
            _interpret_proof(Term(rec_proj.rhs), program, derivations);

        }
        _ => unimplemented!()
    }
}