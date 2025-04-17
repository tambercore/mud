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
use crate::ast::top_decl::TDeclaration::{CommentSegment, LiterateProse, PostulateDecl, RecordDecl, TheoremDecl, VariableDecl};
use crate::ast::var_declaration::VarDecl;
use crate::{abstraction, app, function_type, postulate, quant, term, theorem, unop, var_decl};
use crate::ast::agda_expr::{format_agda_type, AgdaExpr};
use crate::ast::binary_op::BinOperator;
use crate::ast::operator::Operator::{Necessity, Possibility, PropEq};
use crate::ast::theorem_decl::Agdaify;
use crate::interpreter::interpretation_map::insert_interpretation;
use crate::ast::abstraction::Abstraction;
use crate::ast::application::Application;

/// A struct representing an Agda program, consisting of a file path and a list of declarations.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Program {pub filepath : String, pub declarations : Vec<TDeclaration>}



#[macro_export]
/// Macro to generate a new `Program` with the specified filepath and declarations.
macro_rules! program {
    ($filepath:expr, $decl:expr) => {
        Program {
            filepath: $filepath.to_string(),
            declarations: vec![($decl)]
        }
    }
}



/// Trait to insert a postulate entry into an Agda program.
pub trait PostulateInserter {
    fn insert_postulate(&mut self, entry: TDeclaration);
}



/// Implementation of `PostulateInserter` for the `Program` struct.
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



/// Initializes an Agda file, creating an empty `Program` with a sample `Entity` declaration.
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



/// Trait for inserting definitions and modal theorems into an Agda program.
pub trait DefinitionInserter {
    fn insert_definition(&mut self, entry: TDeclaration);
    fn insert_modal_theorem(&mut self, entry: TDeclaration);
}




/// Implementation of `DefinitionInserter` for the `Program` struct.
impl DefinitionInserter for Program {
    fn insert_definition(&mut self, entry: TDeclaration) {
        if !self.declarations.contains(&entry) {
            self.declarations.push(entry);
        }
    }

    fn insert_modal_theorem(&mut self, entry: TDeclaration) {
        // First, find the index of the first postulate
        if let Some(idx) = self.declarations.iter().position(|decl| matches!(decl, PostulateDecl(_))) {
            // Then insert after it
            self.declarations.insert(idx + 1, entry);
        }
    }
}

/* Converts the AgdaFile's postulate entries into Agda code. */
impl Program {



