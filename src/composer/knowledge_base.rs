use crate::ast::agda_expr::AgdaExpr;
use crate::ast::program::{DefinitionInserter, Program};
use crate::ast::record_decl::Record;
use crate::ast::var_declaration::VarDecl;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::top_decl::TDeclaration::RecordDecl;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::term;

pub type KnowledgeBase = Vec<(String, AgdaExpr)>;

pub fn compose_kb(kb: KnowledgeBase, f: &mut Program) -> (String, AgdaExpr) {

    /* KB is a vector of records, each representing a premesis in the system */
    let mut assumptions: Vec<VarDecl> = vec![];
    let mut assumtion_index = 1;

    for (rec_name, rec_type) in kb {
        assumptions.push(VarDecl {
            iden: format!("j{}",
            to_unicode_subscript(assumtion_index)),
            _type: term!(rec_name)
        });
        assumtion_index = assumtion_index + 1;
    }

    let rec = Record {
        record_iden: "KnowledgeBaseᵣ".parse().unwrap(),
        constructor_iden: "KnowledgeBase꜀".parse().unwrap(),
        fields: assumptions,
        comment: None
    };

    f.insert_definition(RecordDecl(rec));
    ("KnowledgeBaseᵣ".parse().unwrap(), *term!("KnowledgeBaseᵣ"))
}