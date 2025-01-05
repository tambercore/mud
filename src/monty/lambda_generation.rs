use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::*;

fn ccg_to_lambda (root: CCGNode) -> LambdaEntity {
    match root.rule {

        CCGRule::BackwardApplication => {
            return LambdaEntity::Application(ccg_to_lambda(root[0]), ccg_to_lambda(root[1])))
        }

        CCGRule::ForwardApplication => {

        }

        CCGRule::BackwardComposition => {}
        CCGRule::BackwardCrossedComposition => {}
        CCGRule::BackwardTypeRaising => {}
        CCGRule::Conjunction => {}
        CCGRule::ForwardComposition => {}
        CCGRule::ForwardCrossedComposition => {}
        CCGRule::ForwardTypeRaising => {}
        CCGRule::GeneralizedBackwardComposition => {}
        CCGRule::GeneralizedBackwardCrossedComposition => {}
        CCGRule::GeneralizedForwardComposition => {}
        CCGRule::GeneralizedForwardCrossedComposition => {}
        CCGRule::Lexical => {}
        CCGRule::RemovePunctuationLeft => {}
        CCGRule::RemovePunctuationRight => {}
        CCGRule::Unary => {}
        CCGRule::Unknown => {}
    }
    LambdaEntity::Variable("test".to_string())
}