    /// Converts the postulate declarations into Agda code.
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
                LiterateProse(inner) => Some(TDeclaration::LiterateProse(inner)),
                _ => None,
            })
            .collect()
    }



    /// Creates and returns a postulate with modal logic rules (e.g., S4 Modal Logic).
    pub fn create_postulate(&mut self) -> TDeclaration {
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
        fields.push(possible_fmap_theorem.clone());
        insert_interpretation(possible_fmap_theorem, format!("If one truth is possibly linked to another, then their possibility carries over."));


        /* ◇-pure */
        let possible_pure_signature = function_type!(term!("A"), unop!(Possibility, term!("A")));
        let possible_pure_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], possible_pure_signature.clone());
        let possible_pure_theorem = theorem!("◇-pure", possible_pure_decl.clone(), None, None);
        fields.push(possible_pure_theorem.clone());
        insert_interpretation(possible_pure_theorem, format!("If a proposition is true, it is also possible"));

            /* ◇-lift */
        let possible_lift_signature = function_type!(unop!(Possibility, function_type!(term!("A"), term!("B"))),function_type!(unop!(Possibility, term!("A")), unop!(Possibility, term!("B"))));
        let possible_lift_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_lift_signature.clone());
        let possible_lift_theorem = theorem!("◇-lift", possible_lift_decl.clone(), None, None);
        fields.push(possible_lift_theorem.clone());
        insert_interpretation(possible_lift_theorem, format!("If a possible rule exists, applying it to a possible truth gives a possible result."));

        /* ◇-bind */
        let possible_bind_signature = function_type!(function_type!(unop!(Possibility, term!("A")) , function_type!(term!("A"), unop!(Possibility, term!("B")))) ,unop!(Possibility, term!("B")));
        let possible_bind_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_bind_signature.clone());
        let possible_bind_theorem = theorem!("◇-bind", possible_bind_decl.clone(), None, None);
        fields.push(possible_bind_theorem.clone());
        insert_interpretation(possible_bind_theorem, format!("If a truth is possible and always leads to another possible truth, then the second truth is possible."));

        /* Define □ as a comonad */
        fields.push(CommentSegment("□ as a comonad".to_string()));

        /* □-fmap */
        let necessary_fmap_signature = function_type!(function_type!(term!("A"), term!("B")),function_type!(unop!(Necessity, term!("A")), unop!(Necessity, term!("B"))));
        let necessary_fmap_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], necessary_fmap_signature.clone());
        let necessary_fmap_theorem = theorem!("□-fmap", necessary_fmap_decl.clone(), None, None);
        fields.push(necessary_fmap_theorem.clone());
        insert_interpretation(necessary_fmap_theorem, format!("A necessary truth preserves necessity when applied to another truth."));

        /* □-extract */
        let necessary_extract_signature = function_type!(unop!(Necessity, term!("A")),term!("A"));
        let necessary_extract_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], necessary_extract_signature.clone());
        let necessary_extract_theorem = theorem!("□-extract", necessary_extract_decl.clone(), None, None);
        fields.push(necessary_extract_theorem.clone());
        insert_interpretation(necessary_extract_theorem, format!("If a proposition is necessary, it is true"));

        /* □-duplicate */
        let necessary_duplicate_signature = function_type!(unop!(Necessity, term!("A")),unop!(Necessity, unop!(Necessity, term!("A"))));
        let necessary_duplicate_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], necessary_duplicate_signature.clone());
        let necessary_duplicate_theorem = theorem!("□-duplicate", necessary_duplicate_decl.clone(), None, None);
        fields.push(necessary_duplicate_theorem.clone());
        insert_interpretation(necessary_duplicate_theorem, format!("If a proposition is necessary, it is necessarily necessary."));

        /* □-cobind */
        let necessary_cobind_signature = function_type!(
            unop!(Necessity, term!("B")),
            function_type!(
                function_type!(unop!(Necessity, term!("B")), term!("A")),
                unop!(Necessity, term!("A"))
            )
        );

        let necessary_cobind_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], necessary_cobind_signature.clone());
        let necessary_cobind_theorem = theorem!("□-cobind", necessary_cobind_decl.clone(), None, None);
        fields.push(necessary_cobind_theorem.clone());
        insert_interpretation(necessary_cobind_theorem, format!("If a truth necessarily depends on a necessary condition, then it is also necessary."));

        postulate!(fields, None)
    }



    /// Add modal logic theorems (e.g., □-k, □-4, □-d)
    fn add_theorems(&mut self, code: &mut String) {
        let necessary_k_signature = function_type!(
            unop!(Necessity, function_type!(term!("A"), term!("B"))),
            function_type!(unop!(Necessity, term!("A")), unop!(Necessity, term!("B"))));

        let necessary_k_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], necessary_k_signature.clone());
        let necessary_k_body = abstraction!("z", abstraction!("z₁", app!(app!(term!("□-fmap"), abstraction!("z₂", app!(term!("z₂"), app!(term!("□-extract"), term!("z₁"))))), term!("z"))));
        let necessary_k_theorem = theorem!("□-k", necessary_k_decl.clone(), Some(Box::from(necessary_k_body)), None);
        insert_interpretation(necessary_k_theorem.clone(), format!("If it is necessary that one proposition follows from another, then if the first proposition is necessary, it follows that the second is necessary."));
        self.insert_modal_theorem(necessary_k_theorem.clone());

        let necessary_t_signature = function_type!(
            unop!(Necessity, term!("A")),
            term!("A")
        );

        let necessary_t_decl = quant!(
            "∀",
            vec![var_decl!("A", term!("Set"))],
            necessary_t_signature.clone()
        );

        let necessary_t_body = term!("□-extract");
        let necessary_t_theorem = theorem!("□-t", necessary_t_decl.clone(), Some(Box::from(necessary_t_body)), None);
        insert_interpretation(necessary_t_theorem.clone(), "If a proposition is necessary, then it is the case.".into());
        self.insert_modal_theorem(necessary_t_theorem.clone());

        let box_four_signature = function_type!(
            unop!(Necessity, term!("A")),
            unop!(Necessity, unop!(Necessity, term!("A")))
        );
        let box_four_decl = quant!(
            "∀",
            vec![var_decl!("A", term!("Set"))],
            box_four_signature.clone()
        );
        let box_four_body = term!("□-duplicate");
        let box_four_theorem = theorem!("□-4", box_four_decl.clone(), Some(Box::from(box_four_body)), None);
        insert_interpretation(box_four_theorem.clone(), "If a proposition is necessary, then it is necessarily necessary.".into());
        self.insert_modal_theorem(box_four_theorem.clone());

        let box_d_signature = function_type!(
            unop!(Necessity, term!("A")),
            unop!(Possibility, term!("A"))
        );
        let box_d_decl = quant!(
            "∀",
            vec![var_decl!("A", term!("Set"))],
            box_d_signature.clone()
        );
        let box_d_body = abstraction!(
            "z",
            app!(term!("◇-pure"), app!(term!("□-extract"), term!("z")))
        );
        let box_d_theorem = theorem!("□-d", box_d_decl.clone(), Some(Box::from(box_d_body)), None);
        insert_interpretation(box_d_theorem.clone(), "If a proposition is necessary, then it is possible.".into());
        self.insert_modal_theorem(box_d_theorem.clone());
    }



    /// Converts the Agda program into a string of Agda code.
    pub fn agdaify(&mut self) -> String {
        let mut code = String::new();
        code.push_str(&format!("\\begin{{code}}\n\n\n"));
        code.push_str(&*self.declarations.pop().unwrap().agdaify());
        code.push_str(&format!("\n\nmodule {} where\n\n", &self.filepath.replace(".lagda", "")));
        code.push_str( &format!("open import Data.Product\n\n"));
        code.push_str(&format!("open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)\n\n"));

        code.push_str("infix 9 □_ \ninfix 10 ◇_ \n\n");

        let postulate = self.create_postulate().agdaify();
        code.push_str(&postulate);

        self.add_theorems(&mut code);

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
                LiterateProse(literate ) => { code.push_str(&format!("\n{}\n", literate.agdaify())) }
                _ => {}
            }
        }
        code.push_str(&format!("\\end{{code}}"));
        code
    }



    /// Writes the generated Agda code to a file.
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
