use std::collections::HashMap;
use std::ptr::eq;
use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, DefinitionInserter, PostulateInserter};
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::ast::application::Application;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::{app, term};
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::agda_expr::AgdaExpr::FunType;
use crate::ast::function_type::FunctionType;
use crate::ast::program::Program;
use crate::ast::record_decl::Record;
use crate::ast::top_decl::TDeclaration::RecordDecl;
use crate::ast::var_declaration::VarDecl;
use crate::composer::case_converter::*;
use crate::composer::compose_predicate::compose_predicate;
use crate::composer::compose_variable::compose_variable;
use crate::composer::langtree::{Join, SemanticTree, Token};

pub fn generate_function_header(arity: usize) -> AgdaExpr {
    if arity == 0 {
        *term!("Set")
    } else {
        FunType(FunctionType{
            lhs: term!("Entity"),
            rhs: Box::new(generate_function_header(arity - 1))
        })
    }
}

pub fn replace_innermost_simple(expr: &AgdaExpr, new_value: AgdaExpr) -> AgdaExpr {
    match expr {
        AgdaExpr::App(app) => {
            let new_rhs = replace_innermost_simple(&app.rhs, new_value);
            app!((*app.lhs).clone(), new_rhs)
        },
        _ => new_value,
    }
}

pub fn compose_product(c: Join, f: &mut Program) -> (String, AgdaExpr) {

/* Extract projections */
let proj1 = c.0;
let proj2 = c.1;

let proj1_iden = compose(proj1, f, vec![]);
let proj2_iden = compose(proj2, f, vec![]);

/* These sometimes have record identifiers in them ᵣ, remove! */
let iden: String = format!("{}×{}", proj1_iden.0, proj2_iden.0)
    .chars()
    .filter(|&c| c != 'ᵣ')
    .collect();

/* Generate Fields */
let mut fields: Vec<VarDecl> = vec![
    VarDecl{
        iden: "e₁".to_string(),
        _type: term!(proj1_iden.0)},
    VarDecl{
        iden: "e₂".to_string(),
        _type: term!(proj2_iden.0)},
];

/* Now, we need to insert the record for it */
let record_name = format!("{}ᵣ", convert_case(&*iden, CaseStyle::PascalCase));
let constructor_name = format!("{}꜀", convert_case(&*iden, CaseStyle::PascalCase));

let rec = Record {
    record_iden: record_name.clone(),
    constructor_iden: constructor_name,
    fields: fields,
    comment: None
};

f.insert_definition(RecordDecl(rec));
(record_name, *term!("Temporary"))
}


pub fn compose(e: Box<SemanticTree>, f: &mut Program, props: Vec<Token>) -> (String, AgdaExpr) {

match *e {
    SemanticTree::NonTerminal(relation) => {compose_predicate(relation, f, props)}
    SemanticTree::Terminal(token) => {compose_variable(token, f, props)}
    SemanticTree::Conj(join) => {compose_product(join, f)}
}

}

