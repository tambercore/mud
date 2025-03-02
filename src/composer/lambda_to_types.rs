use std::collections::HashMap;
use std::ptr::eq;
use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, AgdaFile, AgdaStructure, DefinitionInserter, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::Simple;
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{λPred, λVar, τApp, τDepFunc, τProduct, τRecProj, τSimp};
use crate::brill::utils::TAG_MAPPING;
use crate::brill::wordclass::Wordclass;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::types::LambdaEntity::{App, Var};
use crate::composer::case_converter::*;


/*
Quantifiers thinking through

every x                                 Every pig snorts ( given a pig, get a proof it snorts )

every x     - verb ->   y               Every man likes cheese ( for every man, there is a cheese that he likes )

y           - verb ->   every x         John likes every cheese ( for every cheese, john likes it )

every x     - verb ->   every y         Every man likes every woman ( for every man e1, for every woman e2, e1 likes e2 )


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


pub fn contains_uquant(l: Box<LambdaEntity>) -> bool {
    match *l {
        LambdaEntity::Pred(p) => {
            if p.iden == "every" { true }
            else {
                for a in p.args {
                    if contains_uquant(a) { return true } else { continue }
                }
                return false
            }
        }
        _ => {false}
    }
}


pub fn compose_predicate(mut p: Predicate, f: &mut AgdaFile, props: Vec<Variable>) -> String {

    /* Handle the unwrapping the onion of is */
    if p.iden == "is" && p.args.len() > 1 {
        /* Pop args into props */
        let mut final_idx = p.args.len() - 1;
        let mut lastarg = (p.args.clone())[final_idx].clone();
        match *lastarg.clone() {
            Var(v) => {
                let mut new_props = props.clone();
                new_props.push(v);
                let _ = p.args.pop();
                return compose_predicate(p.clone(), f, new_props)
            }
            LambdaEntity::Pred(mut inner_p) => {
                let inner_arg = inner_p.args.pop().unwrap();
                let mut new_props = props.clone();
                new_props.push(Variable{ name: inner_p.iden, id: None });
                p.args[final_idx] = inner_arg;
                return compose_predicate(p.clone(), f, new_props)
            }
            _ => { panic!("There is an `is` predicate that contains something that isn't pred/var.")}
        }
    } else if p.iden == "is" && p.args.len() == 1 {
        println!("Current Predicate: {}\nCurrent Props {:?}\n", p, props);
    }


    /* Initialise for dependent stuff */
    let mut uquants: Vec<(Variable, Box<LambdaEntity>)> = vec![];
    let mut equants: Vec<(Variable, Box<LambdaEntity>)> = vec![];

    /* Factor out UQuantifiers into uquants & EQuantifiers into equants */
    for i in 0..p.clone().args.len() {
        let mut arg = p.args.get(i).unwrap();
        match contains_uquant((arg.clone()).into()) {
            true => {
                /* It's a universal quantifier node! Move into uquants and replace with `a` */
                /* This is usually in the form every(P(x)) -> P(x)                          */
                let mut internal = match *arg.clone() {
                    LambdaEntity::Pred(p) => p.args.get(0).unwrap().clone(),
                    _ => { panic!("Universal Quantification can't unwrap the every.") }
                };

                uquants.push((
                    Variable{ name: format!("a{}", to_unicode_subscript(uquants.len() + 1)), id: None },
                    internal.clone()));

                p.args[i] = λVar!(format!("a{}", to_unicode_subscript(uquants.len() + 0)));
            }
            false => {
                /* It's a existential quantifier node! Move into equants and replace with `e` */
                equants.push((
                    Variable{ name: format!("e{}", to_unicode_subscript(equants.len() + 1)), id: None },
                    Box::from(*(arg.clone()))));

                p.args[i] = λVar!(format!("e{}", to_unicode_subscript(equants.len() + 0)));
            }
        }
    }

    /* At this point, the arguments to P should contain only references to objects in uquants and equants */
    println!("Current Predicate: {}\nQuants: {:?}\nEQuants: {:?}\n\n", p, uquants, equants);

    let mut symbol_table: HashMap<String, String> = HashMap::new();

    /* We need to propose that the predicate itself is some propositional function over entity */
    /* i.e. e to e to Set                                                                      */
    let arg_c = p.args.len();
    let mut iden = format!("{}", p.iden);
    f.insert_postulate(PostulateEntry(iden.clone(), generate_function_header(arg_c)));

    /* Handle Entity Fields */
    let mut fields: Vec<RecordField> = vec![];
    for (field_iden, arg) in equants {

        /* This will likely rely on records from here! */
        let rec_name = compose(arg.clone(), f, vec![]);
        fields.push(RecordField(field_iden.to_string(), Simple(rec_name.clone())));
        symbol_table.insert(field_iden.name, rec_name);
    }

    /* Build the proof type as: iden e₁ e₂ ... eₙ */
    /* Uses Record Projection to get the inner Entity type */
    let mut var_idens = vec![];
    for current_arg in p.clone().args {
        match *current_arg.clone() {
            Var(v) => { var_idens.push(v.name) }
            _ => { panic!("Predicate still contains non-bound argument.") }
        }
    }

    /* Compose UQuants */
    for (current, typ) in uquants.clone() {
        let rec_name = compose(typ, f, vec![]);
        symbol_table.insert(current.name, rec_name);
    }

    let mut inner = τSimp!("Temporary".parse().unwrap());
    if p.iden != "is" {
         inner = var_idens.iter().fold(
            τSimp!(iden.clone()),
            |acc, name| {
                τApp!(acc,
                    τApp!(
                        τRecProj!( τSimp!(symbol_table.get(name).unwrap().clone()) , τSimp!("e₁".to_string()) ),
                        τSimp!(name.clone())
                    )
                )
            }

        );
    } else {
        /* If it's an is, then this inside will be well... different! */
        if uquants.is_empty() {
            match *(p.args.get(0).unwrap().clone()) {
                Var(v) => { return compose_variable(v, f, props) }
                _ => { panic!("Invalid!") }
            }
        }

        /* This is now if we're saying `every` something is something! */
        let mut props_copy = props.clone();
        let mut returned_proofs: Vec<Box<AgdaType>> = vec![];
        while !props_copy.is_empty() {
            let current_prop = props_copy.pop().unwrap();
            let mut c_predicate = convert_case(format!("is_{}", current_prop).as_str(), CaseStyle::CamelCase);
            let (source_iden, typ) = uquants.get(0).unwrap();

            returned_proofs.push(τApp!(τSimp!(c_predicate.clone()),
                τApp!(
                        τRecProj!( τSimp!(symbol_table.get(&source_iden.name).unwrap().clone()) , τSimp!("e₁".to_string()) ),
                        τSimp!(source_iden.clone().name)
                    )
            ));

            f.insert_postulate(PostulateEntry(c_predicate, generate_function_header(1)));
        }

        if returned_proofs.len() == 0 { panic!("Something has gone wrong!") }
        if returned_proofs.len() == 1 { inner = returned_proofs.pop().unwrap() }
        else if returned_proofs.len() > 1 {
            /* Construct the return type as a product of the returned proofs */
            inner = returned_proofs.into_iter().rev().fold(None, |acc, proof| {
                match acc {
                    None => Some(proof),
                    Some(prod) => Some(τProduct!(proof, prod))
                }
            }).unwrap();
        }

    }

    /* For every uquant */
    while uquants.len() > 0 {
        let (current, typ) = uquants.pop().unwrap();
        let rec_name =  symbol_table.get(&current.name.clone()).unwrap();

        inner = τDepFunc!(current.name, τSimp!(rec_name.clone()), inner.clone());
    }

    fields.push(RecordField("p".to_string(), *inner));



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
