use crate::ast::top_decl::TDeclaration;
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::program::{DefinitionInserter, Program};
use crate::ast::application::Application;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::function_type::FunctionType;
use crate::ast::record_decl::Record;
use crate::ast::var_declaration::VarDecl;
use crate::{app, function_type, record, term, var_decl};
use crate::ast::top_decl::TDeclaration::VariableDecl;
use crate::composer::case_converter::*;
use crate::composer::compose_predicate::{compose_predicate, to_infix_string};
use crate::composer::compose_variable::compose_variable;
use crate::composer::langtree::{Join, SemanticTree, Token};
use crate::interpreter::interpretation_map::insert_interpretation;


/// Generates a function header for a given arity.
pub fn generate_function_header(arity: usize) -> AgdaExpr {
    if arity == 0 {
        term!("Set")
    } else {
        function_type!(term!("Entity"), generate_function_header(arity - 1))
    }
}



/// Replaces the innermost simple expression in a given Agda expression with a new value.
pub fn replace_innermost_simple(expr: &AgdaExpr, new_value: AgdaExpr) -> AgdaExpr {
    match expr {
        AgdaExpr::App(app) => {
            let new_rhs = replace_innermost_simple(&app.rhs, new_value);
            app!((*app.lhs).clone(), new_rhs)
        },
        _ => new_value,
    }
}



/// Composes a product of two terms (a conjunction of two components) and inserts it into the program.
pub fn compose_product(c: Join, f: &mut Program) -> (String, AgdaExpr) {

    /* Extract projections */
    let proj1 = c.0;
    let proj2 = c.1;

    let proj1_iden = compose(proj1.clone(), f, vec![]);
    let proj2_iden = compose(proj2.clone(), f, vec![]);

    /* These sometimes have record identifiers in them ᵣ, remove! */
    let iden: String = format!("{}×{}", proj1_iden.0, proj2_iden.0)
        .chars()
        .filter(|&c| c != 'ᵣ')
        .collect();

    let lhs = var_decl!("e₁", term!(proj1_iden.0.clone()));
    insert_interpretation(VariableDecl(lhs.clone()), format!("an entity is {}", to_infix_string(*proj1.clone())));

    let rhs = var_decl!("e₂", term!(proj2_iden.0.clone()));
    insert_interpretation(VariableDecl(rhs.clone()), format!("an entity is {}", to_infix_string(*proj2.clone())));

    /* Generate Fields */
    let mut fields = vec![
        lhs,
        rhs
    ];

    /* Now, we need to insert the record for it */
    let record_name = format!("{}ᵣ", convert_case(&*iden, CaseStyle::PascalCase));
    let constructor_name = format!("{}꜀", convert_case(&*iden, CaseStyle::PascalCase));


    let rec = record!(record_name, constructor_name, fields, None);

        let interpretation = format!("{} and {}", to_infix_string(*proj1.clone()), to_infix_string(*proj2.clone()));
        insert_interpretation(rec.clone(), interpretation);

    f.insert_definition(rec);
    (record_name, term!("Temporary"))
    }



/// Composes an Agda expression based on a given semantic tree and inserts it into the program.
///
/// This function processes a semantic tree (`SemanticTree`) by recursively composing the appropriate Agda expressions
/// based on whether the tree contains a terminal (a token), a non-terminal (a relation), or a conjunction (a product
/// of two components). The composed expression is then inserted into the program.
pub fn compose(e: Box<SemanticTree>, f: &mut Program, props: Vec<Token>) -> (String, AgdaExpr) {

    match *e {
        SemanticTree::NonTerminal(relation) => {compose_predicate(relation, f, props)}
        SemanticTree::Terminal(token) => {compose_variable(token, f, props)}
        SemanticTree::Conj(join) => {compose_product(join, f)}
    }

}

