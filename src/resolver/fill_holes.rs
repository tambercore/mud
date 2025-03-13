use std::{fs, io};
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::io::{Write, BufRead, BufReader};
use serde_json::Value;

/// Function to start Agda in interaction mode with JSON communication, and return the child process.
fn start_agda() -> Child {
    Command::new("agda")
        .args(["--interaction-json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Agda")
}



/// Function to send a command to Agda, wait for the response, and check for the presence of stop keys in the JSON response.
/// The function returns the response lines received until the stop keys are found.
fn send_command(stdin: &mut ChildStdin, stdout: &mut ChildStdout, command: &str, stop_keys: Vec<&str>) -> Vec<String> {
    writeln!(stdin, "{}", command).expect("Failed to write to Agda");
    stdin.flush().expect("Failed to flush stdin");

    let mut response = Vec::new();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        /* Responses in agda-mode begin with `JSON>`, which should be ignored. */
        let mut line = if line.contains("JSON>") { line.replace("JSON>", "") } else { line.clone() };
        response.push(line.clone());

        if let Ok(json) = serde_json::from_str::<Value>(&line) {
            /* Agda-mode is an interactive shell which does not terminate, therefore
               the function returns once valid JSON is parsed, which contains a certain key (in stop_keys). */
            if stop_keys.iter().any(|&key| json.get(key).is_some()) {
                break;
            }
        }
    }

    response
}



/// Function to parse the response from Agda to extract hole IDs and their respective positions (line, column).
/// Returns a vector of tuples containing hole IDs and their positions.
fn parse_holes(response: Vec<String>) -> Vec<(u32, (i32, i32))> {
    response.iter().filter_map(|line| {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            /* Retrieve the list of holes. */
            Some(json.get("info")?.get("visibleGoals")?.as_array()?
                .iter()
                .filter_map(|goal| {
                    /* Retrieve a hole's id (for filling), and its position (line, col). */
                    let id = goal["constraintObj"]["id"].as_u64()? as u32;
                    let line = goal["constraintObj"]["range"][0]["start"]["line"].as_i64()? as i32;
                    let col = goal["constraintObj"]["range"][0]["start"]["col"].as_i64()? as i32;
                    Some((id, (line, col)))
                })
                .collect::<Vec<_>>())
        } else { None }
    }).flatten().collect()
}



/// Function to send a command to Agda to fill a hole (identified by hole_id) in the given file.
/// The function returns the filled content or `None` if the hole couldn't be filled.
fn fill_hole(stdin: &mut ChildStdin, stdout: &mut ChildStdout, file: String, hole_id: u32) -> Option<String> {
    /* Command to fill in a hole (hole_id) in a given file. Uses parameter -m. */
    let command = format!("IOTCM \"{}\" None Direct (Cmd_autoOne Simplified {} noRange \" -m \")", file, hole_id);
    /* If a hole is filled, the `giveResult` field contains the filled in hole. Else, the `info` field contains an error message. */
    send_command(stdin, stdout, &command, vec!["giveResult", "info"])
        .iter()
        .find_map(|line| serde_json::from_str::<Value>(line).ok().and_then(|json| json.get("giveResult")?.get("str")?.as_str().map(String::from)))
}



/// Function to find the positions of '?' marks in an Agda file.
/// The function returns a vector of tuples containing line and column numbers.
fn find_question_mark_positions(filepath: &str) ->Vec<(i32, i32)> {
    let file = fs::File::open(filepath).expect("Failed to open file.");
    let reader = BufReader::new(file);
    let mut positions = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        if let Some(pos) = line.expect("Failed to parse line.").find('?') {
            positions.push((line_number as i32 + 1, pos as i32 + 1));
        }
    }

    positions
}



/// Function to replace the '?' marks in the file with the corresponding filled hole values.
/// It returns the updated content of the file as a string.
fn replace_holes_in_file(filepath: &str, filled_holes: &Vec<Option<String>>) -> String {
    let positions = find_question_mark_positions(filepath);
    if filled_holes.len() != positions.len() {
        panic!("Number of holes and filled values do not match");
    }

    let mut file_content = fs::read_to_string(filepath).expect("Failed to read file.");
    let mut updated_content = String::new();
    let mut last_pos = 0;

    for (index, &(line_number, col_number)) in positions.iter().enumerate() {
        if let Some(hole) = &filled_holes[index] {
            /* Find the position of the hole in the file. */
            let line_start = file_content.lines().take(line_number as usize - 1).map(|line| line.len() + 1).sum::<usize>();
            /* Replace the `?` with the filled in hole. */
            updated_content.push_str(&file_content[last_pos..line_start + col_number as usize - 1]);
            updated_content.push_str(hole);
            /* Update the position to continue editing. */
            last_pos = line_start + col_number as usize;
        }
    }

    updated_content.push_str(&file_content[last_pos..]);

    updated_content
}



/// Function to fill holes in the Agda file.
/// This function will load the file, parse and fill holes, and update the file with filled content.
pub fn fill_holes(filepath: String) {
    let mut agda = start_agda();
    let stdin = agda.stdin.as_mut().expect("Failed to get stdin");
    let stdout = agda.stdout.as_mut().expect("Failed to get stdout");

    /* Load the Agda file and parse for visible holes */
    let load_command = format!("IOTCM \"{}\" None Direct (Cmd_load \"{}\" [])", filepath, filepath);
    let response = send_command(stdin, stdout, &load_command, vec!["info"]);
    let hole_info = parse_holes(response);

    if hole_info.is_empty() {
        println!("No holes found in {}", filepath);
        return;
    }

    /* Attempt to fill in each hole */
    let filled_holes: Vec<Option<String>> = hole_info.iter()
        .map(|&hole_id| fill_hole(stdin, stdout, filepath.clone(), hole_id.0))
        .collect();

    /* Update the file with holes filled in. */
    let updated_content = replace_holes_in_file(&filepath, &filled_holes);
    fs::write(filepath, updated_content);
    println!("Updated file with filled holes");

    agda.kill().expect("Failed to terminate Agda");
}
