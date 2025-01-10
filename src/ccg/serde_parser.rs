use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use serde::Deserialize;
use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::ccg::word::CCGWord;

#[derive(Debug, Clone, Deserialize)]
pub struct PlainCCGNode {
    #[serde(rename = "type")]
    pub node_type: CCGType,

    #[serde(rename = "text")]
    pub word: Option<CCGWord>,
    pub rule: CCGRule,
    pub children: Option<Vec<PlainCCGNode>>,

    // For serialization, don't store `parent` at all,
}

fn from_plain(plain: PlainCCGNode) -> Rc<RefCell<CCGNode>> {
    let node = Rc::new(RefCell::new(CCGNode {
        node_type: plain.node_type,
        word: plain.word,
        rule: plain.rule,
        children: RefCell::new(vec![]),
        parent: RefCell::new(None),
    }));

    // Recursively build children
    match plain.children {
        None => {}
        Some(children) => {
            for child_plain in children {
                let child_rc = from_plain(child_plain);

                // Set child's parent to node
                child_rc.borrow_mut().parent
                    .replace(Option::from(Rc::downgrade(&node)));

                // Push into node’s children
                node.borrow_mut().children.borrow_mut().push(child_rc);
            }
        }
    }

    node
}

fn ccgnode_parse(file_path: &str) -> Result<PlainCCGNode, String> {
    let mut content = String::new();

    // Read file to a string
    File::open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?
        .read_to_string(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Print for debugging
    println!("Raw CCG Output:\n {}", content);

    // Deserialize to PlainCCGNode
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse CCG from JSON: {}", e))
}

pub fn parse_into_ccgnode(file_path: &str) -> Result<Rc<RefCell<CCGNode>>, String> {
    let plain = ccgnode_parse(file_path)?;   // returns PlainCCGNode
    Ok(from_plain(plain))                                  // convert to Rc<RefCell<CCGNode>>
}