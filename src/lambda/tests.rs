use crate::lambda::types::*;
use crate::lambda::reduce::*;


#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use crate::lambda::types::LambdaEntity::Variable;
    use crate::montague::expression::Expression;
    use crate::montague::expression::Expression::*;
    use super::*;

    // Helper function to get Church TRUE
    fn get_church_true() -> LambdaEntity {
        LambdaEntity::Abstraction(
            Box::from(Variable(Box::from(Var("t".to_string())))),
            Box::new(LambdaEntity::Abstraction(
                Box::from(Variable(Box::from(Var("f".to_string())))),
                Box::new(LambdaEntity::Variable(Box::from(Expression::Var("t".to_string())))),
            )),
        )
    }

    // Helper function to get Church FALSE
    fn get_church_false() -> LambdaEntity {
        LambdaEntity::Abstraction(
            Box::from(Variable(Box::from(Var("t".to_string())))),
            Box::new(LambdaEntity::Abstraction(
                Box::from(Variable(Box::from(Var("f".to_string())))),
                Box::new(LambdaEntity::Variable(Box::from(Expression::Var("f".to_string())))),
            )),
        )
    }

    // Define Church-encoded AND
    fn get_church_and() -> LambdaEntity {
        LambdaEntity::Abstraction(
            Box::from(Variable(Box::from(Var("p".to_string())))),
            Box::new(LambdaEntity::Abstraction(
                Box::from(Variable(Box::from(Var("q".to_string())))),
                Box::new(LambdaEntity::Application(
                    Box::new(LambdaEntity::Application(
                        Box::new(LambdaEntity::Variable(Box::from(Expression::Var("p".to_string())))),
                        Box::new(LambdaEntity::Variable(Box::from(Expression::Var("q".to_string())))),
                    )),
                    Box::new(get_church_false()),
                )),
            )),
        )
    }


    fn is_church_true(entity: &LambdaEntity) -> bool {
        // Check if the entity matches the structure of Church True
        match entity {
            LambdaEntity::Abstraction(ref param1, ref body1) => {
                if *param1 == Box::from(Variable(Box::from(Var("t".to_string())))) {
                    if let LambdaEntity::Abstraction(ref param2, ref body2) = **body1 {
                        if *param2 == Box::from(Variable(Box::from(Var("f".to_string())))) {
                            // Check if the body of the second abstraction is the variable "t"
                            if let LambdaEntity::Variable(ref var) = **body2 {
                                if let Expression::Var(ref inner_var) = **var {
                                    return inner_var == "t";
                                }
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn is_church_false(entity: &LambdaEntity) -> bool {
        // Check if the entity matches the structure of Church False
        match entity {
            LambdaEntity::Abstraction(ref param1, ref body1) => {
                if *param1 == Box::from(Variable(Box::from(Var("t".to_string())))) {
                    if let LambdaEntity::Abstraction(ref param2, ref body2) = **body1 {
                        if *param2 == Box::from(Variable(Box::from(Var("f".to_string())))) {
                            // Check if the body of the second abstraction is the variable "f"
                            if let LambdaEntity::Variable(ref var) = **body2 {
                                if let Expression::Var(ref inner_var) = **var {
                                    return inner_var == "f";
                                }
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    #[test]
    fn test_church_and_tt() {
        let church_true = get_church_true();
        let expr = LambdaEntity::Application(
            Box::new(LambdaEntity::Application(
                Box::new(get_church_and()),
                Box::new(church_true.clone()),
            )),
            Box::new(church_true.clone()),
        );


        let reduced_expr = &reduce(&expr);
        let result = is_church_true(reduced_expr);
        assert!(
            result,
            "Expected Church True, but got {}",
            expr
        );
    }

    #[test]
    fn test_church_and_tf() {
        let church_true = get_church_true();
        let church_false = get_church_false();
        let expr = LambdaEntity::Application(
            Box::new(LambdaEntity::Application(
                Box::new(get_church_and()),
                Box::new(church_true.clone()),
            )),
            Box::new(church_false.clone()),
        );


        let reduced_expr = &reduce(&expr);
        let result = is_church_false(reduced_expr);
        assert!(
            result,
            "Expected Church False, but got {}",
            expr
        );
    }

    #[test]
    fn test_church_and_ft() {
        let church_false = get_church_false();
        let church_true = get_church_true();
        let expr = LambdaEntity::Application(
            Box::new(LambdaEntity::Application(
                Box::new(get_church_and()),
                Box::new(church_false.clone()),
            )),
            Box::new(church_true.clone()),
        );

        let reduced_expr = &reduce(&expr);
        let result = is_church_false(reduced_expr);
        assert!(
            result,
            "Expected Church False, but got {}",
            expr
        );
    }

    #[test]
    fn test_church_and_ff() {
        let church_false = get_church_false();
        let expr = LambdaEntity::Application(
            Box::new(LambdaEntity::Application(
                Box::new(get_church_and()),
                Box::new(church_false.clone()),
            )),
            Box::new(church_false.clone()),
        );

        let reduced_expr = &reduce(&expr);
        let result = is_church_false(reduced_expr);
        assert!(
            result,
            "Expected Church False, but got {}",
            expr
        );
    }

    #[test]
    fn test_expression_application() {

        let lhs = LambdaEntity::Abstraction(Box::from(Variable(Box::from(Var("x".to_string())))),
                                            Box::from((LambdaEntity::Abstraction(Box::from(Variable(Box::from(Var("y".to_string())))),
                                                                                 Box::from((LambdaEntity::Variable(Box::from(Expression::Predicate(Variable(Box::from(Var("Likes".to_string()))), vec!["y".to_string(), "x".to_string()])))))
                                            )))
        );

        let rhs = LambdaEntity::Variable(Box::from(Expression::Var("gouda".to_string())));

        let expr = LambdaEntity::Application(Box::from(lhs), Box::from(rhs));
        let reduced_expr = reduce(&expr);

        let target_expr = LambdaEntity::Abstraction(Box::from(Variable(Box::from(Var("y".to_string())))), Box::from((LambdaEntity::Variable(Box::from(Expression::Predicate(Variable(Box::from(Var("Likes".to_string()))), vec!["y".to_string(), "gouda".to_string()]))))));

        println!("target expr: {target_expr}");

        assert_eq!(
            reduced_expr, target_expr,
            "Expected {:?}, but got {:?}",
            target_expr, reduced_expr
        );

        let lhs = LambdaEntity::Variable(Box::from(Expression::Var("John".to_string())));

        let final_expr = LambdaEntity::Application(Box::from(target_expr), Box::from(lhs));
        let final_expr_reduced = reduce(&final_expr);
        let target = LambdaEntity::Variable(Box::from(Expression::Predicate(Variable(Box::from(Var("Likes".to_string()))), vec!["John".to_string(), "gouda".to_string()])));

        assert_eq!(
            final_expr_reduced, target,
            "Expected {:?}, but got {:?}",
            final_expr_reduced, target
        );
    }



}
