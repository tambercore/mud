use crate::ast::top_decl::TDeclaration;
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::program::{DefinitionInserter, Program};
use crate::ast::record_decl::Record;
use crate::ast::var_declaration::VarDecl;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::top_decl::TDeclaration::{RecordDecl, VariableDecl};
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{record, term, var_decl};
use crate::interpreter::structure::insert_interpretation;

pub type KnowledgeBase = Vec<(String, AgdaExpr)>;

pub fn compose_kb(kb: KnowledgeBase, f: &mut Program) -> TDeclaration {

    /* KB is a vector of records, each representing a premesis in the system */
    let mut assumptions = vec![];
    let mut assumtion_index = 1;

    for (rec_name, rec_type) in kb {

        let var_decl = var_decl!(format!("j{}",
            to_unicode_subscript(assumtion_index)), term!(rec_name));
        assumptions.push(var_decl);
        assumtion_index = assumtion_index + 1;
    }

    let rec = record!("KnowledgeBaseᵣ", "KnowledgeBase꜀", assumptions, None);


    f.insert_definition(rec.clone());
    rec
}