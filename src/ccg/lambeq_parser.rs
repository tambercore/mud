use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use crate::ccg::node::CCGNode;
use crate::ccg::sentence_parser::ccgnode_parse;

/// Sends a POST request with the given sentences to the REST API server
/// and returns a vector of JSON strings representing the CCG trees.
pub fn sentences_to_ccgs(sentences: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::new();
    let url = "http://127.0.0.1:20040/sentences";

    // Construct the JSON payload with the "sentences" field.
    let payload = json!({ "sentences": sentences });

    // Send the POST request.
    let response = client.post(url).json(&payload).send()?;

    // Parse the JSON response.
    let json_resp: Value = response.json()?;

    // Extract the "ccg_trees" field which should be a JSON array.
    if let Some(ccg_trees) = json_resp.get("ccg_trees") {
        // Deserialize the field into a vector of strings.
        let trees: Vec<String> = serde_json::from_value(ccg_trees.clone())?;
        Ok(trees)
    } else {
        Err("Missing 'ccg_trees' field in response".into())
    }
}

fn json_to_ccgnode(json_strings : Vec<String>) -> Vec<(CCGNode, String)> {

    let mut v = Vec::new();

    for str in json_strings {
        // Read and parse the resulting JSON file into a CCGNode.
        let original_tree = serde_json::from_str(&str).expect("Failed to parse JSON to CCGNode.");
        v.push((original_tree, str.clone()))
    }

    v
}