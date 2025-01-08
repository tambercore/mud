use crate::lambda::conjunction::Conjunction;
use crate::lambda::predicate::Predicate;
use crate::lambda::variable::Variable;

use crate::lambda::types::LambdaEntity;
use crate::{λAbs, λApp, λPred, λConj, λVar};


// todo should be in different place

pub fn expand_expression(expression: Box<LambdaEntity>) -> Box<LambdaEntity> {
    let mut current_expression = expression;

    loop {
        match *current_expression.clone() {
            LambdaEntity::Pred(predicate) => {
                // Attempt to expand the predicate
                let expanded_predicate = expand_predicate(predicate);
                if expanded_predicate == current_expression {
                    // If no further expansion is possible, break the loop
                    break;
                } else {
                    // Otherwise, update the current expression and continue
                    current_expression = expanded_predicate;
                }
            }
            LambdaEntity::Conj(conjunction) => {
                // Expand both sides of the conjunction
                let lhs_expanded = expand_expression(conjunction.lhs.clone());
                let rhs_expanded = expand_expression(conjunction.rhs.clone());

                // Return a conjunction of the expanded parts
                current_expression = λConj!(lhs_expanded, rhs_expanded);
                break;
            }
            _ => {
                // If no further expansion is possible, break the loop
                break;
            }
        }
    }

    current_expression
}
fn expand_predicate(predicate: Predicate) -> Box<LambdaEntity> {
    for (i, arg) in predicate.args.iter().enumerate() {
        if let LambdaEntity::Conj(conjunction) = &**arg {
            // Create a predicate with the conjunction's lhs
            let mut lhs_args = predicate.args.clone();
            lhs_args[i] = Box::new(*conjunction.lhs.clone());
            let lhs_predicate = LambdaEntity::Pred(Predicate {
                iden: predicate.iden.clone(),
                args: lhs_args,
            });

            // Create a predicate with the conjunction's rhs
            let mut rhs_args = predicate.args.clone();
            rhs_args[i] = Box::new(*conjunction.rhs.clone());
            let rhs_predicate = LambdaEntity::Pred(Predicate {
                iden: predicate.iden.clone(),
                args: rhs_args,
            });

            // Conjoin the two predicates
            return λConj!(Box::from(lhs_predicate), Box::from(rhs_predicate));
        }
    }

    // If no conjunctions were found, return the original predicate
    Box::from(LambdaEntity::Pred(predicate))
}

#[test]
fn test_predicate_expansion() {

    // P(x ^ y) —> P(x) ^ P(y)
    let simple_conj = λPred!("P".to_string(), vec![λConj!(λVar!("x".to_string()), λVar!("y".to_string()))]);
    assert_eq!(
        expand_expression(simple_conj),
        λConj!(
            λPred!("P".to_string(), vec![λVar!("x".to_string())]),
            λPred!("P".to_string(), vec![λVar!("y".to_string())])
        )
    );

    // P(x ^ (y ^ z)) —> P(x) ^ P((y ^ z)) —> P(x) ^ P(y) ^ P(z)
    let conj_in_conj = λPred!(
        "P".to_string(),
        vec![λConj!(
            λVar!("x".to_string()),
            λConj!(λVar!("y".to_string()), λVar!("z".to_string()))
        )]
    );
    assert_eq!(
        expand_expression(conj_in_conj),
        λConj!(
            λPred!("P".to_string(), vec![λVar!("x".to_string())]),
            λConj!(
                λPred!("P".to_string(), vec![λVar!("y".to_string())]),
                λPred!("P".to_string(), vec![λVar!("z".to_string())])
            )
        )
    );

    // P(x ^ y, z) —> P(x, z) ^ P(y, z)
    let conj_with_other_args = λPred!(
        "P".to_string(),
        vec![
            λConj!(λVar!("x".to_string()), λVar!("y".to_string())),
            λVar!("z".to_string())
        ]
    );
    assert_eq!(
        expand_expression(conj_with_other_args),
        λConj!(
            λPred!("P".to_string(), vec![λVar!("x".to_string()), λVar!("z".to_string())]),
            λPred!("P".to_string(), vec![λVar!("y".to_string()), λVar!("z".to_string())])
        )
    );

    // P((x ^ y) ^ z) —> P(x ^ y) ^ P(z) —> P(x) ^ P(y) ^ P(z)
    let nested_conj = λPred!(
        "P".to_string(),
        vec![λConj!(
            λConj!(λVar!("x".to_string()), λVar!("y".to_string())),
            λVar!("z".to_string())
        )]
    );
    assert_eq!(
        expand_expression(nested_conj),
        λConj!(
            λConj!(
                λPred!("P".to_string(), vec![λVar!("x".to_string())]),
                λPred!("P".to_string(), vec![λVar!("y".to_string())])
            ),
            λPred!("P".to_string(), vec![λVar!("z".to_string())])
        )
    );

    // Edge case: P(x) —> P(x) (no conjunctions to expand)
    let no_conj = λPred!("P".to_string(), vec![λVar!("x".to_string())]);
    assert_eq!(
        expand_expression(no_conj),
        λPred!("P".to_string(), vec![λVar!("x".to_string())])
    );
}
