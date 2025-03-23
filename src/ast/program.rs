use crate::ast::postulate_decl::Postulate;
use crate::AgdaExpr::Term;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::PostulateDecl;
use crate::ast::var_declaration::VarDecl;
use crate::{postulate, term, var_decl};

/// Type to describe an Agda Program. Consists of a file name (String),
/// and a list of Declarations.
#[derive(PartialEq)]
pub struct Program {pub filepath : String, pub declarations : Vec<TDeclaration>}

#[macro_export]
macro_rules! program {
    ($filepath:expr, $decl:expr) => {
        Program {
            filepath: $filepath.to_string(),
            declarations: vec![($decl)]
        }
    }
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

pub fn initialise_agda_file() -> Program {
    let mut f = Program{
        filepath: "test".to_string(),
        declarations: vec!(),
    };

    let typ = term!("Set");
    let decl = var_decl!("Entity", *typ);
    let postulate = postulate!(vec![decl], None);

    /* Add `Entity : Set` as a declaration */
    f.declarations.push(postulate);

    f
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