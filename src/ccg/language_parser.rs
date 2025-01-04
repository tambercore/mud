use std::{process::Command, fs::File, io::Read};
use serde_json::Error as SerdeError;
use crate::brill::wordclass::Wordclass;
use super::ccg_types::{CCGNode, add_tags};

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
    let original_tree = read_json("data/temp_ccg_parsed_sentence.json").expect("Failed to read tree");
    add_tags(original_tree, vec_of_words_to_tags)
}

/// Reads a JSON file and attempts to deserialize it into a `CCGNode`.
///
/// Returns a `Result` containing the parsed `CCGNode` on success, or an error message if the file can't be read or parsed.
fn read_json(file_path: &str) -> Result<CCGNode, String> {
    let mut content = String::new();

    // Open the file and read its content into a string.
    File::open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?
        .read_to_string(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Deserialize the JSON content into a CCGNode.
    serde_json::from_str(&content).map_err(|e: SerdeError| format!("Failed to parse CCG from JSON: {}", e))
}
