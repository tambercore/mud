use std::fmt;
#[derive(Clone, Debug)]
pub enum LambdaElement {
    Term(String),
    Predicate(String, Vec<LambdaElement>)
}

















impl fmt::Display for LambdaElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LambdaElement::Term(name) => write!(f, "{}", name),
            LambdaElement::Predicate(name, args) => {
                if args.is_empty() {
                    write!(f, "{}", name)
                } else {
                    let args_str = args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ");
                    write!(f, "{}({})", name, args_str)
                }
            }
        }
    }
}


impl PartialEq for LambdaElement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LambdaElement::Term(v1), LambdaElement::Term(v2)) => v1 == v2,
            (LambdaElement::Predicate(pred1, args1), LambdaElement::Predicate(pred2, args2)) => {
                pred1 == pred2 && args1 == args2
            }
            _ => false,
        }
    }
}
