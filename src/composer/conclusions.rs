use crate::ast::literate_prose::Literate;
use crate::ast::program::{DefinitionInserter, Program};
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::top_decl::TDeclaration;
use crate::ast::agda_expr::AgdaExpr::{FunType, QuestionMark};
use crate::ast::function_type::FunctionType;
use crate::ast::theorem_decl::Theorem;
use crate::ast::top_decl::TDeclaration::TheoremDecl;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::monty::fresh_variable::to_unicode_subscript;
use crate::{function_type, literate, term, theorem};

pub fn compose_conclusions(conclusions: Vec<(String, AgdaExpr)>, f: &mut Program) -> Vec<TDeclaration> {

    let mut conclusion_records = vec![];
    let mut assumtion_index = 1;

    let prose_iden = format!("\\section{{Theorems}}");
    let prose = literate!(prose_iden);

    f.insert_definition(prose);

    for (idx, (conc_name, conc_type)) in conclusions.iter().enumerate() {

        let iden = format!("thm{}", to_unicode_subscript(assumtion_index));
        let hypothesis = function_type!(term!("KnowledgeBaseᵣ"), term!(conc_name));
        let proof = Box::from(QuestionMark);

        let func = theorem!(iden.clone(), hypothesis, Some(proof), None);
        conclusion_records.push(func.clone());
        assumtion_index = assumtion_index + 1;

        let prose_iden = format!("\\subsection{{Theorem {}: `{}'}}\n\n{}_lp", idx + 1, conc_name, iden);
        let prose = literate!(prose_iden);

        f.insert_definition(prose);
        f.insert_definition(func);
    }
    conclusion_records
}