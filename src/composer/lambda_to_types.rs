use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, AgdaFile, AgdaStructure, DefinitionInserter, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::Simple;
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{τApp, τRecProj, τSimp};
use crate::lambda::conjunction::Conjunction;
use crate::lambda::types::LambdaEntity::App;
/*
    Agda File has

    POSTULATE

    then

    SOME BODY
*/

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



pub fn compose_predicate(p: Predicate, f: &mut AgdaFile) -> String {

    use AgdaType::*;

    let arg_c = p.args.len();
    let mut record_name = format!("{}", p.iden);
    let mut iden = format!("{}", p.iden);

    /* We need to propose that the predicate is some propositional function */
    f.insert_postulate(PostulateEntry(iden.clone(), generate_function_header(arg_c)));

    /* Handle Entity Fields */
    let mut fields: Vec<RecordField> = vec![];
    let mut counter: usize = 0;
    for arg in p.args {
        counter = counter + 1;
        match *(arg.clone()) {
            LambdaEntity::Var(v) => {
                record_name.push_str(format!("_{}", v.name).as_str());
                fields.push(RecordField(format!("e{}", to_unicode_subscript(counter)), Simple(v.name)))
            }
            _ => {}
        }

        /* This will likely rely on records from here! */
        compose(arg, f);
    }


    /* Build the proof type as: iden e₁ e₂ ... eₙ */
    /* Uses Record Projection to get the inner Entity type */
    let proof_type = fields.iter().fold(
        τSimp!(iden.clone()),
        |acc, field| {
            τApp!(acc,
                τApp!(
                    τRecProj!( Box::new(field.1.clone()) , τSimp!("e₁".to_string()) ),
                    τSimp!(field.0.clone())
                )
            )
        }
    );
    fields.push(RecordField("p".to_string(), *proof_type));



    /* Now, we need to insert the record for it */
    iden = format!("{}ᵣ", iden.clone());
    let rec = RecordDefinition {
        record_name: record_name.clone(),
        constructor_name: format!("c_{}", iden.clone()),
        fields: fields,
    };

    f.insert_definition(AgdaStructure::RecordDef(rec));
    record_name.clone()
}



pub fn compose_variable(v: Variable, f: &mut AgdaFile) -> String {

    use AgdaType::*;
    let iden = v.name;

    /* Generate Fields */
    let mut fields: Vec<RecordField> = vec![ RecordField("e₁".to_string(), *τSimp!("Entity".to_string()))];
    fields.push(RecordField("p".to_string(),
        *τApp!( τSimp!(format!("is{}", iden)) , τSimp!("e₁".to_string()) )
    ));

    /* Now, we need to insert the record for it */
    let rec = RecordDefinition {
        record_name: iden.clone(),
        constructor_name: format!("c_{}", iden.clone()),
        fields: fields,
    };

    /* We need to also update the postulate to include the isType function */
    f.insert_postulate(PostulateEntry(format!("is{}", iden), generate_function_header(1)));

    f.insert_definition(AgdaStructure::RecordDef(rec));
    return iden;
}



pub fn compose_product(c: Conjunction, f: &mut AgdaFile) -> String {

    /* Extract projections */
    let proj1 = c.lhs;
    let proj2 = c.rhs;

    let proj1_iden = compose(proj1, f);
    let proj2_iden = compose(proj2, f);

    use AgdaType::*;
    let iden = format!("{}×{}", proj1_iden.clone(), proj2_iden.clone());

    /* Generate Fields */
    let mut fields: Vec<RecordField> = vec![
        RecordField("e₁".to_string(), *τSimp!(proj1_iden.clone())),
        RecordField("e₂".to_string(), *τSimp!(proj2_iden.clone()))
    ];


    /* Now, we need to insert the record for it */
    let rec = RecordDefinition {
        record_name: iden.clone(),
        constructor_name: format!("c_{}", iden.clone()),
        fields: fields,
    };

    f.insert_definition(AgdaStructure::RecordDef(rec));
    return iden;
}



pub fn compose(e: Box<LambdaEntity>, f: &mut AgdaFile) -> String {

    match *e {

        LambdaEntity::App(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Abs(_) => { panic!("Critical! System failed to compute output.") }


        LambdaEntity::Var(v) => { compose_variable(v, f) }

        LambdaEntity::Pred(p) => { compose_predicate(p, f) }

        LambdaEntity::Conj(c) => { compose_product(c, f) }

        _ => { panic!("Compose failed.") }

    }

}
