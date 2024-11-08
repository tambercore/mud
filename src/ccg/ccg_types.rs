use serde::{Deserialize, Deserializer, de::Error};
use std::fmt;

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

/// Helper function to parse composed categories from a string.
///
/// A composed category is in the form of "(left_category operator right_category)".
fn parse_composed_category(value: &str) -> Option<(&str, &str, &str)> {
    let trimmed_value = value.trim();
    if trimmed_value.starts_with('(') && trimmed_value.ends_with(')') {
        let inside = &trimmed_value[1..trimmed_value.len() - 1].trim();
        if let Some(pos) = find_operator_position_outside_parentheses(inside) {
            let (left, remainder) = inside.split_at(pos);
            let (operator, right) = remainder.split_at(1);
            return Some((left.trim(), operator, right.trim()));
        } else {
            return Some((inside.trim(), "", ""));
        }
    }

    // If there's no parentheses, look for the operator directly.
    if let Some(pos) = find_operator_position_outside_parentheses(value) {
        let (left, remainder) = value.split_at(pos);
        let (operator, right) = remainder.split_at(1);
        Some((left.trim(), operator, right.trim()))
    } else {
        None
    }
}

/// Finds the position of an operator (`/` or `\`) that is outside of parentheses.
///
/// The function ensures the operator is not inside any parentheses by tracking open and close parentheses.
fn find_operator_position_outside_parentheses(value: &str) -> Option<usize> {
    let mut paren_count = 0;
    for (i, c) in value.char_indices() {
        match c {
            '(' => paren_count += 1,
            ')' => paren_count -= 1,
            '/' | '\\' if paren_count == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// Enum representing CCG parsing rules.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum CCGRule {
    L,  // Leftward composition rule.
    FA, // Forward application rule.
    BA, // Backward application rule.
}

impl fmt::Display for CCGRule {
    /// Formats the CCG rule as a string (`L`, `FA`, or `BA`).
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            CCGRule::L => "L",
            CCGRule::FA => "FA",
            CCGRule::BA => "BA",
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
    pub children: Option<Vec<CCGNode>>, // Optional child nodes of the current node.
}

impl fmt::Display for CCGCategory {
    /// Formats the CCG category as a string (e.g., `S`, `NP`, `N`, or a composed category).
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CCGCategory::S => write!(f, "S"),
            CCGCategory::NP => write!(f, "NP"),
            CCGCategory::N => write!(f, "N"),
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

        write!(f, "{}{}{}{}", category_str, rule_str, text_str, children_str)
    }
}
