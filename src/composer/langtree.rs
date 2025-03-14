use crate::lambda::types::LambdaEntity;
use ascii_tree::{Tree::*, Tree, write_tree};
use std::fmt;



/* Semantic Tree, in this definition has relations, tokens and joins */

pub type Relation = (String, Vec<Box<SemanticTree>>);
pub type Token = String;
pub type Join = (Box<SemanticTree>, Box<SemanticTree>);

#[derive(Clone, Debug, PartialEq)]
pub enum SemanticTree {
    NonTerminal(Relation),
    Terminal(Token),
    Conj(Join),
}



/* Implementing fmt::Display for the semantic tree */
impl fmt::Display for SemanticTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn to_ascii_tree(node: &SemanticTree) -> Tree {
            match node {
                SemanticTree::NonTerminal((pred, args)) => {
                    let title = format!("Predicate: {}", pred);
                    let children = args.iter()
                        .map(|child| to_ascii_tree(child.as_ref()))
                        .collect();
                    Node(title, children)
                }

                SemanticTree::Terminal(t) => {
                    Node(format!("Terminal: {}", t), vec![])
                }

                SemanticTree::Conj((lhs, rhs)) => {
                    let title = "Conjunction".to_string();
                    let left_child = to_ascii_tree(lhs);
                    let right_child = to_ascii_tree(rhs);
                    Node(title, vec![left_child, right_child])
                }
            }
        }

        let ascii_tree = to_ascii_tree(self);
        let mut output = String::new();
        write_tree(&mut output, &ascii_tree)
            .map_err(|_| fmt::Error)?;

        write!(f, "{}", output)
    }
}


/* Conversion of LC Expressions to Semantic Trees */
pub fn lambda_to_semantic(node: Box<LambdaEntity>) -> Result<SemanticTree, String> {
    match *node {
        LambdaEntity::Var(v) => Ok(SemanticTree::Terminal(v.name.clone())),

        LambdaEntity::Pred(p) => {
            let converted_args = p.args
                .into_iter()
                .map(lambda_to_semantic)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(Box::new)
                .collect();

            Ok(SemanticTree::NonTerminal((p.iden.clone(), converted_args)))
        }

        LambdaEntity::Conj(conj) => {
            Ok(SemanticTree::Conj((
                Box::new(lambda_to_semantic(conj.lhs)?),
                Box::new(lambda_to_semantic(conj.rhs)?),
            ) as Join))
        }

        LambdaEntity::App(_) => Err("Can't convert Application".to_string()),
        LambdaEntity::Abs(_) => Err("Can't convert Abstraction".to_string()),
    }
}