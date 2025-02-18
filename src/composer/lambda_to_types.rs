use crate::composer::structures::{initialise_agda_file, AgdaFile, AgdaType, PostulateEntry, PostulateInserter};
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;

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

    f.insert_postulate(PostulateEntry(iden, generate_function_header(arg_c)));

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
