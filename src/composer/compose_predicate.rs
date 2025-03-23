use crate::ast::record_projection::RecordProjection;
use crate::ast::dependent_function::DependentFunction;
use crate::ast::function_type::FunctionType;
use crate::AgdaType::Simple;
use crate::ast::binary_op::BinOperator;
use std::collections::HashMap;
use crate::composer::postulate::{DefinitionInserter, PostulateInserter};
use crate::composer::structures::{AgdaType};
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{app, bin_op, dependent_function, function_type, record_projection, tToken, term, var_decl, τDepFunc, WORDS_IN_EXISTENCE};
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::agda_expr::AgdaExpr::{BinOp, UnOp};
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::application::TApplication;
use crate::ast::operator::Operator::{Necessity, Product};
use crate::ast::program::Program;
use crate::ast::record_decl::Record;
use crate::ast::top_decl::TDeclaration::RecordDecl;
use crate::ast::unary_op::UnOperator;
use crate::ast::var_declaration::VarDecl;
use crate::brill::utils::TAG_MAPPING;
use crate::brill::wordclass::Wordclass;
use crate::composer::ast_format::format_agda_ast;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::types::LambdaEntity::{App, Var};
use crate::composer::case_converter::*;
use crate::composer::compose_variable::compose_variable;
use crate::composer::lambda_to_types::{compose, generate_function_header, replace_innermost_simple};
use crate::composer::langtree::{Relation, SemanticTree, Token};
use crate::composer::langtree::SemanticTree::Terminal;
use crate::wordnet::interface::{get_meanings, init_wordnet};
use crate::wordnet::wordnode::Wordnode;
use crate::composer::ast::*;
use crate::composer::synonym_handler::handle_synonyms;



pub fn add_describer(current_prop: Token, f: &mut Program) {

    /* Add the describer as an Adjective */
    let mut property = convert_case(format!("is_{}", current_prop).as_str(), CaseStyle::CamelCase);
    let entry = var_decl!(property.clone(), generate_function_header(1));
    f.insert_postulate(*entry);
    handle_synonyms(current_prop.as_str(), f);
}



