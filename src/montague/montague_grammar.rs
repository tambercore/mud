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
            let num_arguments = count_rules(&ccg_tag)?;

            // Create abstractions based on the number of arguments
            let mut expression = LambdaEntity::Variable(Box::from(Expression::Predicate(word, vec![])));
            for i in (1..=num_arguments).rev() {
                let arg_name = format!("x{}", i);
                let expr = LambdaEntity::Abstraction(arg_name.clone(), Box::from(expression.clone()));
                if let LambdaEntity::Variable(mut expression_box) = expr {
                    if let Expression::Predicate(_, ref mut args) = *expression_box {
                        args.push(arg_name);
                    }
                }
            }

            Ok(expression.clone())
        }
        _ => Err(String::from("not yet implemented"))
    }
}


/// Helper function to count the number of arguments in a CCGNode
/// Helper function to count the number of FA and BA rules in a CCGNode tree
fn count_rules(ccg_tag: &CCGNode) -> Result<usize, String> {
    let mut fa_count = 0;
    let mut ba_count = 0;

    // Check if the node applies any rules itself.
    match &ccg_tag.category {
        // If the category is of the form A/B or B\A, we look at its children.
        CCGCategory::Composed { left, right, operator } => {
            // If the operator is Forward, we have a potential FA rule.
            if let CCGOperator::Forward = operator {
                    fa_count += 1;
            }
            // If the operator is Backward, we have a potential BA rule.
            if let CCGOperator::Backward = operator {
                    ba_count += 1;
            }
            // Recursively count rules in the left and right subtrees.
            let left_rules = count_rules(&CCGNode { category: *(*left).clone(), ..ccg_tag.clone() })?;
            let right_rules = count_rules(&CCGNode { category: *(*right).clone(), ..ccg_tag.clone() })?;
            fa_count += left_rules + right_rules;
            ba_count += left_rules + right_rules;
        }
        // Simple categories like S, NP, etc. do not apply any rules.
        _ => {}
    }

    Ok((fa_count + ba_count))
}
