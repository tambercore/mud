use attohttpc;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::sync::RwLock;
use lazy_static::lazy_static;
use crate::ccg::node::CCGNode;
use anyhow::Result;

/// Global HashSets for storing parsed CCG structures and their JSON representations
lazy_static! {
    pub static ref SENTENCE_TO_CCG: RwLock<HashSet<(String, CCGNode)>> = RwLock::new(HashSet::new());
    pub static ref SENTENCE_TO_JSON: RwLock<HashSet<(String, String)>> = RwLock::new(HashSet::new());
}

/// Function to send sentences to the REST API and populate the global hashsets
pub fn sentences_to_ccg_hashsets(sentences: Vec<String>) -> Result<()> {
    let url = "http://127.0.0.1:20041/sentences"; /* Define the API endpoint */

    /* Construct the JSON payload with the "sentences" field */
    let payload = json!({ "sentences": sentences });
    // println!("payload: {}", payload);

    /* Send the POST request */
    let response = attohttpc::post(url)
        .header("Content-Type", "application/json")
        .json(&payload)?
        .send()?
        .error_for_status()?; // Ensures the response is an HTTP success code

    /* Parse the JSON response */
    let json_resp: Value = response.json()?;

    /* Extract the "ccg_trees" field which should be a JSON array */
    let ccg_trees = json_resp.get("ccg_trees")
        .ok_or_else(|| anyhow::anyhow!("Missing 'ccg_trees' field in response"))?
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid format for 'ccg_trees' field"))?;

    /* Get write access to the global hashsets */
    let mut ccg_set = SENTENCE_TO_CCG.write().unwrap();
    let mut json_set = SENTENCE_TO_JSON.write().unwrap();

    /* Clear existing data if needed (optional) */
    ccg_set.clear();
    json_set.clear();

    for (i, json_str) in ccg_trees.iter().enumerate() {
        /* Ensure JSON is a valid string */
        let json_str = serde_json::to_string(json_str)?; // Convert JSON object to a valid string

        /* Parse JSON into CCGNode */
        let original_tree: CCGNode = serde_json::from_str(&json_str)?;

        /* Get the original sentence */
        let sentence = sentences[i].clone();

        /* Insert into global hashsets */
        ccg_set.insert((sentence.clone(), original_tree));
        json_set.insert((sentence, json_str));
    }

    Ok(())
}
