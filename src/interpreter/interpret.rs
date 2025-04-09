use crate::ast::abstraction::Abstraction;
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::agda_expr::AgdaExpr::{App, Term};
use crate::ast::application::Application;
use crate::ast::program::Program;
use crate::ast::record_decl::Record;
use crate::ast::record_projection::RecordProjection;
use crate::ast::theorem_decl::Theorem;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{PostulateDecl, RecordDecl, TheoremDecl, VariableDecl};
use crate::ast::var_declaration::VarDecl;
use crate::interpreter::derivation::{print_derivations, Derivation, Derivations};
use crate::interpreter::interpretation_map::{get_interpretation, INTERPRETATIONS};
use crate::lambda::variable::Variable;
use crate::term;

/// Function to interpret Agsy's proof of a conclusion in natural language.
pub fn interpret_proof(expr: AgdaExpr, program: &Program) -> Derivations {
    let mut derivations = Derivations {contents: vec![]} ;
    let mut counter = 1;

    if let AgdaExpr::Abs(abs) = expr {
        add_assumptions(program, &mut derivations);
        _interpret_proof(*abs.expr, program, &mut derivations, &mut counter);
    }
    else {
        panic!("Expected proof to start with abstraction.");
    }

    print_derivations(&derivations);
    derivations
}

/// Function to gather assumptions from the knowledge base and
/// add them as natural language assumptions.

