use crate::lambda::types::LambdaEntity;

/*
    Agda File has

    POSTULATE

    then

    SOME BODY
*/

pub fn compose(e: Box<LambdaEntity>) -> () {
    match *e {
        LambdaEntity::App(_) => { panic!("Critical! System failed to compute output.") }
        LambdaEntity::Abs(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Var(_) => {}
        LambdaEntity::Pred(_) => {}
        LambdaEntity::Conj(_) => {}
        LambdaEntity::DepFun(_) => {}
        LambdaEntity::DepSum(_) => {}
    }
}
