use crate::type_theory::utility::Agdaify;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType {
    pub params: Vec<String>, // List of argument types (e.g., ["Nat", "Nat"])
    pub return_type: String,  // Return type (e.g., "Bool")
}

impl Agdaify for FunctionType {
    fn to_agda(&self) -> String {
        let param_str = self.params.join(" → "); // Join parameter types with "→"
        format!("{} → {}", param_str, self.return_type)
    }
}