pub fn add_assumptions(program: &Program, derivations: &mut Derivations) {
    let kb = find_record(String::from("KnowledgeBaseᵣ"), program).expect("Expected record.");
    for (idx, field) in kb.fields.iter().enumerate() {
        if let Term(field_iden) = *field.clone()._type {
            let field_record = RecordDecl(find_record(field_iden, program).expect("Expected record."));
            let interpretation = get_interpretation(&field_record).expect(
                format!("Expected record to have interpretation: {:?}", field_record.clone()).as_str()
            );

            let id = format!("A{}", idx);
            derivations.contents.push(Derivation { contents: format!("{}", interpretation), expr: field_record, Id: id });
        } else { panic!("Expected KB field to contain a term.") }
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

/// Given a theorem identifier, return the theorem.
pub fn find_theorem(iden: String, program: &Program) -> Option<Theorem> {
    for decl in &program.declarations {
        if let TheoremDecl(theorem) = decl {
            if theorem.iden == iden {
                return Some(theorem.clone());
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



pub fn _interpret_proof(expr: AgdaExpr, program: &Program, derivations: &mut Derivations, counter: &mut i32) {
    match expr {
        AgdaExpr::Term(term) => interpret_term(term.clone(), program, derivations, counter),
        AgdaExpr::App(app) => interpret_application(app.clone(), program, derivations, counter),
        AgdaExpr::Abs(abs) => interpret_abstraction(abs.clone(), program, derivations, counter),
        AgdaExpr::RecProj(rec_proj) => interpret_record_projection(rec_proj.clone(), program, derivations, counter),
        _ => unimplemented!()
    }
}


pub fn interpret_term(term: String, program: &Program, derivations: &mut Derivations, counter: &mut i32) {

    /* Handle record constructions. */
    if term.ends_with("꜀") {

        let record = find_record(term.replace("꜀", "ᵣ"), program).expect("Expected record.");
        let interpretable_record = get_interpretation(&RecordDecl(record.clone())).expect(
            format!("Expected record to be interpretable: {:?}", record.clone()).as_str()
        );
        let interpretable_fields: Vec<String> = record.fields.iter()
            .map(interpret_record_field)
            .collect();

        let formatted_fields = interpretable_fields.join(", and ");


        let interpreted_string = format!(
            "To know that {}, it must be known that {}",
            interpretable_record, formatted_fields
        );

        derivations.contents.push(Derivation { contents: interpreted_string, expr: RecordDecl(record), Id: (*counter.to_string()).to_owned() });
        *counter += 1;
    }
    else {
        // todo: handle this properly

        println!("searching for {:?}", term.clone());

        /* Some terms are hardcoded, e.g. □-k. Check if the term has an interpretation. */
        if let Some(theorem) = find_theorem(term.clone(), program) {

            println!("found theorem {:?}", theorem.clone());
            if let Some(interpretation) = get_interpretation(&TheoremDecl(theorem.clone())) {
                println!("found interpretation: {:?}", interpretation.clone());
                derivations.contents.push(Derivation {contents: interpretation, expr: TheoremDecl(theorem), Id: (*counter.to_string()).to_owned()})
            }
        }

    }
}

pub fn interpret_application(app: Application, program: &Program, derivations: &mut Derivations, counter: &mut i32) {
    _interpret_proof(*app.lhs.clone(), program, derivations, counter);
    _interpret_proof(*app.rhs.clone(), program, derivations, counter);
}

pub fn interpret_abstraction(abs: Abstraction, program: &Program, derivations: &mut Derivations, counter: &mut i32) {
    // todo: do this properly
    _interpret_proof(*abs.expr.clone(), program, derivations, counter);
}

pub fn interpret_record_projection(rec_proj: RecordProjection, program: &Program, derivations: &mut Derivations, counter: &mut i32) {
    /* Only consider records (terms come as a result). */
    if let Some(record) = find_record(rec_proj.lhs.clone(), program) {
        /* todo : use the knowledge base?. */

        if rec_proj.lhs.clone() != "KnowledgeBaseᵣ" {
            /* Find the record field whose name is rec_proj.rhs */

            /* From _, it is known that ... */
            let proof_lhs = get_interpretation(&RecordDecl(record.clone())).expect("Expecting interpretation.");

            /* If rhs is a term, look for it directly */
            /* If rhs is an application, look for its lhs, and then _interpret_proof on its rhs */

            match *rec_proj.rhs.clone() {
                Term(rec_rhs) => {
                    construct_projection(record, rec_rhs, proof_lhs.clone(), derivations, counter);
                    return;
                }
                App(app_rhs) => {
                    if let Term(app_lhs) = *app_rhs.lhs {
                        _interpret_proof(*app_rhs.rhs, program, derivations, counter);
                        construct_projection(record, app_lhs, proof_lhs.clone(), derivations, counter);
                        return;
                    } else {panic!("Failed to parse: {:?}", app_rhs)}
                }
                _ => unimplemented!()
            }

        }
    }

    /* Continue with the rest of the expression. */
    _interpret_proof(*rec_proj.rhs, program, derivations, counter);
}

pub fn construct_projection(record: Record, rhs: String, proof_lhs: String, derivations: &mut Derivations, counter: &mut i32) {
    for field in record.clone().fields {
        if field.iden == rhs {
            let proof_rhs = get_interpretation(&VariableDecl(field.clone())).expect(format!("Expecting interpretation for {:?}.", field.clone()).as_str());
            let proof_lhs_id = derivations.find_id_by_contents(proof_lhs.clone().as_str()).expect("Expecting proof to have an ID.");
            let derivation = match *field._type.clone() {
                Term(term) => {
                    format!("Given from {} that {}, it is known that {}.", proof_lhs_id, proof_lhs.clone(), proof_rhs.clone())
                }
                AgdaExpr::App(app) => {
                    format!("Given from {} that {}, it is known that {}.", proof_lhs_id, proof_lhs.clone(), proof_rhs.clone())
                }

                AgdaExpr::DepFun(function) => {
                    format!("Given from {} that {}, {}.", proof_lhs_id, proof_lhs.clone(), proof_rhs.clone())
                }

                /* todo: is this always a modality? */
                AgdaExpr::UnOp(operator) => {
                    format!("Given from {} that {}, {}.", proof_lhs_id, proof_lhs.clone(), proof_rhs.clone())
                }
                _ => unimplemented!("{:?}", field)
            };
            derivations.contents.push(Derivation { contents: derivation, expr: RecordDecl(record), Id: (*counter.to_string()).to_owned()  });
            *counter += 1;
            return;
        }
    }
}


