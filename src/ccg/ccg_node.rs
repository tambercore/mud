use super::ccg_rule::*;
use super::ccg_type::*;

type CCGNode = (CCGType, CCGRule, Vec<CCGNode>);