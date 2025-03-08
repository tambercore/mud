use crate::composer::function_def::FunctionDefinition;
use crate::composer::postulate::{AgdaFile, AgdaStructure, DefinitionInserter};
use crate::composer::structures::AgdaType;
use crate::composer::structures::AgdaType::Simple;
use crate::monty::fresh_variable::to_unicode_subscript;

pub fn compose_conclusions(conclusions: Vec<(String, AgdaType)>, f: &mut AgdaFile) -> () {

    let mut assumtion_index = 1;
    for (conc_name, conc_type) in conclusions {
        let func = FunctionDefinition{
            function_name: format!("thm{}", to_unicode_subscript(assumtion_index)),
            function_type: AgdaType::Function(Box::from(Simple("KnowledgeBaseᵣ".parse().unwrap())), Box::from(Simple(conc_name))),
        };
        assumtion_index = assumtion_index + 1;
        f.insert_definition(AgdaStructure::FunctionDef(func));
    }
}