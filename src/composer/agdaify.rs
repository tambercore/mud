use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::composer::postulate::{AgdaFile, AgdaStructure, PostulateEntry};
use crate::composer::structures::{AgdaType};



/// Helper function that prints an AgdaType with awareness of operator precedence.
/// Lower numbers indicate looser binding; we wrap in parentheses when the inner
/// expression's binding (my_prec) is less than the context (prec).
fn format_agda_type_prec(agda_type: &AgdaType, prec: u8) -> String {
    match agda_type {
        AgdaType::Simple(s) => s.clone(),

        AgdaType::Function(from, to) => {
            // Function arrow (→) has precedence level 1.
            let my_prec = 1;
            // Use a tighter context for the left-hand side.
            let from_str = format_agda_type_prec(from, my_prec + 1);
            let to_str = format_agda_type_prec(to, my_prec);
            let s = format!("{} → {}", from_str, to_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        AgdaType::Application(func, arg) => {
            // Function application binds tighter than the function arrow.
            let my_prec = 2;
            let func_str = format_agda_type_prec(func, my_prec);
            // The argument is printed in an even tighter context.
            let arg_str = format_agda_type_prec(arg, my_prec + 1);
            let s = format!("{} {}", func_str, arg_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        AgdaType::RecordProj(rec, proj) => {
            // Record projection (.) binds very tightly.
            let my_prec = 3;
            let rec_str = format_agda_type_prec(rec, my_prec);

            // Projection field is usually atomic; use a higher precedence.
            let proj_str = format_agda_type_prec(proj, 4);
            let s = format!("{}.{}", rec_str, proj_str);
            if my_prec < prec { format!("({})", s) } else { s }
        }

        AgdaType::DepFunc(var, typ, rest) => {
            let rest_str = format_agda_type_prec(rest, prec);
            format!("({} : {}) → {}", var, format_agda_type_prec(typ, prec), rest_str)
        }

        AgdaType::Product(item1, item2) => {
            format!("{} × {}", format_agda_type_prec(item1, prec), format_agda_type_prec(item2, prec))
        }

        AgdaType::PropEq(item1, item2) => {
            format!("{} ≡ {}", format_agda_type_prec(item1, prec), format_agda_type_prec(item2, prec))
        }

        AgdaType::ModalNecessity(prop) => {
            format!("□ {}", format_agda_type_prec(prop, prec))
        }
    }
}



/// The public function that prints an AgdaType.
/// It starts the printing process with a base precedence of 0.
pub fn format_agda_type(agda_type: &AgdaType) -> String {
    format_agda_type_prec(agda_type, 0)
}



/* Converts the AgdaFile's postulate entries into Agda code. */
impl AgdaFile {
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


        let mut postulate = self.postulate.clone();
        let (propeqs, regular_postulates): (Vec<_>, Vec<_>) =
            postulate.into_iter().partition(|entry| matches!(entry.1, AgdaType::PropEq(_, _)));

        for PostulateEntry(name, agda_type) in regular_postulates {
            let typ_str = format_agda_type(&agda_type);
            code.push_str(&format!("  {} : {}\n", name, typ_str));
        }

        // Handle propositional equalities separately afterward
        for PostulateEntry(name, agda_type) in propeqs {
            let typ_str = format_agda_type(&agda_type);
            code.push_str(&format!("  {} : {}\n", name, typ_str));
        }
        
        for def in &self.definitions {
            match def {
                AgdaStructure::RecordDef(rec) => { code.push_str( &format!("\n{}\n", rec.agdaify())) }
                AgdaStructure::FunctionDef(func) => { code.push_str(&format!("\n{}\n", func.agdaify())) }
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
