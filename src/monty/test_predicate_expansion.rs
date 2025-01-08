use crate::lambda::conjunction::Conjunction;
use crate::lambda::predicate::Predicate;
use crate::lambda::variable::Variable;

use crate::lambda::types::{Expandable, LambdaEntity};
use crate::{λAbs, λApp, λPred, λConj, λVar};

#[test]
fn test_predicate_expansion() {

    // P(x ^ y) —> P(x) ^ P(y)
    let simple_conj = λPred!("P".to_string(), vec![λConj!(λVar!("x".to_string()), λVar!("y".to_string()))]);
    assert_eq!(
        (*simple_conj).expand(),
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
       (*conj_in_conj).expand(),
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
        (*conj_with_other_args).expand(),
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
        (*nested_conj).expand(),
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
        (*no_conj).expand(),
        λPred!("P".to_string(), vec![λVar!("x".to_string())])
    );
}
