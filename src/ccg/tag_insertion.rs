use crate::brill::wordclass::Wordclass;
use crate::ccg::node::CCGNode;
use crate::ccg::word::CCGWord;

// Function that recursively add tags
fn _insert_tags<I>(old_node: &CCGNode, words_and_tags_iter: &mut I) -> CCGNode where I: Iterator<Item = (String, Wordclass)>,
{
    // Recur through children.
    let new_children = old_node.children.as_ref().map(|child_vec| {
        child_vec
            .iter()
            .map(|child| { Box::new(_insert_tags(child, words_and_tags_iter)) })
            .collect()
    });

    // If this node has a word, pull one (word, tag) from the iterator
    let new_word = match &old_node.word {
        Some(old_word) => {
            // Grab the next (String, Wordclass) pair, if there isn't enough, something has gone wrong (panic!).
            let (updated_text, updated_tag) = words_and_tags_iter
                .next()
                .expect("Ran out of Wordclass pairs while reconstructing CCG tree.");

            Some(CCGWord {
                text: updated_text,
                tag: updated_tag,
            })
        }
        None => None,
    };

    return CCGNode {
        node_type: old_node.node_type.clone(),
        rule: old_node.rule.clone(),
        word: new_word,
        children: new_children,
        is_universal_quantification_node: old_node.is_universal_quantification_node,
        is_existential_quantification_node: old_node.is_existential_quantification_node
    }
}


// Function to insert tags
pub fn insert_tags(old_tree: &CCGNode, words_and_tags: Vec<(String, Wordclass)>) -> CCGNode {
    let mut iter = words_and_tags.into_iter();
    _insert_tags(old_tree, &mut iter)
}
