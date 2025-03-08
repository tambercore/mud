use std::{fs, io};
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

fn send_command(stdin: &mut ChildStdin, stdout: &mut ChildStdout, command: &str, stop_keys: Vec<&str>) -> Vec<String> {
    writeln!(stdin, "{}", command).expect("Failed to write to Agda");
    stdin.flush().expect("Failed to flush stdin");

    let mut response = Vec::new();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let mut line = line.expect("Failed to read line");

        // Debugging the output line
        println!("Received line: {}", line);

        // Agda sometimes prefixes output with JSON>
        if line.contains("JSON>") {
            line = line.replace("JSON>", "");
            response.push(line.clone());
        } else {
            response.push(line.clone());
        }

        // Attempt to parse JSON from the line
        if let Ok(json) = serde_json::from_str::<Value>(&line) {
            println!("\n\nParsed JSON: {json}\n\n");

            // Check if any of the stop keys are in the parsed JSON
            if stop_keys.iter().any(|&key| json.get(key).is_some()) {
                println!("One of the stop keys detected, exiting loop.");
                break;
            }
        } else {
            // If parsing JSON fails, log the line to debug
            println!("Failed to parse line as JSON: {}", line);
        }
    }

    println!("done");

    response
}



/* Parses Agda JSON response and extracts hole IDs. */
fn parse_holes(response: Vec<String>) -> Vec<(u32, (i32, i32))> {
    println!("PARSING - {:?}", response);

    response.iter().filter_map(|line| {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            Some(json.get("info")?.get("visibleGoals")?.as_array()?.iter()
                .filter_map(|goal| {
                    let id = goal["constraintObj"]["id"].as_u64()? as u32;
                    let line = goal["constraintObj"]["range"][0]["start"]["line"].as_i64()? as i32;
                    let col = goal["constraintObj"]["range"][0]["start"]["col"].as_i64()? as i32;
                    Some((id, (line, col)))
                })
                .collect::<Vec<_>>())
        } else {
            None
        }
    }).flatten()
        .collect()
}


/* Sends a command to fill a specific hole in an Agda file. */
fn fill_hole(stdin: &mut ChildStdin, stdout: &mut ChildStdout, file: String, hole_id: u32) -> Option<String> {
    let command = format!("IOTCM \"{}\" None Direct (Cmd_autoOne Simplified {} noRange \" -m \")", file, hole_id);
    let response = send_command(stdin, stdout, &command, vec!["giveResult", "info"]);

    response.iter().find_map(|line| {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            json.get("giveResult")?.get("str")?.as_str().map(String::from)
        } else {
            None
        }
    })
}

fn replace_holes_in_file(filepath: &str, filled_holes: &Vec<Option<String>>) -> io::Result<()> {
    // Get the positions of all '?' in the file
    let positions = find_question_mark_positions(filepath)?;
    println!("HOLE POSITIONS: {:?}", positions);

    // Ensure there are enough filled holes to replace the '?' marks
    if filled_holes.len() != positions.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Number of holes and filled values do not match",
        ));
    }

    let mut file_content = fs::read_to_string(filepath)?;
    let mut updated_content = String::new();
    let mut last_pos = 0; // Keeps track of the last position we processed in the content

    // Iterate through each hole position and replace '?' with corresponding filled value
    for (index, &(line_number, col_number)) in positions.iter().enumerate() {
        if let Some(hole) = &filled_holes[index] {
            // Find the start of the current line
            let line_start = file_content.lines().take(line_number as usize - 1).map(|line| line.len() + 1).sum::<usize>();

            // Append all content up to the current hole
            updated_content.push_str(&file_content[last_pos..line_start + col_number as usize - 1]);

            // Replace the '?' with the corresponding filled value
            updated_content.push_str(hole);

            // Update the last processed position
            last_pos = line_start + col_number as usize;
        }
    }

    // Append the remaining content after the last hole
    updated_content.push_str(&file_content[last_pos..]);

    // Write the updated content back to the file
    let mut file = fs::File::create("output_filled.agda")?;
    file.write_all(updated_content.as_bytes())?;
    file.flush()?;

    println!("Updated file with filled holes");
    Ok(())
}


fn find_question_mark_positions(filepath: &str) -> io::Result<Vec<(i32, i32)>> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut positions = Vec::new();

    // Iterate over the lines of the file
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        // Search for the '?' character in the line
        if let Some(pos) = line.find('?') {
            positions.push((line_number as i32 + 1, pos as i32 + 1)); // 1-based line and column
        }
    }

    Ok(positions)
}

/* Process an Agda file, find and fill holes. */
pub fn fill_holes(filepath: String) {
    let mut agda = start_agda();
    let stdin = agda.stdin.as_mut().expect("Failed to get stdin");
    let stdout = agda.stdout.as_mut().expect("Failed to get stdout");

    let load_command = format!("IOTCM \"{}\" None Direct (Cmd_load \"{}\" [])", filepath, filepath);
    let response = send_command(stdin, stdout, &load_command, vec!["info"]);
    let hole_info = parse_holes(response);

    println!("HOLE INFO: {:?}", hole_info.clone());

    if hole_info.is_empty() {
        println!("No holes found in {}", filepath);
        return;
    }

    let filled_holes: Vec<Option<String>> = hole_info.iter()
        .map(|&hole_id| fill_hole(stdin, stdout, filepath.clone(), hole_id.0))
        .collect();

    println!("Filled holes: {:?}", filled_holes);

    if let Err(e) = replace_holes_in_file(&filepath, &filled_holes) {
        eprintln!("Error updating file: {}", e);
    }

    agda.kill().expect("Failed to terminate Agda");
}
