use crate::ast::literate_prose::Literate;
use crate::ast::program::{DefinitionInserter, Program};
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::top_decl::TDeclaration;
use crate::ast::agda_expr::AgdaExpr::{FunType, QuestionMark};
use crate::ast::function_type::FunctionType;
use crate::ast::theorem_decl::Theorem;
use crate::ast::top_decl::TDeclaration::{RecordDecl, TheoremDecl};
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{function_type, literate, term, theorem};
use crate::interpreter::interpret::find_record;
use crate::interpreter::interpretation_map::get_interpretation;


/// Composes a list of conclusions into theorems, including their proof structure and associated literate prose.
pub fn compose_conclusions(conclusions: Vec<(String, AgdaExpr)>, f: &mut Program) -> Vec<TDeclaration> {

    let mut conclusion_records = vec![];
    let mut assumtion_index = 1;


    for (idx, (conc_name, conc_type)) in conclusions.iter().enumerate() {

        let iden = format!("thm{}", to_unicode_subscript(assumtion_index));
        let hypothesis = function_type!(term!("KnowledgeBaseᵣ"), term!(conc_name));
        let proof = Box::from(QuestionMark);

        let func = theorem!(iden.clone(), hypothesis, Some(proof), None);
        conclusion_records.push(func.clone());
        assumtion_index = assumtion_index + 1;

        let mut prose_iden = {
            if idx != 0 {
                format!("\\subsection{{Theorem {}: `{}'}}\n\n{}_lp", idx + 1, get_interpretation(&RecordDecl(find_record((*(conc_name.clone())).parse().unwrap(), f).unwrap())).unwrap(), iden)
            } else {
                format!("\\section{{Theorems}}\n\\subsection{{Theorem {}: `{}'}}\n\n{}_lp", idx + 1, get_interpretation(&RecordDecl(find_record((*(conc_name.clone())).parse().unwrap(), f).unwrap())).unwrap(), iden)
            }
        };

        let prose = literate!(prose_iden);
        f.insert_definition(prose);
        f.insert_definition(func);
    }
    conclusion_records
}