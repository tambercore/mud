use super::ccg_rule::*;
use super::ccg_type::*;

#[derive(Debug, Clone)]
pub struct CCGNode {
    pub node_type: CCGType,
    pub rule: CCGRule,
    pub children: Vec<Box<CCGNode>>, // Use Box to handle recursion
}