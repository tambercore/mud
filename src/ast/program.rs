use crate::ast::theorem_decl::Theorem;
use crate::ast::quantification::Quantification;
use crate::ast::unary_op::UnOperator;
use crate::ast::function_type::FunctionType;
use crate::ast::infix_decl::Infix;
use crate::ast::import_decl::Import;
use crate::ast::abstraction::Abstraction;
use crate::ast::application::Application;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::ast::postulate_decl::Postulate;
use crate::AgdaExpr::Term;
use crate::ast::top_decl::TDeclaration;
use crate::ast::top_decl::TDeclaration::{CommentSegment, ImportDecl, InfixDecl, PostulateDecl, RecordDecl, TheoremDecl, VariableDecl};
use crate::ast::var_declaration::VarDecl;
use crate::{abstraction, app, bin_op, function_type, import, infix, postulate, quant, term, theorem, unop, var_decl};
use crate::ast::agda_expr::{format_agda_type, AgdaExpr};
use crate::ast::binary_op::BinOperator;
use crate::ast::operator::Operator::{Necessity, Possibility, PropEq};
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
    fn insert_postulate(&mut self, entry: TDeclaration);
}

pub trait DefinitionInserter {
    fn insert_definition(&mut self, entry: TDeclaration);
}





/* Implement the trait for AgdaFile */
impl PostulateInserter for Program {
    fn insert_postulate(&mut self, entry: TDeclaration) {
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

    f.create_prelude();

    f
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


    pub fn create_prelude(&mut self) {

        /* Handle imports */
        self.insert_definition(import!("Data.Product", vec![]));
        self.insert_definition(import!("Relation.Binary.PropositionalEquality", vec!["_≡_".to_string(), "refl".to_string(), "subst".to_string(), "sym".to_string(), "cong".to_string()]));

        /* Initialise Postulate */
        self.insert_definition(postulate!(vec![], None));

        /* Define operators */
        self.insert_postulate(CommentSegment("rule in S4 Modal Logic".to_string()));
        self.insert_postulate(infix!(Necessity));
        self.insert_postulate(infix!(Possibility));

        /* Define ◇ as a monad */
        self.insert_postulate(CommentSegment("◇ as a monad".to_string()));

        /* ◇-fmap */
        let possible_fmap_signature = function_type!(function_type!(function_type!(term!("A"), term!("B")), unop!(Possibility, term!("A"))), unop!(Possibility, term!("B")));
        let possible_fmap_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_fmap_signature.clone());
        let possible_fmap_theorem = theorem!("◇-fmap", possible_fmap_signature.clone(), possible_fmap_decl, None);
        self.insert_postulate(possible_fmap_theorem);

        /* ◇-pure */
        let possible_pure_signature = function_type!(term!("A"), unop!(Possibility, term!("A")));
        let possible_pure_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], possible_pure_signature.clone());
        let possible_pure_theorem = theorem!("◇-pure", possible_pure_signature.clone(), possible_pure_decl, None);
        self.insert_postulate(possible_pure_theorem);

        /* ◇-lift */
        let possible_lift_signature = function_type!(unop!(Possibility, function_type!(term!("A"), term!("B"))),function_type!(unop!(Possibility, term!("A")), unop!(Possibility, term!("B"))));
        let possible_lift_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_lift_signature.clone());
        let possible_lift_theorem = theorem!("◇-lift", possible_lift_signature.clone(), possible_lift_decl, None);
        self.insert_postulate(possible_lift_theorem);

        /* ◇-bind */
        let possible_bind_signature = function_type!(function_type!(unop!(Possibility, term!("A")) , function_type!(term!("A"), unop!(Possibility, term!("B")))) ,unop!(Possibility, term!("B")));
        let possible_bind_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], possible_bind_signature.clone());
        let possible_bind_theorem = theorem!("◇-bind", possible_bind_signature.clone(), possible_bind_decl, None);
        self.insert_postulate(possible_bind_theorem);

        /* Define □ as a comonad */
        self.insert_postulate(CommentSegment("□ as a comonad".to_string()));

        /* □-fmap */
        let necessary_fmap_signature = function_type!(function_type!(term!("A"), term!("B")),function_type!(unop!(Necessity, term!("A")), unop!(Necessity, term!("B"))));
        let necessary_fmap_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], necessary_fmap_signature.clone());
        let necessary_fmap_theorem = theorem!("□-fmap", necessary_fmap_signature.clone(), necessary_fmap_decl, None);
        self.insert_postulate(necessary_fmap_theorem);

        /* □-extract */
        let necessary_extract_signature = function_type!(unop!(Necessity, term!("A")),term!("A"));
        let necessary_extract_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], necessary_extract_signature.clone());
        let necessary_extract_theorem = theorem!("□-extract", necessary_extract_signature.clone(), necessary_extract_decl, None);
        self.insert_postulate(necessary_extract_theorem);

        /* □-duplicate */
        let necessary_duplicate_signature = function_type!(unop!(Necessity, term!("A")),unop!(Necessity, unop!(Necessity, term!("A"))));
        let necessary_duplicate_decl = quant!("∀", vec![var_decl!("A", term!("Set"))], necessary_duplicate_signature.clone());
        let necessary_duplicate_theorem = theorem!("□-duplicate", necessary_duplicate_signature.clone(), necessary_duplicate_decl, None);
        self.insert_postulate(necessary_duplicate_theorem);

        /* □-cobind */
        let necessary_cobind_signature = function_type!(function_type!(unop!(Necessity, term!("B")),function_type!(unop!(Necessity, term!("B")), term!("A"))),unop!(Necessity, term!("A")));
        let necessary_cobind_decl = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], necessary_cobind_signature.clone());
        let necessary_cobind_theorem = theorem!("□-cobind", necessary_cobind_signature.clone(), necessary_cobind_decl, None);
        self.insert_postulate(necessary_cobind_theorem);

        self.insert_postulate(CommentSegment("Define the S4 Modal Logic".to_string()));

        /* □-k */
        let necessary_k_hypothesis = quant!("∀", vec![var_decl!("A", term!("Set")), var_decl!("B", term!("Set"))], function_type!(unop!(Necessity, function_type!(term!("A"), term!("B"))), function_type!(unop!(Necessity, term!("A")), unop!(Necessity, term!("B")))));
        let necessary_k_proof = abstraction!("z",abstraction!("z₁",app!(app!(term!("□-fmap"),abstraction!("z₂",app!(term!("z₂"), app!(term!("□-extract"), term!("z₁"))))), term!("z")))) ;
        let necessary_k_decl = theorem!("□-k", necessary_k_hypothesis, necessary_k_proof, None);
        self.insert_postulate(necessary_k_decl);

        /* □-t */
        let necessary_t_hypothesis = quant!("∀",vec![var_decl!("A", term!("Set"))],function_type!(unop!(Necessity, term!("A")), term!("A")));
        let necessary_t_proof = term!("□-extract");
        let necessary_t_decl = theorem!("□-t", necessary_t_hypothesis, necessary_t_proof, None);
        self.insert_postulate(necessary_t_decl);

        /* □-4 */
        let necessary_4_hypothesis = quant!("∀",vec![var_decl!("A", term!("Set"))],function_type!(unop!(Necessity, term!("A")), unop!(Necessity, unop!(Necessity, term!("A")))));
        let necessary_4_proof = term!("□-duplicate");
        let necessary_4_decl = theorem!("□-4", necessary_4_hypothesis, necessary_4_proof, None);
        self.insert_postulate(necessary_4_decl);

        /* □-d */
        let necessary_d_hypothesis = quant!("∀",vec![var_decl!("A", term!("Set"))],function_type!(unop!(Necessity, term!("A")), unop!(Possibility, term!("A"))));
        let necessary_d_proof = abstraction!("z",app!(term!("◇-pure"), app!(term!("□-extract"), term!("z"))));
        let necessary_d_decl = theorem!("□-d", necessary_d_hypothesis, necessary_d_proof, None);
        self.insert_postulate(necessary_d_decl);

        /* Add Entity : Set to postulate */
        self.insert_postulate(VariableDecl(var_decl!("Entity", term!("Set"))))



    }

    pub fn agdaify(&self) -> String {
        let mut code = String::new();

        // push postulate, then everything else

        let mut postulates = self.get_postulates();
        for postulate in postulates {


            let mut propeqs = Vec::new();
            let mut regular_postulates = Vec::new();

            // Manually partition without using `.iter()`
            for entry in &postulate.fields {
                if let VariableDecl(var) = entry {
                    if let AgdaExpr::BinOp(BinOperator { symbol: ref symb, lhs: _, rhs: _ }) = &*var._type {
                        if *symb == PropEq {
                            propeqs.push(entry);
                        }
                    }
                }

                else {
                    regular_postulates.push(entry);
                }
            }

            // Process regular postulates
            for entry in regular_postulates {
                match entry {
                    ImportDecl(import) => code.push_str( &format!("\n{}\n", import.agdaify())),
                    InfixDecl(infix) => code.push_str( &format!("\n{}\n", infix.agdaify())),
                    TheoremDecl(theorem) => code.push_str( &format!("\n{}\n", theorem.agdaify())),
                    VariableDecl(var) => code.push_str( &format!("\n{}\n", var.agdaify())),
                    CommentSegment(comment) => code.push_str( &format!("-- \n{}\n", comment)),
                    _ => panic!("Unexpected entry in postulate.")
                };
            }

            // Process propositional equalities separately
            for entry in propeqs {
                match entry {
                    VariableDecl(var) => code.push_str( &format!("\n{}\n", var.agdaify())),
                    _ => panic!("Unexpected entry in propositional equalities postulate.")
                };
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