pub fn contains_uquant(l: Box<SemanticTree>) -> bool {
    match *l {
        SemanticTree::NonTerminal(relation) => {
            if relation.0 == "every" {
                true }
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
pub fn unwrap(mut p: Relation, f: &mut Program, props: Vec<Token>) -> (Relation, Vec<Token>) {

    /* Base Case */
    if p.0 != "is" || p.1.len() <= 1 { return (p, props.clone()) }

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
                    SemanticTree::NonTerminal(relation) => relation.1.get(0).unwrap().clone(),
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


pub fn generate_predicate_output(mut returned_proofs: Vec<Box<AgdaExpr>>) -> Box<AgdaExpr> {
    if returned_proofs.len() == 0 { panic!("Something has gone wrong!") }
    if returned_proofs.len() == 1 { returned_proofs.pop().unwrap() }
    else {
        /* Construct the return type as a product of the returned proofs */
        returned_proofs.into_iter().rev().fold(None, |acc, proof| {
            match acc {
                None => Some(proof),
                Some(prod) => Some(Box::from(BinOp(bin_op!(*prod, *proof, Product))))
            }
        }).unwrap()
    }
}

pub fn handle_modal_necessity(rel: Relation, f: &mut Program, props: Vec<Token>) -> (String, AgdaExpr) {

    let mut relation = rel.clone();
    if relation.1.len() != 1 { panic!("`Necessity with more than one arg`") }

    let arg = relation.1.pop().unwrap();
    let (prop_rec_name, prop_projection) = compose(arg, f, props);

    let mut iden = format!("{}_{}", rel.0, prop_rec_name.replace("ᵣ", ""));
    let mut record_name = format!("{}ᵣ", convert_case(&*iden, CaseStyle::PascalCase));
    let mut constructor_name = format!("{}꜀", convert_case(&*iden, CaseStyle::PascalCase));

    let operator = UnOperator { op: Necessity,
        expr: term!(prop_rec_name.clone())
    };
    let mut fields: Vec<VarDecl> = vec![
        VarDecl {
            iden: String::from("I"),
            _type: Box::from(UnOp(operator)),
        }
        /* RecordField(String::from("I"),
                    *τModalNecessity!(τSimp!(String::from(prop_rec_name)))
        )*/
    ];

    let proj_func = replace_innermost_simple(&prop_projection, AgdaExpr::App(app!(
        *term!("□-T"),
        *term!("I")
    )));

    let record = Record {
        record_iden: record_name.clone(),
        constructor_iden: constructor_name,
        fields: fields,
        comment: None,
    };

    f.insert_definition(RecordDecl(record));

    (record_name, proj_func)
}

pub fn compose_predicate(relation: Relation, f: &mut Program, props: Vec<Token>) -> (String, AgdaExpr) {

    let mut is_negated: i32 = 0;

    if(vec!["necessarily", "must", "needs", "need"].contains(&&*relation.0.to_lowercase()) && relation.1.len() == 1 ) {
        return handle_modal_necessity(relation, f, props);
    }

    /* Handle 'is' cases using unwrapping. */
    let (mut p, props) = unwrap(relation, f, props.clone());


    /* Prenex Normal Transformation (derive quantifiers and bind anaphora) */
    let (mut uquants, mut equants): (QVec, QVec) = (vec![], vec![]);
    prenex(&mut p, &mut equants, &mut uquants);


    /* Admin (boring) */
    let mut iden = format!("{}", p.0);
    let mut record_name = format!("{}ᵣ", convert_case(&*iden, CaseStyle::PascalCase));
    let mut constructor_name = format!("{}꜀", convert_case(&*iden, CaseStyle::PascalCase));
    let mut symbol_table: HashMap<String, (String, AgdaExpr)> = HashMap::new();
    let mut fields: Vec<VarDecl> = vec![];


    /* For each existential quantifier, it needs to be added as an entity (field)
     * in the record. i.e. John likes Cheese -> e1: John, e1 Cheese
     */
    for (identifier, _type) in equants.clone() {
        let pair = compose(_type.clone(), f, vec![]);
        symbol_table.insert(identifier.clone(), pair);
        let field = VarDecl {
            iden: identifier.to_string(),
            _type: term!(pair.0.clone()),
        };
        fields.push(field);
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

    let mut inner = term!("Temporary");

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

        let mut returned_proofs: Vec<Box<AgdaExpr>> = vec![];
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
            add_describer(current_prop, f);


            /* This constructs the `property` returned in the dependent function. If the current property `current_prop`
             * is `isHappy` this applies `isHappy` to the record projection function of the universally quantified anaphora,
             * e.g. if the anaphora is John then this would generate `isHappy (John.e₁ a₁)`.
             * (The Function `replace_innermost_simple` handles the conversion of e₁ to a₁ here.)
             */

            let app = app!(*term!(rhs_property.clone()),
                               replace_innermost_simple(&uquant_projection_function,
                                                                  *term!(uquant_iden.clone())));

            returned_proofs.push(Box::from(AgdaExpr::App(app)));
        }

        inner = generate_predicate_output(returned_proofs);

        /* Handle layers of negation e.g. `not not P` / `not P` */
        for _ in (0..is_negated) { inner = Box::from(AgdaExpr::FunType(function_type!(*inner, *term!("⊥".to_string())))); }
    }

    /* Handles normal predicates */
    else {
        /* Postulate the predicate as a function to Set. */
        f.insert_postulate(*var_decl!(iden.clone(), generate_function_header(p.1.len())));

        /* Then, `inner` becomes the application of that function to the arguments */
        inner = var_idens.iter().fold(
            term!(iden.clone()),
            |acc, name| {
                let proj = symbol_table.get(name).unwrap().clone().1;
                let app_proj = replace_innermost_simple(&proj, *term!(name.clone()));

                Box::from(AgdaExpr::App(app!(*acc, app_proj)))
            }
        );
    }

    /* If there are universal quantifiers, we need to bind these outside using Π-types. This
     * is accomplished by folding `uquants` into `inner` e.g. (a₁ : T₁) -> inner ...
     */
    inner = uquants.into_iter().rev()
        .fold(inner, |acc, (current, typ)| {
            let rec_name = symbol_table.get(&current).unwrap().0.clone();
            let var_decl = var_decl!(current, term!(rec_name));
            Box::from(AgdaExpr::DepFun(dependent_function!(*var_decl, *acc)))
        });

    /* Store this in the record under `p` */
    let var = VarDecl {
        iden: String::from("p"),
        _type: Box::from(inner),
    };
    fields.push(var);


    /* Format record and constructor names correctly. */
    record_name = format!("{}ᵣ", convert_case(&*record_name.replace('ᵣ', ""), CaseStyle::PascalCase));
    constructor_name = format!("{}꜀", convert_case(&*constructor_name.replace('꜀', "").replace('ᵣ', ""), CaseStyle::PascalCase));


    /* Make Record */
    let record = Record {
        record_iden: record_name.clone(),
        constructor_iden: constructor_name,
        fields: fields,
        comment: None,
    };

    /* Insert Definition */
    f.insert_definition(RecordDecl(record));


    /* Calculate the Projection Function & Return */
    let projection =
        if fields.len() == 2 {
            let outer_projection = symbol_table.get("e₁").unwrap().clone().1;
            let record_proj = record_projection!(*term!(record_name), *term!("e₁"));
            let app = app!(AgdaExpr::RecProj(record_proj), *term!("e₁"));
            replace_innermost_simple(&outer_projection, AgdaExpr::App(app))
        } else { *term!(record_name) };


    (record_name, projection)
}
