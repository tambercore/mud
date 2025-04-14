use clap::{Arg, Command};



/// Structure to store configuration settings parsed from command-line arguments.
pub struct Config {
    pub knowledge: Vec<String>,
    pub conclusions: Vec<String>,
    pub output_file: String,
    pub server: bool,
}



impl Config {
    /// Function to parse command-line arguments and create a `Config` instance.
    pub fn from_args(default_sentence: &str) -> Self {
        let matches = Command::new("mudskip")
            .about("Mudskip provides a complete symbolic derivation from Natural Language Statements to Agda, using Lambeq, CCG, λ-Calculus, and Dependent Type Theory.")
            .arg(Arg::new("sentence")
                .short('i')
                .long("input")
                .value_name("TEXT")
                .help("Specify premises and conclusions, of form 'p1 & ... & pn -> c1 & ... & cn.' ")
                .num_args(1))
            .arg(Arg::new("server")
                .short('s')
                .long("server")
                .help("Run as a server")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Specify output file location (must have .lagda extension)")
                .default_value("output_file.lagda")
                .num_args(1))
            .arg(Arg::new("version")
                .short('v')
                .long("version")
                .help("Show version and exit")
                .action(clap::ArgAction::SetTrue))
            .get_matches();

        /* Handle `-v` or `--version` */
        if matches.get_flag("version") {
            println!("Mudskip, version 1.0.0");
            std::process::exit(0);
        }

        /* If "sentence" is not provided, use the default_sentence */
        let input = matches
            .get_one::<String>("sentence")
            .map(String::to_string)
            .unwrap_or_else(|| default_sentence.to_string());

        let (knowledge, conclusions) = parse_input(input).unwrap_or_else(|err| {
            eprintln!("Error parsing input: {}", err);
            std::process::exit(1);
        });

        /* Check if server flag is set */
        let server = matches.get_flag("server");

        let output_file = matches.get_one::<String>("output").unwrap().to_string();

        /* Validate that the output file has a .agda extension */
        if !output_file.ends_with(".lagda") {
            eprintln!("Error: Output file must have a .lagda extension.");
            std::process::exit(1);
        }

        Self { knowledge, conclusions, output_file, server }
    }
}



/// Function to parse an input string into lists of premises and conclusions.
pub fn parse_input(input: String) -> Result<(Vec<String>, Vec<String>), String> {
    let parts: Vec<&str> = input.split("->").collect();

    match parts.len() {
        1 => Ok((
            /* Split premises by '&' and trim whitespace */
            parts[0].split('&').map(|s| s.trim().to_string()).collect(),
            vec![]
        )),
        2 => Ok((
            /* Split premises and conclusions by '&' and trim whitespace */
            parts[0].split('&').map(|s| s.trim().to_string()).collect(),
            parts[1].split('&').map(|s| s.trim().to_string()).collect(),
        )),
        _ => Err("Invalid input format. Expected 'A & B -> C & D' or similar.".to_string()),
    }
}
