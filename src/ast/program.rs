use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::ast::postulate_decl::Postulate;
use crate::AgdaExpr::Term;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{PostulateDecl, RecordDecl, TheoremDecl};
use crate::ast::var_declaration::VarDecl;
use crate::{postulate, term, var_decl};
use crate::ast::agda_expr::{format_agda_type, AgdaExpr};
use crate::ast::binary_op::BinOperator;
use crate::ast::operator::Operator::PropEq;
use crate::ast::theorem_decl::Agdaify;

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

/* Converts the AgdaFile's postulate entries into Agda code. */
impl Program {

    fn get_postulates(&self) -> Vec<Postulate> {
        <Vec<TDeclaration> as Clone>::clone(&self.declarations)
            .into_iter()
            .filter_map(|decl| if let PostulateDecl(inner) = decl { Some(inner) } else { None })
            .collect()
    }


    fn get_definitions(&self) -> Vec<TDeclaration> {
        self.declarations.clone()
            .into_iter()
            .filter_map(|decl| match decl {
                RecordDecl(inner) => Some(TDeclaration::RecordDecl(inner)),
                TheoremDecl(inner) => Some(TDeclaration::TheoremDecl(inner)),
                _ => None,
            }) // Assuming Postulate is (Vec<VarDecl>, Option<String>), cloning may be required
            .collect()
    }
    pub fn agdaify(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!("module {} where\n\n", &self.filepath.replace(".agda", "")));
        code.push_str( &format!("open import Data.Product\n\n"));
        code.push_str(&format!("open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)\n\n"));
        code.push_str("infix 9 □_
infix 10 ◇_

postulate
    -- rule in S4 Modal Logic
    □_   : Set → Set
    ◇_   : Set → Set

    -- ◇ as a monad
    ◇-fmap : ∀ {A B : Set}   → (A → B) → ◇ A → ◇ B
    ◇-pure : ∀ {A : Set}     → A → ◇ A
    ◇-lift : ∀ {A B : Set}   → ◇ (A → B) → ◇ A → ◇ B
    ◇-bind : ∀ {A B : Set}   → (◇ A) → (A → ◇ B) → ◇ B

    -- □ as a comonad
    □-fmap : ∀ {A B : Set} → (A → B) → □ A → □ B
    □-extract : ∀ {A : Set} → □ A → A
    □-duplicate : ∀ {A : Set} → □ A → □ □ A
    □-cobind : ∀ {A B : Set} → □ B → (□ B → A) → □ A

-- Derive S4 Modal Logic (as follows)
□-k : ∀ {A B : Set} → □ (A → B) → (□ A → □ B)
□-k = λ z z₁ → □-fmap (λ z₂ → z₂ (□-extract z₁)) z

□-t : ∀ {A : Set} → □ A → A
□-t = □-extract

□-4 : ∀ {A : Set} → □ A → □ □ A
□-4 = □-duplicate

-- □-d says that if □ A then it is possible that A
□-d : ∀ {A : Set} → □ A → ◇ A
□-d = λ z → ◇-pure (□-extract z)");

        code.push_str("\n\n-- Now, introduce the relevant language constructions\npostulate\n");

        // push postulate, then everything else

        let mut postulates = self.get_postulates();
        for postulate in postulates {
            let mut propeqs = Vec::new();
            let mut regular_postulates = Vec::new();

            // Manually partition without using `.iter()`
            for entry in &postulate.fields {
                if let AgdaExpr::BinOp(BinOperator { symbol: ref symb, lhs: _, rhs: _ }) = &*entry._type {
                    if *symb == PropEq {
                        propeqs.push(entry);
                    }
                } else {
                    regular_postulates.push(entry);
                }
            }

            // Process regular postulates
            for entry in regular_postulates {
                let VarDecl { iden: name, _type: agda_type } = entry;
                let typ_str = format_agda_type(agda_type);
                code.push_str(&format!("  {} : {}\n", name, typ_str));
            }

            // Process propositional equalities separately
            for entry in propeqs {
                let VarDecl { iden: name, _type: agda_type } = entry;
                let typ_str = format_agda_type(&agda_type);
                code.push_str(&format!("  {} : {}\n", name, typ_str));
            }
        }


        for def in &self.get_definitions() {
            match def {
                RecordDecl(rec) => { code.push_str( &format!("\n{}\n", rec.agdaify())) }
                TheoremDecl(func) => { code.push_str(&format!("\n{}\n", func.agdaify())) }
                _ => {}
            }
        }
        code
    }

    pub fn write_to_file(&mut self, filepath: String) -> std::io::Result<()> {
        // Update the internal filename attribute
        self.filepath = filepath.to_string();

        // Generate the Agda file contents
        let agda_code = self.agdaify();

        // Store the formatted filename in a variable to extend its lifetime
        let path = Path::new(&filepath);

        // Create and write to the file
        let mut file = File::create(path)?;
        file.write_all(agda_code.as_bytes())?;

        Ok(())
    }
}
