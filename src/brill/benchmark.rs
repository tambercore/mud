use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufWriter;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
//use std::fs::File;
use super::wordclass::Wordclass;
use super::conllu_parser::{parse_conllu_file, UPOS}; // Import your custom parser
use super::brill_tagger::tag_sentence;
use super::contextual_rulespec::ContextualRulespec;
use super::lex_rulespec_id::LexicalRulespec;
use super::init_tagger::WordclassMap;


pub fn wordclass_to_upos(wordclass: &Wordclass) -> UPOS {
    match wordclass {
        Wordclass::CC    => UPOS::CCONJ, // Coordinating conjunction
        Wordclass::CD    => UPOS::NUM,   // Cardinal number
        Wordclass::DT    => UPOS::DET,   // Determiner
        Wordclass::FW    => UPOS::X,     // Foreign word
        Wordclass::IN    => UPOS::SCONJORADP,   // Preposition or subordinating conjunction
        Wordclass::JJ    => UPOS::ADJ,   // Adjective
        Wordclass::JJR   => UPOS::ADJ,   // Comparative adjective
        Wordclass::JJS   => UPOS::ADJ,   // Superlative adjective
        Wordclass::LS    => UPOS::X,     // List marker
        Wordclass::MD    => UPOS::VERB,   // Modal verb
        Wordclass::NN    => UPOS::NOUN,  // Noun
        Wordclass::NNS   => UPOS::NOUN,  // Plural noun
        Wordclass::NNP   => UPOS::PROPN,  // Proper noun
        Wordclass::NNPS  => UPOS::PROPN,  // Plural proper noun
        Wordclass::PDT   => UPOS::DET,   // Predeterminer
        Wordclass::POS   =>UPOS::PART,  // Possessive ending
        Wordclass::RB    => UPOS::ADV,   // Adverb
        Wordclass::RBR   => UPOS::ADV,   // Comparative adverb
        Wordclass::RBS   => UPOS::ADV,   // Superlative adverb
        Wordclass::RP    => UPOS::PART,  // Particle
        Wordclass::SYM   => UPOS::SYM,   // Symbol
        Wordclass::TO    => UPOS::PART,  // Infinitive marker (to)
        Wordclass::UH    => UPOS::INTJ,  // Interjection
        Wordclass::VB    => UPOS::VERB,   // Verb
        Wordclass::VBD   => UPOS::VERB,   // Past tense verb
        Wordclass::VBG   => UPOS::VERB,   // Gerund/Present participle
        Wordclass::VBN   => UPOS::VERB,   // Past participle
        Wordclass::WDT   => UPOS::PRON,   // Wh-determiner
        Wordclass::WPR   => UPOS::PRON,   // Wh-pronoun
        Wordclass::WPO   => UPOS::PRON,   // Wh-other pronoun
        Wordclass::WRB   => UPOS::ADV,    // Wh-adverb
        Wordclass::PUNC  => UPOS::PUNCT,  // Punctuation
        Wordclass::EX    => UPOS::ADV,    // Existential "there"
        Wordclass::PRPO  => UPOS::PRON,   // Possessive pronoun
        Wordclass::PRPE  => UPOS::PRON,   // Personal pronoun
        Wordclass::VBP   => UPOS::VERB,    // Present tense verb
        Wordclass::VBZ   => UPOS::VERB,    // 3rd-person singular verb
        Wordclass::NUM   => UPOS::NUM,    // Numeral
        Wordclass::ANY   => UPOS::PROPN,   // Ambiguous, may need clarification in usage
    }
}

#[derive(Serialize)]  // Enable CSV serialization for the struct
struct BenchmarkData {
    sentence_id: usize,
    original_word: String,
    original_upos: String,
    predicted_word: String,
    predicted_tag: String,
    match_status: bool,
}

#[derive(Debug, Deserialize)]
struct CsvRow {
    sentence_id: usize,
    original_word: String,
    original_upos: String,
    predicted_word: String,
    predicted_tag: String,
    match_status: bool,
}

