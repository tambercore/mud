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
use crate::{tToken, λPred, λVar, τApp, τDepFunc, τFunc, τProduct, τRecProj, τSimp};
use crate::brill::utils::TAG_MAPPING;
use crate::brill::wordclass::Wordclass;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::types::LambdaEntity::{App, Var};
use crate::composer::case_converter::*;
use crate::composer::compose_variable::compose_variable;
use crate::composer::lambda_to_types::{compose, generate_function_header, replace_innermost_simple};
use crate::composer::langtree::{Relation, SemanticTree, Token};
use crate::composer::langtree::SemanticTree::Terminal;

pub fn contains_uquant(l: Box<SemanticTree>) -> bool {
    match *l {
        SemanticTree::NonTerminal(relation) => {
            if relation.0 == "every" { true }
            else {
                for a in relation.1 {
                    if contains_uquant(a) { return true } else { continue }
                }
                return false
            }
        }
        _ => false
    }
}

// When we handle 'is' predicates, we need to 'unwrap' them. This means we recursively peel
// out each variable/predicate on the right into props. i.e. is(john, lovely(man)) [] -> is(john) [lovely, man]
pub fn unwrap(mut p: Relation, f: &mut AgdaFile, props: Vec<Token>) -> (Relation, Vec<Token>) {

    /* Base Case */
    if p.0 != "is" || p.1.len() <= 1 { return (p, props.clone()) }

    println!("relation: {:?}", p);

    /* Recursive Case: Get the right-hand side variable/predicate */
    let mut final_idx = p.1.len() - 1;
    let mut lastarg = p.1.last().clone().expect("Expected arguments in relation.");
    match *lastarg.clone() {

        /* If it's a predicate, replace with its first variable and move p to props */
        SemanticTree::NonTerminal(mut relation) => {
            let inner_arg = relation.1.pop().unwrap();
            let mut new_props = props.clone();
            new_props.push(relation.0);
            p.1[final_idx] = inner_arg;
            return unwrap(p.clone(), f, new_props)
        }
        /* If it's a variable, then pop it from the args, and add to props */
        SemanticTree::Terminal(token) => {
            let mut new_props = props.clone();
            new_props.push(token);
            let _ = p.1.pop();
            return unwrap(p.clone(), f, new_props)
        }
        _ => { panic!("There is an `is` predicate that contains something that isn't pred/var.")}
    }
}


type QVec = Vec<(Token, Box<SemanticTree>)>;

/// Convert to Prenex Normal Form (i.e. P(T1, T2) -> {(a,T1), (b,T2)} P(a, b) .
pub fn prenex(p: &mut Relation, equants: &mut QVec, uquants: &mut QVec) -> () {
    /* Factor out UQuantifiers into uquants & EQuantifiers into equants */
    for i in 0..p.clone().1.len() {
        let mut arg = p.1.get(i).unwrap();
        match contains_uquant(arg.clone().into()) {
            true => {
                /* It's a universal quantifier node! Move into uquants and replace with `a` */
                /* This is usually in the form every(P(x)) -> P(x)                          */
                let mut internal = match *arg.clone() {
                    SemanticTree::NonTerminal(relation) => p.1.get(0).unwrap().clone(),
                    _ => { panic!("Universal Quantification can't unwrap the every.") }
                };

                uquants.push((
                    format!("a{}", to_unicode_subscript(uquants.len() + 1)),
                    Box::new(*internal.clone())));

                p.1[i] = tToken!(format!("a{}", to_unicode_subscript(uquants.len() + 0)));
            }
            false => {
                /* It's a existential quantifier node! Move into equants and replace with `e` */
                equants.push((
                    format!("e{}", to_unicode_subscript(equants.len() + 1)),
                    Box::new(*arg.clone())));

                p.1[i] = tToken!(format!("e{}", to_unicode_subscript(equants.len() + 0)));
            }
        }
    }
}


