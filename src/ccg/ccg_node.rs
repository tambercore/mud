use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;
use crate::ccg::ccg_word::CCGWord;
use super::ccg_rule::CCGRule;
use super::ccg_type::CCGType;


#[derive(Debug, Clone)]
pub struct CCGNode {
    pub node_type: CCGType,
    pub word: CCGWord,
    pub rule: CCGRule,
    pub children: Vec<Box<CCGNode>>, // Use Box to handle recursion
}



impl<'de> Deserialize<'de> for CCGNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
    }
}
