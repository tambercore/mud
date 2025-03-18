use std::io::{stdout, Write};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::sync::OnceLock;
use colored::*;


/// Structure to encapsulate a singular [`Task`], representing something in the
/// command line that is in progress, or has been done. This notes timings, as
/// a time taken for each task is provided.
struct Task {
    id: usize,
    depth: i32,
    name: String,
    start: Instant,
    end: Option<Instant>,
}



/// Structure to encapsulate a [`Progress`] representing a collection of tasks,
/// these are handled internally to the library (not relevant elsewhere).
pub struct Progress {
    tasks: Vec<Task>,
    next_id: usize,
}



/// Implementing some methods on the [`Progress`] structure, these are used
/// to manage, create and update [`Task`] instances.
impl Progress {


    /// Funtion to create a new [`Progress`], initializing tasks as
    /// an empty vector.
    pub fn new() -> Self {
        Progress {
            tasks: Vec::new(),
            next_id: 0,
        }
    }


    /// Function to start a new [`Task`], similar to a constructor for [`Task`].
    /// Depth indents the task display, and name is a nonexclusive task identifier.
    /// Returns a unique task ID as a [`usize`].
    pub fn new_task(&mut self, depth: i32, name: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let task = Task {
            id,
            depth,
            name: name.to_string(),
            start: Instant::now(),
            end: None,
        };
        self.tasks.push(task);
        self.print_last_task();
        id
    }


    /// Function that marks a [`Task`] as complete. If it is the most recent (i.e. at the bottom
    /// of the display), the module updates its line in place - otherwise it does not update.
    pub fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.end = Some(Instant::now());
            // Only update if this is the last printed task.
            if self.tasks.last().map(|t| t.id) == Some(id) {
                self.update_last_task();
            }
        }
    }


    /// Function to format time as `ms` or `s` dependent on size.
    fn format_time(duration: Duration) -> String {
        if duration.as_secs() >= 1 {
            format!("{}s", duration.as_secs())
        } else if duration.as_millis() >= 2 {
            format!("{}ms", duration.as_millis())
        } else {
            format!("{}µs", duration.as_micros())
        }
    }

    /// Function to print the last task (as in-progress). This formats the current task
    /// and provides `...` as a placeholder for the time taken. Prints in amber.
    fn print_last_task(&self) {
        if let Some(task) = self.tasks.last() {
            let indent = "  ".repeat(task.depth as usize);
            let arrow = "↳";
            let status = "\x1b[33m[In-Progress]\x1b[0m";
            let elapsed_placeholder = "...";
            println!(
                "{}{} {:<10} {:<40} {:>35}",
                indent, arrow, status, task.name, elapsed_placeholder
            );
            stdout().flush().unwrap();
        }
    }


    /// Function to update the last printed task. This formats the current task
    /// and provides the time taken in an appropriate unit. Prints in green.
    fn update_last_task(&self) {
        if let Some(task) = self.tasks.last() {
            // Move cursor up one line and clear it.
            print!("\x1B[1A\x1B[2K");
            let indent = "  ".repeat(task.depth as usize);
            let arrow = "↳";
            let status = "\x1b[32m[Done]      \x1b[0m";
            let elapsed = task
                .end
                .map(|end| {
                    let duration = end.duration_since(task.start);
                    Self::format_time(duration)
                })
                .unwrap_or_else(|| "...".to_string());
            println!(
                "{}{} {:<10} {:<40} {:>35}",
                indent, arrow, status, task.name, elapsed
            );
            stdout().flush().unwrap();
        }
    }
}



/// Progress is managed through the [`GLOBAL_PROGRESS`] instance, which saves having to pass
/// [`Progress`] objects around the entire program.
static GLOBAL_PROGRESS: OnceLock<Mutex<Progress>> = OnceLock::new();



/// Function to get the [`GLOBAL_PROGRESS`] instance (_static_ lifetime).
fn get_global_progress() -> &'static Mutex<Progress> {
    GLOBAL_PROGRESS.get_or_init(|| Mutex::new(Progress::new()))
}



/// Function to create a new [`Task`] in the global [`Progress`] instance.
pub fn create_task(depth: i32, name: &str) -> usize {
    let global_progress = get_global_progress();
    let mut prog = global_progress.lock().unwrap();
    prog.new_task(depth, name)
}



/// Function to update a [`Task`] in the global [`Progress`] instance.
pub fn update_task(id: usize) {
    let global_progress = get_global_progress();
    let mut prog = global_progress.lock().unwrap();
    prog.complete_task(id);
}



/// Function to display a formatted header with the [mud] prefix.
pub fn show_header(header: &str) {
    /*
     * \x1b[38;5;130m   sets a brown tone.
     * \x1b[1m          enables bold text.
     * \x1b[0m          resets all formatting.
     */
    println!("\x1b[38;5;130m[mud]\x1b[0m \x1b[1m{}\x1b[0m", header);
    stdout().flush().unwrap();
}



/*
Example Usage

fn main() {
    show_header("Initializing Brill Tagger");

    let lex_id = create_task(1, "Parsing Lexical Rules");
    std::thread::sleep(Duration::from_secs(1));
    update_task(lex_id);

    let context_id = create_task(1, "Parsing Contextual Rules");
    std::thread::sleep(Duration::from_millis(500));
    update_task(context_id);

    let doing_id = create_task(1, "Doing Task");
    std::thread::sleep(Duration::from_secs(1));
    update_task(doing_id);

}
 */