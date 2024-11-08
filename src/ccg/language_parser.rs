use std::{process::Command, fs::File, io::Read};
use serde_json::Error as SerdeError;
use super::ccg_types::CCGNode;

/// Runs a Python script to process an English sentence and parses the result into a CCG node.
///
/// This function assumes the presence of a virtual environment with `lambeq` installed and
/// the necessary Python script located at `data/lambeq/run_lambeq.py`.
/// After executing the Python script, it reads and deserializes the CCG JSON result from `data/temp_ccg_parsed_sentence.json`.
///
/// # Returns
/// Returns a `Result` containing the parsed `CCGNode` on success, or an error message if the process fails.
pub fn english_to_ccg() -> Result<CCGNode, String> {
    // Run the Python script to process the English sentence.
    Command::new("data/lambeq/lambeq_env/Scripts/python.exe")
        .arg("data/lambeq/run_lambeq.py")
        .output() // Execute the command and expect no result.
        .map_err(|_| "Failed to execute Python command. Ensure the virtual environment and lambeq are properly installed.")?;

    // Read and parse the resulting JSON file into a CCGNode.
    read_json("data/temp_ccg_parsed_sentence.json")
}

/// Reads a JSON file and attempts to deserialize it into a `CCGNode`.
///
/// # Arguments
/// * `file_path` - The path to the JSON file to read.
///
/// # Returns
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
