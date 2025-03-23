use std::collections::HashMap;
use std::ptr::eq;
use crate::ccg::rule::CCGRule;
use crate::composer::postulate::{initialise_agda_file, AgdaStructure, DefinitionInserter, PostulateEntry, PostulateInserter};
use crate::composer::record::{RecordDefinition, RecordField};
use crate::composer::structures::{AgdaType};
use crate::composer::structures::AgdaType::{Application, Simple};
use crate::lambda::predicate::Predicate;
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::{term};
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


pub fn replace_innermost_simple(expr: AgdaType, new_value: AgdaType) -> AgdaType {
    match expr {

        // If the current expression is an App, recursively replace in the nested expression
        Application(lhs, rhs) => {
            let new_rhs = replace_innermost_simple(*rhs.clone(), new_value.clone());
            // let new_lhs = replace_innermost_simple(*lhs, new_value.clone());

            // Continue with recursive replacement on the right side of the app chain
            if let Application(_, _) = *rhs {
                Application(lhs.clone(), Box::new(new_rhs))
            } else {
                Application(lhs.clone(), Box::new(new_rhs))
            }
        }
        // If the current expression is a Simple, replace it with the new_value
        _ => new_value,
    }
}

pub fn compose_product(c: Join, f: &mut Program) -> (String, AgdaType) {

    /* Extract projections */
    let proj1 = c.0;
    let proj2 = c.1;

    let proj1_iden = compose(proj1, f, vec![]);
    let proj2_iden = compose(proj2, f, vec![]);

    use AgdaType::*;

    /* These sometimes have record identifiers in them ᵣ, remove! */
    let iden: String = format!("{}×{}", proj1_iden.clone().0, proj2_iden.clone().0)
        .chars()
        .filter(|&c| c != 'ᵣ')
        .collect();

    /* Generate Fields */
    let mut fields: Vec<VarDecl> = vec![
        VarDecl{
            iden: "e₁".to_string(),
            _type: term!(proj1_iden.clone().0)},
        VarDecl{
            iden: "e₂".to_string(),
            _type: term!(proj2_iden.clone().0)},
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
    (record_name, term!("Temporary"))
}


pub fn compose(e: Box<SemanticTree>, f: &mut Program, props: Vec<Token>) -> (String, AgdaType) {

    match *e {
        SemanticTree::NonTerminal(relation) => {compose_predicate(relation, f, props)}
        SemanticTree::Terminal(token) => {compose_variable(token, f, props)}
        SemanticTree::Conj(join) => {compose_product(join, f)}
    }

}

