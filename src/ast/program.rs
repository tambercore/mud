use crate::ast::theorem_decl::Theorem;
use crate::ast::quantification::Quantification;
use crate::ast::unary_op::UnOperator;
use crate::ast::function_type::FunctionType;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::ast::postulate_decl::Postulate;
use crate::AgdaExpr::Term;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{CommentSegment, PostulateDecl, RecordDecl, TheoremDecl, VariableDecl};
use crate::ast::var_declaration::VarDecl;
use crate::{function_type, postulate, quant, term, theorem, unop, var_decl};
use crate::ast::agda_expr::{format_agda_type, AgdaExpr};
use crate::ast::binary_op::BinOperator;
use crate::ast::operator::Operator::{Necessity, Possibility, PropEq};
use crate::ast::theorem_decl::Agdaify;

/// Type to describe an Agda Program. Consists of a file name (String),
/// and a list of Declarations.
#[derive(Eq, Hash, PartialEq)]
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
    fn insert_postulate(&mut self, entry: TDeclaration);
}



/* Implement the trait for AgdaFile */
impl PostulateInserter for Program {
    fn insert_postulate(&mut self, entry: TDeclaration) {
        for decl in &mut self.declarations.iter_mut().rev() {
            if let PostulateDecl(ref mut p) = decl {
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
    let decl = VariableDecl(var_decl!("Entity", typ));
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

    pub fn create_postulate(&self) -> TDeclaration {
        let mut fields: Vec<TDeclaration> = Vec::new();

        /* Define operators */
        fields.push(CommentSegment("rule in S4 Modal Logic".to_string()));
        fields.push(VariableDecl(var_decl!("□_", function_type!(term!("Set"), term!("Set")))));
        fields.push(VariableDecl(var_decl!("◇_", function_type!(term!("Set"), term!("Set")))));

        /* Define ◇ as a monad */
        fields.push(CommentSegment("◇ as a monad".to_string()));

        /* ◇-fmap */
        let possible_fmap_signature = function_type!(function_type!(function_type!(term!("A"), term!("B")), unop!(Possibility, term!("A"))), unop!(Possibility, term!("B")));
        let possible_fmap_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_fmap_signature.clone());
        let possible_fmap_theorem = theorem!("◇-fmap", possible_fmap_decl.clone(), None, None);
        fields.push(possible_fmap_theorem);

        /* ◇-pure */
        let possible_pure_signature = function_type!(term!("A"), unop!(Possibility, term!("A")));
        let possible_pure_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], possible_pure_signature.clone());
        let possible_pure_theorem = theorem!("◇-pure", possible_pure_decl.clone(), None, None);
        fields.push(possible_pure_theorem);

        /* ◇-lift */
        let possible_lift_signature = function_type!(unop!(Possibility, function_type!(term!("A"), term!("B"))),function_type!(unop!(Possibility, term!("A")), unop!(Possibility, term!("B"))));
        let possible_lift_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_lift_signature.clone());
        let possible_lift_theorem = theorem!("◇-lift", possible_lift_decl.clone(), None, None);
        fields.push(possible_lift_theorem);

        /* ◇-bind */
        let possible_bind_signature = function_type!(function_type!(unop!(Possibility, term!("A")) , function_type!(term!("A"), unop!(Possibility, term!("B")))) ,unop!(Possibility, term!("B")));
        let possible_bind_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_bind_signature.clone());
        let possible_bind_theorem = theorem!("◇-bind", possible_bind_decl.clone(), None, None);
        fields.push(possible_bind_theorem);

        /* Define □ as a comonad */
        fields.push(CommentSegment("□ as a comonad".to_string()));

        /* □-fmap */
        let necessary_fmap_signature = function_type!(function_type!(term!("A"), term!("B")),function_type!(unop!(Necessity, term!("A")), unop!(Necessity, term!("B"))));
        let necessary_fmap_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], necessary_fmap_signature.clone());
        let necessary_fmap_theorem = theorem!("□-fmap", necessary_fmap_decl.clone(), None, None);
        fields.push(necessary_fmap_theorem);

        /* □-extract */
        let necessary_extract_signature = function_type!(unop!(Necessity, term!("A")),term!("A"));
        let necessary_extract_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], necessary_extract_signature.clone());
        let necessary_extract_theorem = theorem!("□-extract", necessary_extract_decl.clone(), None, None);
        fields.push(necessary_extract_theorem);

        /* □-duplicate */
        let necessary_duplicate_signature = function_type!(unop!(Necessity, term!("A")),unop!(Necessity, unop!(Necessity, term!("A"))));
        let necessary_duplicate_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], necessary_duplicate_signature.clone());
        let necessary_duplicate_theorem = theorem!("□-duplicate", necessary_duplicate_decl.clone(), None, None);
        fields.push(necessary_duplicate_theorem);

        /* □-cobind */
        let necessary_cobind_signature = function_type!(function_type!(unop!(Necessity, term!("B")),function_type!(unop!(Necessity, term!("B")), term!("A"))),unop!(Necessity, term!("A")));
        let necessary_cobind_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], necessary_cobind_signature.clone());
        let necessary_cobind_theorem = theorem!("□-cobind", necessary_cobind_decl.clone(), None, None);
        fields.push(necessary_cobind_theorem);


        postulate!(fields, None)
    }
    pub fn agdaify(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!("module {} where\n\n", &self.filepath.replace(".agda", "")));
        code.push_str( &format!("open import Data.Product\n\n"));
        code.push_str(&format!("open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)\n\n"));

        code.push_str("infix 9 □_ \ninfix 10 ◇_ \n\n");

        /* let postulate = "postulate
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
        □-cobind : ∀ {A B : Set} → □ B → (□ B → A) → □ A ";

        code.push_str(postulate);*/

        let postulate = self.create_postulate().agdaify();
        code.push_str(&postulate);

        code.push_str("\n\n -- Derive S4 Modal Logic (as follows)\n");
        code.push_str("□-k : ∀ {A B : Set} → □ (A → B) → (□ A → □ B)\n");
        code.push_str("□-k = λ z z₁ → □-fmap (λ z₂ → z₂ (□-extract z₁)) z\n");

        code.push_str("□-t : ∀ {A : Set} → □ A → A\n");
        code.push_str("□-t = □-extract\n");

        code.push_str("□-4 : ∀ {A : Set} → □ A → □ □ A\n");
        code.push_str("□-4 = □-duplicate\n");

        code.push_str("-- □-d says that if □ A then it is possible that A\n");
        code.push_str("□-d : ∀ {A : Set} → □ A → ◇ A\n");
        code.push_str("□-d = λ z → ◇-pure (□-extract z)\n");

        code.push_str("\n\n-- Now, introduce the relevant language constructions\npostulate\n");

        // push postulate, then everything else

        let mut postulates = self.get_postulates();
        for postulate in postulates {
            let mut propeqs = Vec::new();
            let mut regular_postulates = Vec::new();

            // Manually partition without using `.iter()`
            for entry in &postulate.fields {
                if let VariableDecl(vardecl) = entry {
                    if let AgdaExpr::BinOp(BinOperator { symbol: ref symb, lhs: _, rhs: _ }) = &*vardecl._type {
                        if *symb == PropEq {
                            propeqs.push(entry);
                        }
                    }
                    else {
                        regular_postulates.push(entry);
                    }
                }
                else {
                    regular_postulates.push(entry);
                }
            }

            // Process regular postulates
            for entry in regular_postulates {
                code.push_str( &format!("    {}", entry.agdaify()));
            }

            // Process propositional equalities separately
            for entry in propeqs {
                code.push_str( &format!("    {}", entry.agdaify()));
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