pub fn generate_predicate_output(mut returned_proofs: Vec<Box<AgdaType>>) -> Box<AgdaType> {
    if returned_proofs.len() == 0 { panic!("Something has gone wrong!") }
    if returned_proofs.len() == 1 { returned_proofs.pop().unwrap() }
    else {
        /* Construct the return type as a product of the returned proofs */
        returned_proofs.into_iter().rev().fold(None, |acc, proof| {
            match acc {
                None => Some(proof),
                Some(prod) => Some(τProduct!(proof, prod))
            }
        }).unwrap()
    }
}


pub fn compose_predicate(relation: Relation, f: &mut AgdaFile, props: Vec<Token>) -> (String, AgdaType) {

    let mut is_negated: i32 = 0;


    /* Handle 'is' cases using unwrapping. */
    let (mut p, props) = unwrap(relation, f, props.clone());


    /* Prenex Normal Transformation (derive quantifiers and bind anaphora) */
    let (mut uquants, mut equants): (QVec, QVec) = (vec![], vec![]);
    prenex(&mut p, &mut equants, &mut uquants);


    /* Admin (boring) */
    let mut iden = format!("{}", p.0);
    let mut record_name = format!("{}ᵣ", convert_case(&*iden, CaseStyle::PascalCase));
    let mut constructor_name = format!("{}꜀", convert_case(&*iden, CaseStyle::PascalCase));
    let mut symbol_table: HashMap<String, (String, AgdaType)> = HashMap::new();
    let mut fields: Vec<RecordField> = vec![];


    /* For each existential quantifier, it needs to be added as an entity (field)
     * in the record. i.e. John likes Cheese -> e1: John, e1 Cheese
     */
    for (identifier, _type) in equants.clone() {
        let pair = compose(_type.clone(), f, vec![]);
        symbol_table.insert(identifier.clone(), pair.clone());
        fields.push(RecordField(identifier.to_string(), Simple(pair.0.clone())));
    }


    /* For universally quantified references, there isn't a need for this. They are
     * composed and added to the symbol table.
     */
    for (identifier, _type) in uquants.clone() {
        let pair = compose(_type, f, vec![]);
        symbol_table.insert(identifier, pair);
    }


    /* Verify there are no unbound references in the predicate arguments. */
    let mut var_idens: Vec<String> = p.clone().1.iter().map(
        |arg|  { match *arg.clone() {
            SemanticTree::Terminal(token) => {token}
            _ => { panic!("Predicate still contains non-bound argument.")}
        }}).collect();


    /* Append record fields to the name and constructor name of the record. */
    record_name.extend(var_idens.iter().map(|v| {
        format!("_{}", symbol_table.get(v).unwrap().0.clone()) }));
    constructor_name.extend(var_idens.iter().map(|v| {
        format!("_{}", symbol_table.get(v).unwrap().0.clone()) }));

    let mut inner = τSimp!("Temporary".parse().unwrap());

    /* If there are no Universal Quantifiers, we compose is as a variable using props.
     * This handles cases such as `x is a adj noun`, `x is adj`, `x is noun`.
     */
    if p.0 == "is" && uquants.is_empty() {
        match *p.1.get(0).unwrap().clone() {
            Terminal(v) => {
                let v_name = symbol_table.get(v.as_str()).unwrap().clone().0.replace('ᵣ', "");
                return compose_variable(v_name, f, props)
            }
            _ => { panic!("Invalid!") }
        }
    }

    /* Handle `is` cases with some Universal Quantification on the left.
     * This is handled as a Pi Type.
     */
    else if(p.0 == "is") {

        /* Append record fields to the name and constructor name of the record. */
        /* Admin */
        record_name.extend(props.iter().map(|v| { format!("_{}", v) }));
        constructor_name.extend(props.iter().map(|v| { format!("_{}", v) }));

        /* In a sentence of `every _ is _` there will be at most 1 every - or it's nonsensical. */
        /* This uquant can also be a compound type i.e. `man` so we get the rec proj. function  */
        let (uquant_iden, _) = uquants.get(0).unwrap();
        let uquant_projection_function = symbol_table.get(uquant_iden).unwrap().clone().1;

        let mut returned_proofs: Vec<Box<AgdaType>> = vec![];
        for current_prop in props.clone() {

            /* If this `prop` is a not, then we should increase negation by 1 layer. */
            if current_prop == "not" {
                is_negated = is_negated + 1;
                continue;
            }

            /* The right-hand side property gets converted to `is_property` notation, then postulated as a
             * function over entities (to Set). Since this in the `every` branch, the dependent function returns
             * a proof of this property for the given entity.
             */
            let mut rhs_property = convert_case(format!("is_{}", current_prop).as_str(), CaseStyle::CamelCase);
            f.insert_postulate(PostulateEntry(rhs_property.clone(), generate_function_header(1)));


            /* This constructs the `property` returned in the dependent function. If the current property `current_prop`
             * is `isHappy` this applies `isHappy` to the record projection function of the universally quantified anaphora,
             * e.g. if the anaphora is John then this would generate `isHappy (John.e₁ a₁)`.
             * (The Function `replace_innermost_simple` handles the conversion of e₁ to a₁ here.)
             */
            returned_proofs.push(τApp!(τSimp!(rhs_property.clone()),
                        Box::from(replace_innermost_simple(uquant_projection_function.clone(),
                        Simple(uquant_iden.clone())))
            ));
        }

        inner = generate_predicate_output(returned_proofs);

        /* Handle layers of negation e.g. `not not P` / `not P` */
        for _ in (0..is_negated) { inner = τFunc!(inner, τSimp!("⊥".to_string())); }
    }

    /* Handles normal predicates */
    else {
        /* Postulate the predicate as a function to Set. */
        f.insert_postulate(PostulateEntry(iden.clone(), generate_function_header(p.1.len())));

        /* Then, `inner` becomes the application of that function to the arguments */
        inner = var_idens.iter().fold(
            τSimp!(iden.clone()),
            |acc, name| {
                let proj = symbol_table.get(name).unwrap().clone().1;
                let app_proj = replace_innermost_simple(proj, *τSimp!(name.clone()));
                τApp!(acc, Box::from(app_proj))
            }
        );
    }

    /* If there are universal quantifiers, we need to bind these outside using Π-types. This
     * is accomplished by folding `uquants` into `inner` e.g. (a₁ : T₁) -> inner ...
     */
    inner = uquants.into_iter().rev()
        .fold(inner, |acc, (current, typ)| {
            let rec_name = symbol_table.get(&current).unwrap().0.clone();
            τDepFunc!(current, τSimp!(rec_name), acc)
        });

    /* Store this in the record under `p` */
    fields.push(RecordField("p".to_string(), *inner));


    /* Format record and constructor names correctly. */
    record_name = format!("{}ᵣ", convert_case(&*record_name.replace('ᵣ', ""), CaseStyle::PascalCase));
    constructor_name = format!("{}꜀", convert_case(&*constructor_name.replace('꜀', "").replace('ᵣ', ""), CaseStyle::PascalCase));


    /* Make Record */
    let rec = RecordDefinition {
        record_name: record_name.clone(),
        constructor_name: constructor_name,
        fields: fields.clone(),
    };


    /* Insert Definition */
    f.insert_definition(AgdaStructure::RecordDef(rec));


    /* Calculate the Projection Function & Return */
    let projection =
        if fields.len() == 2 {
            let outer_projection = symbol_table.get("e₁").unwrap().clone().1;
            let application = *τApp!( τRecProj!( τSimp!(record_name.clone()) , τSimp!("e₁".to_string()) ), τSimp!("e₁".to_string()));
            replace_innermost_simple(outer_projection, application)
        } else { *τSimp!(record_name.to_string()) };


    (record_name, projection)
}
