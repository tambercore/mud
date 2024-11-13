use crate::lambda::types::*;
use crate::lambda::reduce::*;


#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to get Church TRUE
    fn get_church_true() -> LambdaEntity {
        LambdaEntity::Abstraction(
            "t".to_string(),
            Box::new(LambdaEntity::Abstraction(
                "f".to_string(),
                Box::new(LambdaEntity::Variable("t".to_string())),
            )),
        )
    }

    // Helper function to get Church FALSE
    fn get_church_false() -> LambdaEntity {
        LambdaEntity::Abstraction(
            "t".to_string(),
            Box::new(LambdaEntity::Abstraction(
                "f".to_string(),
                Box::new(LambdaEntity::Variable("f".to_string())),
            )),
        )
    }

    // Define Church-encoded AND
    fn get_church_and() -> LambdaEntity {
        LambdaEntity::Abstraction(
            "p".to_string(),
            Box::new(LambdaEntity::Abstraction(
                "q".to_string(),
                Box::new(LambdaEntity::Application(
                    Box::new(LambdaEntity::Application(
                        Box::new(LambdaEntity::Variable("p".to_string())),
                        Box::new(LambdaEntity::Variable("q".to_string())),
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
                if param1 == "t" {
                    if let LambdaEntity::Abstraction(ref param2, ref body2) = **body1 {
                        if param2 == "f" {
                            // Check if the body of the second abstraction is the variable "t"
                            if let LambdaEntity::Variable(ref var) = **body2 {
                                return var == "t";
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
                if param1 == "t" {
                    if let LambdaEntity::Abstraction(ref param2, ref body2) = **body1 {
                        if param2 == "f" {
                            // Check if the body of the second abstraction is the variable "f"
                            if let LambdaEntity::Variable(ref var) = **body2 {
                                return var == "f";
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

}
