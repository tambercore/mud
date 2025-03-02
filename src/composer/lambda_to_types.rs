use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, AgdaFile, AgdaStructure, DefinitionInserter, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::Simple;
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{λPred, λVar, τApp, τRecProj, τSimp};
use crate::brill::utils::TAG_MAPPING;
use crate::brill::wordclass::Wordclass;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::types::LambdaEntity::{App, Var};
use crate::composer::case_converter::*;



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



pub fn compose_is(p: Predicate, f: &mut AgdaFile, props: Vec<Variable>) -> String {

    /*

     match lhs:
        Fine under the interpretation assumptions.
        Proper Noun     - - - >     Dependent function type
        Fine under the interpretation assumptions.
        Common Noun     - - - >     Dependent function type

        Broken.
        Verb            - - - >     Dependent function type
        seeing = believing is broken



     */

    /* Arg1 here is the subject, so this could be something like `alien socrates` */
    let arg1 = p.args.get(0).unwrap();

    /* Arg2 here is the property, so this could be something like `myth` , i.e. alien socrates is a myth */
    let mut arg2 = p.args.get(1).unwrap();

    match *arg2.clone() {
        Var(var) => {
            let mut props_new = props.clone();
            props_new.push(var);
            compose(Box::from(*(arg1.clone())), f, props_new)
        }

        LambdaEntity::Pred(right_p) => {
            let mut new_props = props.clone();
            new_props.push(Variable{name: right_p.iden});

            let refined_arg_two = right_p.args.get(0).unwrap();

            let _args = vec![Box::from(*(arg1.clone())), Box::from(*(refined_arg_two.clone()))];
            compose_is(
                Predicate{iden: String::from("is"), args: _args},
                f, new_props
            )
        }

        _ => { panic!("Compose Is Failed.") }
    }
}



pub fn compose_predicate(p: Predicate, f: &mut AgdaFile, props: Vec<Variable>) -> String {

    use AgdaType::*;

    let arg_c = p.args.len();
    let mut iden = format!("{}", p.iden);
    let mut pred_iden = format!("{}", p.iden);

    /* Is this predicate an adjective? */
    /* Add a 1-arity check here too, likely? */
    for (word, tags, tag) in TAG_MAPPING.get().unwrap() {
        if *iden == *word && [Wordclass::NN, Wordclass::JJ].contains(tag) {

            /* This predicate is an adjective! */
            let mut props_copy = props.clone();
            props_copy.push(Variable{name: iden});
            return compose(
                Box::from(*p.args.get(0).unwrap().clone()), f,
                props_copy
            )
        }
    }

    if (iden == "is") { return compose_is(p, f, vec![]) }

    /* We need to propose that the predicate is some propositional function */
    f.insert_postulate(PostulateEntry(iden.clone(), generate_function_header(arg_c)));

    /* Handle Entity Fields */
    let mut fields: Vec<RecordField> = vec![];
    let mut counter: usize = 0;
    for arg in p.args {

        /* This will likely rely on records from here! */
        let rec_name = compose(arg.clone(), f, props.clone());

        counter = counter + 1;
        iden.push_str(format!("_{}", rec_name.replace("ᵣ", "")).as_str());
        fields.push(RecordField(format!("e{}", to_unicode_subscript(counter)), Simple(rec_name)))
    }

    /* Build the proof type as: iden e₁ e₂ ... eₙ */
    /* Uses Record Projection to get the inner Entity type */
    let proof_type = fields.iter().fold(
        τSimp!(pred_iden.clone()),
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
    let record_name = format!("{}ᵣ", convert_case(&*iden, CaseStyle::PascalCase));
    let constructor_name = format!("{}꜀", convert_case(&*iden, CaseStyle::PascalCase));

    let rec = RecordDefinition {
        record_name: record_name.clone(),
        constructor_name: constructor_name,
        fields: fields,
    };

    f.insert_definition(AgdaStructure::RecordDef(rec));
    record_name
}



pub fn compose_variable(v: Variable, f: &mut AgdaFile, props: Vec<Variable>) -> String {

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
    return record_name;
}



pub fn compose_product(c: Conjunction, f: &mut AgdaFile) -> String {

    /* Extract projections */
    let proj1 = c.lhs;
    let proj2 = c.rhs;

    let proj1_iden = compose(proj1, f, vec![]);
    let proj2_iden = compose(proj2, f, vec![]);

    use AgdaType::*;

    /* These sometimes have record identifiers in them ᵣ, remove! */
    let iden: String = format!("{}×{}", proj1_iden.clone(), proj2_iden.clone())
        .chars()
        .filter(|&c| c != 'ᵣ')
        .collect();

    /* Generate Fields */
    let mut fields: Vec<RecordField> = vec![
        RecordField("e₁".to_string(), *τSimp!(proj1_iden.clone())),
        RecordField("e₂".to_string(), *τSimp!(proj2_iden.clone()))
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
    record_name
}



pub fn compose(e: Box<LambdaEntity>, f: &mut AgdaFile, props: Vec<Variable>) -> String {

    match *e {

        LambdaEntity::App(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Abs(_) => { panic!("Critical! System failed to compute output.") }

        LambdaEntity::Var(v) => { compose_variable(v, f, props) }

        LambdaEntity::Pred(p) => { compose_predicate(p, f, props) }

        LambdaEntity::Conj(c) => { compose_product(c, f) }

        _ => { panic!("Compose failed.") }

    }

}
