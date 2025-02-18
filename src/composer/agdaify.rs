use crate::composer::postulate::{AgdaFile, PostulateEntry};
use crate::composer::structures::{AgdaType};



/* Helper function to format AgdaType as an Agda type string. */
fn format_agda_type(agda_type: &AgdaType) -> String {
    match agda_type {
        AgdaType::Simple(s) => s.clone(),
        AgdaType::Function(from, to) => {
            let from_str = format_agda_type(from);
            let to_str = format_agda_type(to);
            format!("{} → {}", from_str, to_str)
        }
    }
}



/* Converts the AgdaFile's postulate entries into Agda code. */
impl AgdaFile {
    pub fn agdaify(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!("module {} where\n\n", &self.filename));
        code.push_str("postulate\n");
        for PostulateEntry(name, agda_type) in &self.postulate {
            let typ_str = format_agda_type(agda_type);
            // Each postulate becomes a line in the Agda output.
            code.push_str(&format!("  {} : {}\n", name, typ_str));
        }
        code
    }
}
