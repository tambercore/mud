use clap::{Arg, Command};

pub struct Config {
    pub sentence: String,
    pub output_file: String,
}

impl Config {
    pub fn from_args(default_sentence: &str) -> Self {
        let matches = Command::new("mudskip")
            .about("A tool for processing sentences and outputting to Agda files.")
            .arg(Arg::new("sentence")
                .short('i')
                .long("input")
                .value_name("TEXT")
                .help("Specify input sentence")
                .num_args(1))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Specify output file location (must have .agda extension)")
                .default_value("output_file.agda")
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
        let sentence = matches
            .get_one::<String>("sentence")
            .map(String::to_string)
            .unwrap_or_else(|| default_sentence.to_string());

        let output_file = matches.get_one::<String>("output").unwrap().to_string();

        if !output_file.ends_with(".agda") {
            eprintln!("Error: Output file must have a .agda extension.");
            std::process::exit(1);
        }

        Self { sentence, output_file }
    }
}
