use std::process::Command;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CCGNode {
    #[serde(rename = "type")]
    node_type: String,
    pub(crate) rule: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) children: Option<Vec<CCGNode>>,
}

pub fn english_to_ccg() -> Result<CCGNode, Box<dyn std::error::Error>> {
    let file_path = "data/ccg_parsed_sentence.json";
    Command::new("data/lambeq/lambeq_env/Scripts/python.exe")
        .arg("data/lambeq/run_lambeq.py")
        .output()
        .expect("Failed to execute Python command. Ensure a venv, named `lambeq_env`, is installed at data/lambeq/lambeq_env.\
        Ensure that `lambeq` is correctly installed inside the venv.");

    Ok(read_json(file_path)?)
}

fn read_json(file_path: &str) -> Result<CCGNode, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let root: CCGNode = serde_json::from_str(&contents)?;
    Ok(root)
}

