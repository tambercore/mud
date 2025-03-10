use std::collections::HashMap;
use std::ptr::eq;
use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, AgdaFile, AgdaStructure, DefinitionInserter, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::{Application, Simple};
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{λPred, λVar, τApp, τDepFunc, τFunc, τProduct, τRecProj, τSimp};
use crate::brill::utils::TAG_MAPPING;
use crate::brill::wordclass::Wordclass;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::types::LambdaEntity::{App, Var};
use crate::composer::case_converter::*;
use crate::composer::compose_predicate::compose_predicate;

pub fn generate_function_header(arity: usize) -> AgdaType {
    if arity == 0 {
        AgdaType::Simple("Set".to_string())
    } else {
        AgdaType::Function(
            Box::new(AgdaType::Simple("Entity".to_string())),
            Box::new(generate_function_header(arity - 1)),
        )
    }
}


pub fn replace_innermost_simple(expr: AgdaType, new_value: AgdaType) -> AgdaType {
    match expr {

        // If the current expression is an App, recursively replace in the nested expression
        Application(lhs, rhs) => {
            let new_rhs = replace_innermost_simple(*rhs.clone(), new_value.clone());
            // let new_lhs = replace_innermost_simple(*lhs, new_value.clone());

            // Continue with recursive replacement on the right side of the app chain
            if let Application(_, _) = *rhs {
                Application(lhs.clone(), Box::new(new_rhs))
            } else {
                Application(lhs.clone(), Box::new(new_rhs))
            }
        }
        // If the current expression is a Simple, replace it with the new_value
        _ => new_value,
    }
}





pub fn compose_variable(v: Variable, f: &mut AgdaFile, props: Vec<Variable>) -> (String, AgdaType) {

    use AgdaType::*;
    let iden = v.name;

    /* Generate Fields */
    let mut predicate_iden = convert_case(format!("is_{}", iden).as_str(), CaseStyle::CamelCase);
    let mut fields: Vec<RecordField> = vec![ RecordField("e₁".to_string(), *τSimp!("Entity".to_string()))];
    fields.push(RecordField("p₁".to_string(),
                            *τApp!( τSimp!( predicate_iden.clone() ) , τSimp!("e₁".to_string()) )
    ));

    /* Generate each property as a proof */
    let mut counter: usize = 1;
    for p in (props.clone()) {
        counter = counter + 1;
        let mut c_predicate = convert_case(format!("is_{}", p.name).as_str(), CaseStyle::CamelCase);
        fields.push(RecordField(format!("p{}", to_unicode_subscript(counter)),
                                *τApp!( τSimp!( c_predicate.clone() ) , τSimp!("e₁".to_string()) )
        ));
        f.insert_postulate(PostulateEntry(c_predicate, generate_function_header(1)));
    }

    /* Now, we need to insert the record for it */
    let props_iden = format!("{}{}",
                             props.iter().fold(String::new(), |mut acc, p| { acc.push_str(&p.name); acc.push('_'); acc }),
                             iden);

    let record_name = format!("{}ᵣ", convert_case(props_iden.clone().as_str(), CaseStyle::PascalCase));
    let constructor_name = format!("{}꜀", convert_case(props_iden.clone().as_str(), CaseStyle::PascalCase));

    let rec = RecordDefinition {
        record_name: record_name.clone(),
        constructor_name: constructor_name,
        fields: fields,
    };

    /* We need to also update the postulate to include the isType function */
    f.insert_postulate(PostulateEntry(predicate_iden, generate_function_header(1)));
    f.insert_definition(AgdaStructure::RecordDef(rec));

    let projection = τApp!(τRecProj!( τSimp!(record_name.clone()) , τSimp!("e₁".to_string()) ), τSimp!("e₁".to_string()));
    (record_name, *projection)
}



pub fn compose_product(c: Conjunction, f: &mut AgdaFile) -> (String, AgdaType) {

    /* Extract projections */
    let proj1 = c.lhs;
    let proj2 = c.rhs;

    let proj1_iden = compose(proj1, f, vec![]);
    let proj2_iden = compose(proj2, f, vec![]);

    use AgdaType::*;

    /* These sometimes have record identifiers in them ᵣ, remove! */
    let iden: String = format!("{}×{}", proj1_iden.clone().0, proj2_iden.clone().0)
        .chars()
        .filter(|&c| c != 'ᵣ')
        .collect();

    /* Generate Fields */
    let mut fields: Vec<RecordField> = vec![
        RecordField("e₁".to_string(), *τSimp!(proj1_iden.clone().0)),
        RecordField("e₂".to_string(), *τSimp!(proj2_iden.clone().0))
    ];

    /* Now, we need to insert the record for it */
    let record_name = format!("{}ᵣ", convert_case(&*iden, CaseStyle::PascalCase));
    let constructor_name = format!("{}꜀", convert_case(&*iden, CaseStyle::PascalCase));

    let rec = RecordDefinition {
        record_name: record_name.clone(),
        constructor_name: constructor_name,
        fields: fields,
    };

    f.insert_definition(AgdaStructure::RecordDef(rec));
    (record_name, *τSimp!("Temporary".to_string()))
}


pub fn compose(e: Box<LambdaEntity>, f: &mut AgdaFile, props: Vec<Variable>) -> (String, AgdaType) {

    match *e {

        LambdaEntity::App(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Abs(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Var(v) => { compose_variable(v, f, props) }

        LambdaEntity::Pred(p) => { compose_predicate(p, f, props) }

        LambdaEntity::Conj(c) => { compose_product(c, f) }

        _ => { panic!("Compose failed.") }

    }

}

