use crate::composer::case_converter::{convert_case, CaseStyle};
use crate::composer::postulate::{AgdaFile, AgdaStructure, DefinitionInserter, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::AgdaType;
use crate::lambda::variable::Variable;
use crate::{τApp, τFunc, τRecProj, τSimp};
use crate::composer::compose_predicate::generate_predicate_output;
use crate::composer::lambda_to_types::generate_function_header;
use crate::composer::langtree::Token;
use crate::monty::fresh_variable::to_unicode_subscript;

pub fn compose_variable(token: Token, f: &mut AgdaFile, props: Vec<Token>) -> (String, AgdaType) {

    /* Handle negation layers and remove instances of negation from props */
    let negation_layers = props.iter().filter(|p| *p == "not").count() as i32;
    let props: Vec<Token> = props.into_iter().filter(|p| p != "not").collect();

    use crate::composer::structures::AgdaType::*;

    /* Generate Fields */
    let mut predicate_iden = convert_case(format!("is_{}", token).as_str(), CaseStyle::CamelCase);
    let mut fields: Vec<RecordField> = vec![ RecordField("e₁".to_string(), *τSimp!("Entity".to_string()))];
    fields.push(RecordField("p₁".to_string(),
                            *τApp!( τSimp!( predicate_iden.clone() ) , τSimp!("e₁".to_string()) )
    ));

    /* Generate each property as a proof */
    let mut types: Vec<AgdaType> = vec![];
    for p in (props.clone()) {
        let mut c_predicate = convert_case(format!("is_{}", p).as_str(), CaseStyle::CamelCase);
        types.push(*τApp!( τSimp!( c_predicate.clone() ) , τSimp!("e₁".to_string())));
        f.insert_postulate(PostulateEntry(c_predicate, generate_function_header(1)));
    }

    /* Handle cases without negation */
    if negation_layers == 0 {
        let mut counter: usize = 0;
        for _type in types {
            fields.push(RecordField(format!("p{}", to_unicode_subscript(counter)), _type ));
            counter = counter + 1;
        }
    }

    /* Handle cases with negation */
    else {
        let mut inner = generate_predicate_output(types.into_iter().map(|x| {Box::from(x)}).collect());
        for _ in (0..negation_layers) { inner = Box::from(*τFunc!(Box::from(inner), τSimp!("⊥".to_string()))); }
        fields.push(RecordField(format!("p{}", to_unicode_subscript(0)), *inner));
    }

    /* Now, we need to insert the record for it */
    let props_iden = format!("{}{}",
         props.iter().fold(String::new(), |mut acc, p| { acc.push_str(&p); acc.push('_'); acc }),
         token);

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
