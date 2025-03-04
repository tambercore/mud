use std::env;
use std::process::exit;

/* Configuration struct to hold command line arguments */
pub struct Config {
    pub sentence: String,
    pub help: bool,
    pub version: bool,
}

impl Config {

    /* Parse CLI and return a Config Instance */
    pub fn from_args(sentence: &str) -> Self {

        // `Sentence` is initialized in `main`. This is overwritten by the `-i` argument.
        let mut sentence = String::from(sentence);
        let mut help = false;
        let mut version = false;

        let args: Vec<String> = env::args().collect();
        let mut iter = args.iter().peekable();
        iter.next(); // Skip executable name

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-i" => {
                    if let Some(value) = iter.next() {
                        sentence = value.clone();
                    }
                }
                "-h" => help = true,
                "-v" => version = true,
                _ => continue,
            }
        }

        Self { sentence, help, version }
    }
}

/* Handle parsed arguments and display corresponding outputs */
pub fn handle_arguments(config: &Config) -> Result<(), i32> {
    if config.help {
        println!("Usage: mudskip [OPTIONS]\n\nOptions:\n  -i <sentence>  Specify input sentence\n  -h             Show this help message\n  -v             Show version");
        return Err(0);
    }
    if config.version {
        println!("Mudskip, version 1.0.0");
        return Err(0);
    }
    Ok(())
}