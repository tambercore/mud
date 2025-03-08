use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout, ChildStderr};
use std::io::{Write, BufRead, BufReader};
use serde_json::Value;

/* Starts Agda in interaction mode and returns the child process. */
fn start_agda() -> Child {
    Command::new("agda")
        .args(["--interaction-json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Agda")
}

fn send_command(stdin: &mut ChildStdin, stdout: &mut ChildStdout, command: &str, stop_key: &str) -> Vec<String> {
    writeln!(stdin, "{}", command).expect("Failed to write to Agda");
    stdin.flush().expect("Failed to flush stdin");

    let mut response = Vec::new();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.starts_with("JSON>") { // Agda sometimes prefixes output with JSON>
            response.push(line[5..].to_string());
        } else {
            response.push(line.clone());
        }
        println!("{}", line.clone());

        if let Ok(json) = serde_json::from_str::<Value>(&line) {
            // `agda-mode` is interactive, and the shell does not terminate on its own.
            // Once a given phrase (stop_key) is detected, return the fields.
            if json.get(stop_key).is_some() {
                break;
            }
        }
    }

    response
}


/* Parses Agda JSON response and extracts hole IDs. */
fn parse_hole_ids(response: Vec<String>) -> Vec<u32> {
    response.iter().filter_map(|line| {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            json.get("info")?.get("visibleGoals")?.as_array()?.iter()
                .filter_map(|goal| goal["constraintObj"]["id"].as_u64().map(|id| id as u32))
                .next()
        } else {
            None
        }
    }).collect()
}

/* Sends a command to fill a specific hole in an Agda file. */
fn fill_hole(stdin: &mut ChildStdin, stdout: &mut ChildStdout, file: String, hole_id: u32) -> Option<String> {
    let command = format!("IOTCM \"{}\" None Direct (Cmd_autoOne Simplified {} noRange \" -m \")", file, hole_id);
    let response = send_command(stdin, stdout, &command, "giveResult");

    response.iter().find_map(|line| {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            json.get("giveResult")?.get("str")?.as_str().map(String::from)
        } else {
            None
        }
    })
}

/* Process an Agda file, find and fill holes. */
pub fn fill_holes(filepath: String) {
    let mut agda = start_agda();
    let stdin = agda.stdin.as_mut().expect("Failed to get stdin");
    let stdout = agda.stdout.as_mut().expect("Failed to get stdout");

    let load_command = format!("IOTCM \"{}\" None Direct (Cmd_load \"{}\" [])", filepath, filepath);
    let response = send_command(stdin, stdout, &load_command, "info");
    let hole_ids = parse_hole_ids(response);

    if hole_ids.is_empty() {
        println!("No holes found in {}", filepath);
        return;
    }

    let filled_holes: Vec<String> = hole_ids.iter()
        .filter_map(|&hole_id| fill_hole(stdin, stdout, filepath.clone(), hole_id))
        .collect();

    println!("Filled holes: {:?}", filled_holes);

    agda.kill().expect("Failed to terminate Agda");
}