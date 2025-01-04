use serde::{Deserialize, Deserializer, de::Error};
use std::fmt;
use crate::brill::wordclass::Wordclass;


/// Enum representing CCG operators used in category composition.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum CCGOperator {
    #[serde(rename = "/")]
    Forward,  // The forward operator (used in forward composition).

    #[serde(rename = "\\")]
    Backward, // The backward operator (used in backward composition).
}


impl fmt::Display for CCGOperator {
    /// Formats the CCG operator as a string (`/` or `\`).
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            CCGOperator::Forward => "/",
            CCGOperator::Backward => "\\",
        })
    }
}


/// Enum representing different CCG categories, including simple and composed categories.
#[derive(Debug, Clone, PartialEq)]
pub enum CCGCategory {
    S,  // Sentence category.
    NP, // Noun Phrase category.
    N,  // Noun category.

    // A composed category with a left and right category and an operator (forward or backward).
    Composed {
        left: Box<CCGCategory>,  // Left category in the composition.
        right: Box<CCGCategory>, // Right category in the composition.
        operator: CCGOperator,   // The composition operator (either `Forward` or `Backward`).
    },
    V,
    CONJ,
}


/// Implementation for `Deserialize` to parse CCG categories from JSON strings.
impl<'de> Deserialize<'de> for CCGCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, try to deserialize the value as a simple string.
        let value: String = Deserialize::deserialize(deserializer)?;

        // Match the string to the known simple category types.
        match value.as_str() {
            "s" => Ok(CCGCategory::S),
            "np" => Ok(CCGCategory::NP),
            "n" => Ok(CCGCategory::N),
            "conj" => Ok(CCGCategory::CONJ),
            _ => {
                // If it's not a simple category, attempt to parse it as a composed category.
                if let Some((left_str, op, right_str)) = parse_composed_category(value.as_str()) {
                    let left = Box::new(Self::from_string(left_str).map_err(D::Error::custom)?);
                    let operator = match op {
                        "/" => CCGOperator::Forward,
                        "\\" => CCGOperator::Backward,
                        _ => return Err(D::Error::custom(format!("Invalid composition operator: {}", value))),
                    };
                    let right = Box::new(Self::from_string(right_str).map_err(D::Error::custom)?);

                    // Return the composed category.
                    Ok(CCGCategory::Composed { left, right, operator })
                } else {
                    // If the category is invalid, return an error.
                    Err(D::Error::custom(format!("Invalid category format: {}", value.as_str())))
                }
            }
        }
    }
}


impl CCGCategory {
    /// Helper function to convert a string into a CCGCategory.
    /// This function handles both simple and composed categories.
    fn from_string(value: &str) -> Result<Self, String> {
        match value {
            "s" => Ok(CCGCategory::S),
            "np" => Ok(CCGCategory::NP),
            "n" => Ok(CCGCategory::N),
            "conj" => Ok(CCGCategory::CONJ),
            _ => {
                // Try to parse as a composed category.
                if let Some((left_str, op, right_str)) = parse_composed_category(value) {
                    let left = Box::new(CCGCategory::from_string(left_str)?);
                    let operator = match op {
                        "/" => CCGOperator::Forward,
                        "\\" => CCGOperator::Backward,
                        _ => return Err(format!("Invalid composition operator: {}", value)),
                    };
                    let right = Box::new(CCGCategory::from_string(right_str)?);
                    return Ok(CCGCategory::Composed { left, right, operator });
                }
                Err(format!("Unsupported category type {}", value))
            }
        }
    }
}


/// Enum representing CCG parsing rules.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum CCGRule {
    L,  // Leftward composition rule.
    FA, // Forward application rule.
    BA, // Backward application rule.
    U, // Unary rule.
    CONJ, // conjunction rule.
}


impl fmt::Display for CCGRule {
    /// Formats the CCG rule as a string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            CCGRule::L => "L",
            CCGRule::FA => "FA",
            CCGRule::BA => "BA",
            CCGRule::U => "U",
            CCGRule::CONJ => "CONJ",
        })
    }
}


/// Struct representing a CCG node in a parse tree.
///
/// A node contains a category, an optional rule, optional text, and optional child nodes.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CCGNode {
    #[serde(rename = "type")]
    pub category: CCGCategory,      // The category of the node (e.g., S, NP, N, or composed category).
    pub rule: Option<CCGRule>,     // The rule applied to the node (e.g., `L`, `FA`, `BA`).
    pub text: Option<String>,      // Optional text associated with the node.
    pub class: Option<Wordclass>,   // Wordclass of terminal node
    pub children: Option<Vec<CCGNode>>, // Optional child nodes of the current node.
}


