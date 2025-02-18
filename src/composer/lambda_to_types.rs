use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, AgdaFile, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::Simple;
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{τApp, τRecProj, τSimp};
use crate::lambda::types::LambdaEntity::App;
/*
    Agda File has

    POSTULATE

    then

    SOME BODY
*/

pub fn generate_function_header(arity: usize) -> AgdaType
{
    if arity == 0 {
        AgdaType::Simple("Set".to_string())
    } else {
        AgdaType::Function(
            Box::new(AgdaType::Simple("Entity".to_string())),
            Box::new(generate_function_header(arity - 1)),
        )
    }
}



pub fn compose_predicate(p: Predicate, f: &mut AgdaFile) -> () {

    use AgdaType::*;

    let arg_c = p.args.len();
    let iden = p.iden;

    /* We need to propose that the predicate is some propositional function */
    f.insert_postulate(PostulateEntry(iden.clone(), generate_function_header(arg_c)));

    /* Handle Entity Fields */
    let mut fields: Vec<RecordField> = vec![];
    let mut counter: usize = 0;
    for arg in p.args {
        counter = counter + 1;
        match *(arg.clone()) {
            LambdaEntity::Var(v) => {
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
    let rec = RecordDefinition {
        record_name: iden.clone(),
        constructor_name: format!("c_{}", iden.clone()),
        fields: fields,
    };

    let s = rec.agdaify();
    println!("{}", s)

}



pub fn compose_variable(v: Variable, f: &mut AgdaFile) {

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

    let s = rec.agdaify();
    println!("{}", s)
}



pub fn compose(e: Box<LambdaEntity>, f: &mut AgdaFile) -> () {

    match *e {

        LambdaEntity::App(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Abs(_) => { panic!("Critical! System failed to compute output.") }


        LambdaEntity::Var(v) => { compose_variable(v, f) }

        LambdaEntity::Pred(p) => { compose_predicate(p, f); }

        LambdaEntity::Conj(_) => {}

        LambdaEntity::DepFun(_) => {}

        LambdaEntity::DepSum(_) => {}

    }

    return;
}
