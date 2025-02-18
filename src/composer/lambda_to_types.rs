use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, AgdaFile, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::Simple;
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
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

    let arg_c = p.args.len();
    let iden = p.iden;

    /* We need to propose that the predicate is some propositional function */
    f.insert_postulate(PostulateEntry(iden.clone(), generate_function_header(arg_c)));



    let mut fields: Vec<RecordField> = vec![];
    let mut counter: usize = 0;
    for arg in p.args {
        counter = counter + 1;
        match *(arg.clone()) {
            LambdaEntity::Var(v) => {
                fields.push(RecordField(format!("e{}", to_unicode_subscript(counter)), AgdaType::Simple(v.name)))
            }
            _ => {}
        }

        /* This will likely rely on records from here! */
        compose(arg, f);
    }



    /* Build the proof type as: iden e₁ e₂ ... eₙ */
    let proof_type = fields.iter().fold(
        AgdaType::Simple(iden.clone()),
        |acc, field| {
            AgdaType::Application(Box::new(acc), Box::new(AgdaType::Simple(field.0.clone())))
        }
    );
    fields.push(RecordField("p".to_string(), proof_type));



    /* Now, we need to insert the record for it */
    let rec = RecordDefinition {
        record_name: iden.clone(),
        constructor_name: format!("c_{}", iden.clone()),
        fields: fields,
    };

    let s = rec.agdaify();
    println!("{}", s)

}

pub fn compose(e: Box<LambdaEntity>, f: &mut AgdaFile) -> () {

    match *e {

        LambdaEntity::App(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Abs(_) => { panic!("Critical! System failed to compute output.") }


        LambdaEntity::Var(_) => {}

        LambdaEntity::Pred(p) => { compose_predicate(p, f); }

        LambdaEntity::Conj(_) => {}

        LambdaEntity::DepFun(_) => {}

        LambdaEntity::DepSum(_) => {}

    }

    return;
}
