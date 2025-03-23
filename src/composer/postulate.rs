
/* Structure for Postulate Entry */
use warp::post;
use crate::ast::agda_expr::AgdaExpr::Term;
use crate::ast::postulate_decl::Postulate;
use crate::ast::program::Program;
use crate::ast::top_decl::TDeclaration::PostulateDecl;
use crate::ast::var_declaration::VarDecl;
use crate::composer::structures::AgdaType;
use crate::{postulate, term, var_decl};
use crate::ast::record_decl::Record;
use crate::ast::top_decl::TDeclaration;

#[derive(Clone, Debug, PartialEq)]
pub struct PostulateEntry(pub String, pub AgdaType);

/* #[derive(Clone, Debug, PartialEq)]
pub enum AgdaStructure {
    RecordDef(RecordDefinition),
    FunctionDef(FunctionDefinition)
}*/


pub fn initialise_agda_file() -> Program {
    let mut f = Program{
        filepath: "test".to_string(),
        declarations: vec!(),
    };

    let typ = term!("Set");
    let decl = var_decl!("Entity", *typ);
    let postulate = postulate!(vec![*decl], None);

    /* Add `Entity : Set` as a declaration */
    f.declarations.push(PostulateDecl(postulate));

    f
}



/* Trait to insert a postulate entry into an AgdaFile */
pub trait PostulateInserter {
    fn insert_postulate(&mut self, entry: VarDecl);
}



/* Implement the trait for AgdaFile */
impl PostulateInserter for Program {
    fn insert_postulate(&mut self, entry: VarDecl) {
        for decl in &mut self.declarations {
            if let PostulateDecl(p) = decl {
                if !p.fields.contains(&entry) {
                    p.fields.push(entry);
                }
                return
            }
        }
    }
}



pub trait DefinitionInserter {
    fn insert_definition(&mut self, entry: TDeclaration);
}

impl DefinitionInserter for Program {
    fn insert_definition(&mut self, entry: TDeclaration) {
        if !self.declarations.contains(&entry) {
            self.declarations.push(entry);
        }
    }
}