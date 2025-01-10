use crate::brill::wordclass::Wordclass;
use crate::ccg::node::CCGNode;
use crate::ccg::word::CCGWord;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn _insert_tags<I>(old_node: &Rc<RefCell<CCGNode>>, words_and_tags_iter: &mut I, parent: Option<Weak<RefCell<CCGNode>>>, ) -> Rc<RefCell<CCGNode>> where I: Iterator<Item = (String, Wordclass)>,
{
    // Extract old_node’s fields
    let old_borrow = old_node.borrow();
    let node_type = old_borrow.node_type.clone();
    let rule = old_borrow.rule.clone();
    let old_word = old_borrow.word.clone();

    drop(old_borrow);
    // (We drop it here so we can borrow_mut() later without panic.)

    // Create a new node
    let new_node = Rc::new(RefCell::new(CCGNode {
        node_type,
        rule,
        // If this node has a word, consume one (String, Wordclass) pair from the iterator
        word: match old_word {
            Some(_) => {
                let (updated_text, updated_tag) = words_and_tags_iter
                    .next()
                    .expect("Ran out of Wordclass pairs while reconstructing CCG tree.");

                Some(CCGWord {
                    text: updated_text,
                    tag: updated_tag,
                })
            }
            None => None,
        },
        children: RefCell::new(vec![]),
        parent: RefCell::new(parent),
    }));

    // Process children recursively
    let old_borrow = old_node.borrow();
    if !old_borrow.children.borrow().is_empty() {
        for child_rc in old_borrow.children.borrow().iter() {
            let child = _insert_tags(
                child_rc,
                words_and_tags_iter,
                Some(Rc::downgrade(&new_node)),
            );
            new_node.borrow_mut().children.borrow_mut().push(child);
        }
    }

    new_node
}

pub fn insert_tags(
    old_tree: Rc<RefCell<CCGNode>>,
    words_and_tags: Vec<(String, Wordclass)>,
) -> Rc<RefCell<CCGNode>> {
    let mut iter = words_and_tags.into_iter();
    _insert_tags(&old_tree, &mut iter, None)
}