pub fn benchmark_pos_tagger(conllu_filepath: &str, lexical_ruleset: &Vec<LexicalRulespec>, contextual_ruleset: &HashMap<Wordclass, Vec<ContextualRulespec>>, wc_mapping: &mut WordclassMap) -> f32 {
    // Open the file and create a buffered reader
    let sentences = parse_conllu_file(conllu_filepath).expect("Failed to parse file");

    let total_sentences = sentences.len();
    println!("Total sentences: {}", total_sentences);

    let mut total_score = 0.0;
    let mut sentence_count = 0;
    let max_sentences = 100; // Limit to 100 sentences for quicker benchmarking.

    // Vector to store all benchmark data for analysis
    let mut benchmark_data: Vec<BenchmarkData> = Vec::new();

    for (i, sentence) in sentences.iter().enumerate() {
        let str_sentence: String = sentence.iter()
            .map(|token| token.form.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        let tagged_sentence = tag_sentence(&str_sentence, &lexical_ruleset, &contextual_ruleset, wc_mapping);

        println!("\nSentence {} score:", i + 1);
        println!("{:<20} | {:<20} | {:<15} | {:<10} | {}",
                 "Original Word", "Original UPOS", "Predicted Word", "Predicted Tag", "Match");
        println!("{}", "-".repeat(80));

        let mut matches = 0;
        let total_tokens = sentence.len();

        for (token, (word, tag)) in sentence.iter().zip(tagged_sentence.iter()) {
            let predicted_upos = wordclass_to_upos(tag).clone();
            let correct = token.upos == Some(predicted_upos.clone());

            if correct {
                matches += 1;
            }

            let original_upos = match token.clone().upos {
                Some(upos) => format!("{:?}", upos),
                None => "None".to_string(),
            };

            // Print the comparison
            println!("{:<20} | {:<20} | {:<15} | {:<10?} | {}",
                     token.form,
                     original_upos,
                     word,
                     predicted_upos,
                     if correct { "✔" } else { "✘" });

            // Store data for this token in the benchmark data vector
            benchmark_data.push(BenchmarkData {
                sentence_id: i + 1,
                original_word: token.form.clone(),
                original_upos: original_upos.clone(),
                predicted_word: word.clone(),
                predicted_tag: format!("{:?}", predicted_upos.clone()),
                match_status: correct,
            });
        }

        let sentence_score = matches as f32 / total_tokens as f32;
        total_score += sentence_score;
        sentence_count += 1;

        println!("\nSentence {} (of {}) match score: {:.2}\n", i + 1, total_sentences, sentence_score);
        println!("{}", "=".repeat(80));

        //if sentence_count > max_sentences { break; }
    }

    // Calculate the average score
    let avg_score = if sentence_count > 0 {
        total_score / sentence_count as f32
    } else {
        0.0
    };

    println!("Average match score: {:.2}", avg_score);

    // Write the benchmark data to a CSV file for analysis
    save_benchmark_data_to_csv("pos_benchmark_results.csv", &benchmark_data);

    avg_score
}



// Function to analyze a CSV of tagging results
pub fn analyze_pos_csv(filepath: &str) {
    // Open the CSV file
    let file = File::open(filepath).expect("Failed to open CSV file");
    let mut reader = ReaderBuilder::new()
        .has_headers(true) // The CSV contains headers
        .from_reader(file);

    // Variables to track analysis results
    let mut total_tokens = 0;
    let mut correct_tokens = 0;
    let mut incorrect_tokens = 0;
    let mut mistagged_counts: HashMap<(String, String), usize> = HashMap::new(); // (original_tag, predicted_tag) -> count

    // Iterate through each row in the CSV
    for result in reader.deserialize() {
        let row: CsvRow = result.expect("Failed to parse CSV row");

        // Update total token count
        total_tokens += 1;

        // Check if the token was correctly tagged
        if row.match_status {
            correct_tokens += 1;
        } else {
            incorrect_tokens += 1;
            // Track the original and predicted tag pair for mistagging analysis
            let key = (row.original_upos.clone(), row.predicted_tag.clone());
            let counter = mistagged_counts.entry(key).or_insert(0);
            *counter += 1;
        }
    }

    // Calculate accuracy
    let accuracy = (correct_tokens as f32 / total_tokens as f32) * 100.0;

    // Print summary
    println!("Total tokens: {}", total_tokens);
    println!("Correctly tagged tokens: {} ({:.2}%)", correct_tokens, accuracy);
    println!("Incorrectly tagged tokens: {}", incorrect_tokens);

    // Print top mistagged pairs
    println!("\nTop 5 most frequent mistagged pairs (Original -> Predicted):");
    let mut sorted_mistagged: Vec<((String, String), usize)> = mistagged_counts.into_iter().collect();
    sorted_mistagged.sort_by(|a, b| b.1.cmp(&a.1));  // Sort by frequency, descending

    for ((original, predicted), count) in sorted_mistagged.iter().take(5) {
        println!("{} -> {}: {} times", original, predicted, count);
    }
}
// Function to write benchmark data to a CSV file
fn save_benchmark_data_to_csv(filepath: &str, data: &Vec<BenchmarkData>) {
    let file = File::create(filepath).expect("Unable to create file");
    let writer = BufWriter::new(file);

    let mut csv_writer = csv::Writer::from_writer(writer);

    // Write the header and the data
    for entry in data {
        csv_writer.serialize(entry).expect("Failed to write data");
    }

    csv_writer.flush().expect("Failed to flush data");
}

/*
#[test]
fn analyse_csv() {
    let filepath: &str = "pos_benchmark_results.csv";
    analyze_pos_csv(filepath);
}
*/