impl fmt::Display for CCGCategory {
    /// Formats the CCG category as a string (e.g., `S`, `NP`, `N`, or a composed category).
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CCGCategory::S => write!(f, "S"),
            CCGCategory::NP => write!(f, "NP"),
            CCGCategory::N => write!(f, "N"),
            CCGCategory::V => write!(f, "V"),
            CCGCategory::CONJ => write!(f, "CONJ"),
            CCGCategory::Composed { ref left, ref right, ref operator } => {
                write!(f, "({} {} {})", left, operator, right)
            }
        }
    }
}


impl fmt::Display for CCGNode {
    /// Formats the CCG node as a string, including category, rule, text, and children.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let category_str = format!("{}", self.category);
        let rule_str = if let Some(rule) = &self.rule {
            format!(" ({})", rule)
        } else {
            String::new()
        };

        let class_str = if let Some(class) = &self.class {
            format!(" ({})", class)
        } else {
            String::new()
        };

        let text_str = if let Some(text) = &self.text {
            format!(": {}", text)
        } else {
            String::new()
        };

        let children_str = if let Some(children) = &self.children {
            let children_display: Vec<String> = children.iter().map(|child| format!("{}", child)).collect();
            format!("\nChildren: [{}]", children_display.join(", "))
        } else {
            String::new()
        };

        write!(f, "{}{}{}{}{}", category_str, rule_str, text_str, class_str, children_str)
    }
}


/// Given a tree, add POS tags to nodes containing words.
pub(crate) fn add_tags(mut tree: CCGNode, words_to_tags: Vec<(String, Wordclass)>) -> CCGNode {
    if let Some(ref text) = tree.text {
        // Check if the current node has a word that matches one in `words_to_tags`.
        if let Some((_, tag)) = words_to_tags.iter().find(|(word, _)| word == text) {
            tree.class = Some(tag.clone());
        }
    }

    // If the node has children, recursively apply `add_tags` to them.
    if let Some(children) = &mut tree.children {
        for child in children {
            *child = add_tags(child.clone(), words_to_tags.clone());
        }
    }

    tree
}


/// Helper function to parse composed categories from a string.
/// A composed category is in the form of "(left_category operator right_category)".
fn parse_composed_category(value: &str) -> Option<(&str, &str, &str)> {
    // If the value is a simple category like (s) or (s\np), remove the outer parentheses
    let value = if is_simple_parentheses(value) {
        &value[1..value.len() - 1]  // Slice to remove the first and last character
    } else {
        value
    };

    if let Some(pos) = find_operator_position_outside_parentheses(value) {
        let (left, remainder) = value.split_at(pos);
        let (operator, right) = remainder.split_at(1);
        Some((left.trim(), operator, right.trim()))
    } else {
        None
    }
}


/// Checks if the string is a simple category surrounded by parentheses (e.g., (s) or (s\np))
fn is_simple_parentheses(value: &str) -> bool {
    // A simple parenthesized category should not contain any operators and should not be empty
    if value.starts_with('(') && value.ends_with(')') {
        let inner = &value[1..value.len() - 1]; // Remove the outer parentheses
        !inner.contains('(') && !inner.contains(')')
    } else {
        false
    }
}


/// Finds the position of an operator (`/` or `\`) that is outside of parentheses.
/// The operator is considered outside of parentheses if the string starts with `(`
/// and ends with `)` and there is a matching parenthesis pair around the operator.
fn find_operator_position_outside_parentheses(value: &str) -> Option<usize> {
    let mut paren_count = 0;

    // Iterate over each character in the string
    for (i, c) in value.char_indices() {
        match c {
            '(' => paren_count += 1,   // Increment count on encountering '('
            ')' => paren_count -= 1,   // Decrement count on encountering ')'
            // If it's an operator and we are outside parentheses
            '/' | '\\' if paren_count == 0 => return Some(i),
            _ => {}
        }
    }
    None
}


/// Recursively extracts terminal nodes (nodes with `text`) from a CCGNode tree.
pub fn get_terminal_nodes(node: &CCGNode) -> Vec<&CCGNode> {
    let mut terminals = Vec::new();

    // If the current node has text, it's a terminal node.
    if node.text.is_some() {
        terminals.push(node);
    }

    // If the node has children, recursively collect terminal nodes from them.
    if let Some(children) = &node.children {
        for child in children {
            terminals.extend(get_terminal_nodes(child));
        }
    }

    terminals
}