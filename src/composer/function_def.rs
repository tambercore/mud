/* use crate::composer::agdaify::format_agda_type;
use crate::composer::ast::AgdaAst;
use crate::composer::ast_format::format_agda_ast;
use crate::composer::structures::AgdaType;


/* Singular Function Definition */
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    pub function_name: String,
    pub function_type: AgdaType,
    pub function_body: AgdaAst,
}

impl FunctionDefinition {
    /// Converts the FunctionDefinition into valid Agda code.
    pub fn agdaify(&self) -> String {
        let mut code = String::new();
        code.push_str(&(format!("{} : {}\n", self.function_name, format_agda_type(&self.function_type))));
        code.push_str(&(format!("{} = {}\n", self.function_name, format_agda_ast(&self.function_body))));
        code
    }
}*/