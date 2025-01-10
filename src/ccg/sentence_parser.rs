use std::{process::Command, fs::File, io::Read};
use std::cell::RefCell;
use std::rc::Rc;
use serde_json::Error as SerdeError;
use crate::brill::wordclass::Wordclass;
use crate::ccg::serde_parser::parse_into_ccgnode;
use super::tag_insertion::insert_tags;
use super::node::{CCGNode};



/// Runs a Python script to process an English sentence and parses the result into a CCG node.
///
/// This function assumes the presence of a virtual environment with `lambeq` installed and
/// the necessary Python script located at `data/lambeq/run_lambeq.py`.
/// After executing the Python script, it reads and deserializes the CCG JSON result from `data/temp_ccg_parsed_sentence.json`.
///
/// Returns a `Result` containing the parsed `CCGNode` on success, or an error message if the process fails.
pub fn english_to_ccg(sentence: &str, vec_of_words_to_tags: Vec<(String, Wordclass)>) -> CCGNode {

    // Determine the Python executable path based on the OS
    let python_executable = if cfg!(target_os = "windows") {
        "data/lambeq/lambeq_env/Scripts/python.exe"
    } else if cfg!(target_os = "macos") {
        "data/lambeq/lambeq_env/bin/python3" // Adjust path for macOS
    } else {
        "data/lambeq/lambeq_env/bin/python3" // or whatever linux uses.
    };

    // Pass the sentence to the Python script as a command line argument
    let output = Command::new(python_executable)
        .arg("data/lambeq/run_lambeq.py")
        .arg(sentence) // Pass sentence as argument to Python script
        .output()
        .map_err(|_| "Failed to execute Python command. Ensure the virtual environment and lambeq are properly installed.");

    // Read and parse the resulting JSON file into a CCGNode.
    let original_tree = parse_into_ccgnode("data/temp_ccg_parsed_sentence.json").expect("Failed to read tree");
    let tagged_tree_rc = insert_tags(original_tree, vec_of_words_to_tags);
    let tagged_tree: CCGNode = tagged_tree_rc.borrow().clone();

    tagged_tree
}
