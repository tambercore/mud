use crate::brill::wordclass::Wordclass;
use crate::ccg::ccg_types::{CCGCategory, CCGNode, CCGOperator};
use crate::lambda::types::LambdaEntity;
use crate::montague::expression::Expression;

/// Given a (word, POS TAG), return the corresponding montague grammar representation
///
/// // JOHN : NNP, LIKES: VBZ, GOUDA: NN
pub fn map_word_to_expression(word: String, pos_tag: &Wordclass, ccg_tag: &CCGNode) -> Result<LambdaEntity, String> {
    match pos_tag {
        Wordclass::NNP => Ok(LambdaEntity::Variable(Box::from(Expression::Variable(word)))),
        Wordclass::NN => Ok(LambdaEntity::Variable(Box::from(Expression::Variable(word)))),
        Wordclass::VBZ => {
            // Extract the number of arguments from the CCGNode
            println!("tagging verb {}", ccg_tag);
            let num_arguments = count_rules(&ccg_tag)?;

            if num_arguments == 0 {
                return Err(String::from("Invalid number of arguments (0) for verb"));
            }

            println!("arguments: {num_arguments}");

            // Create an empty list of arguments
            let mut arguments = Vec::new();
            for i in 1..=num_arguments {
                arguments.push(format!("x{}", i));
            }

            // Create a base predicate
            let expression = LambdaEntity::Variable(Box::from(Expression::Predicate(word, arguments)));

            // Wrap the predicate in abstractions for each argument
            let mut final_expression = expression;

            for i in (1..=num_arguments) {
                let arg_name = format!("x{}", i);

                // Wrap the current expression in an abstraction
                final_expression = LambdaEntity::Abstraction(arg_name, Box::from(final_expression));
            }

            Ok(final_expression)
        }
        _ => Err(String::from("not yet implemented"))
    }
}


/// Helper function to count the number of arguments in a CCGNode
/// Helper function to count the number of FA and BA rules in a CCGNode tree
fn count_rules(ccg_tag: &CCGNode) -> Result<usize, String> {
    let mut application_count = 0;

    // Check if the node applies any rules itself.
    match &ccg_tag.category {
        // If the category is of the form A/B or B\A, we look at its children.
        CCGCategory::Composed { left, right, operator } => {
            // If the operator is Forward, we have a potential FA rule.
            if let CCGOperator::Forward = operator {
                    application_count += 1;
            }
            // If the operator is Backward, we have a potential BA rule.
            if let CCGOperator::Backward = operator {
                    application_count += 1;
            }
            // Recursively count rules in the left and right subtrees.
            let left_rules = count_rules(&CCGNode { category: *(*left).clone(), ..ccg_tag.clone() })?;
            let right_rules = count_rules(&CCGNode { category: *(*right).clone(), ..ccg_tag.clone() })?;
            application_count += left_rules + right_rules;
        }
        // Simple categories like S, NP, etc. do not apply any rules.
        _ => {}
    }

    Ok((application_count))
}
