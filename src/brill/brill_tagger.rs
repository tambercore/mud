use std::collections::HashMap;
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use super::contextual_rulespec::{contextual_rule_apply, ContextualRulespec};
use super::wordclass::Wordclass;
use super::init_tagger::{initialize_tagger, WordclassMap};
use super::contractions::find_contractions;
use super::lex_rulespec_id::LexicalRulespec;
use super::lexical_rulespec::lexical_rule_apply;

pub fn get_sentence_tags(sentence: &str, wc_mapping: &mut WordclassMap) -> Vec<(String, Vec<Wordclass>)> {
    let tokenised_sentence = tokenize_sentence(sentence);
    get_possible_tags(tokenised_sentence, wc_mapping)
}

/// Function to tag a `sentence` using lexical and contextual rules.
pub fn tag_sentence(sentence: &str, lexical_ruleset: &Vec<LexicalRulespec>, contextual_ruleset: &HashMap<Wordclass, Vec<ContextualRulespec>>, wc_mapping: &mut WordclassMap) -> Vec<(String, Wordclass)> {

    // Tokenise sentence, and map each word to its possible tags.
    let tokenised_sentence = tokenize_sentence(sentence);
    let words_to_tags: Vec<(String, Vec<Wordclass>)> = get_possible_tags(tokenised_sentence, wc_mapping);

    /* Add possible tags to interpretations */
    /*for (word, tags) in words_to_tags.clone() {

        /* Join the tags with " or " */
        let tags_str = tags.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", or ");

        /* Format the statement */
        let statement = format!("'{}' may be {}", word, tags_str);

        /* Insert the interpretation */
        insert_interpretation(Interpretation {
            statement: statement.clone(),
            source: "Brill Tagger".to_string(),
        });
    }*/

    //println!("possible tags: {:?}", words_to_tags);
    let mut sentence_to_tag: Vec<(String, Wordclass)> = retrieve_sentence_to_tag(words_to_tags.clone());

    // Apply lexical and contextual rules.
    apply_lexical_rules(&mut sentence_to_tag, &lexical_ruleset, &words_to_tags, &wc_mapping, 10);
    apply_contextual_rules(&mut sentence_to_tag, &words_to_tags, &contextual_ruleset, 100).ok_or("Max iterations reached in contextual rules");




    // After applying all rules, change any `Wordclass::ANY` tags to `Wordclass::NN`
    sentence_to_tag.iter_mut().for_each(|(_, tag)| {
        if *tag == Wordclass::ANY {
            *tag = Wordclass::NNP;
        }
    });

    /* Add tags to interpretations */
    /*for (word, tag) in sentence_to_tag.clone() {

        /* Format the statement */
        let statement = format!("'{}' is a {}", word, tag);

        /* Insert the interpretation */
        insert_interpretation(Interpretation {
            statement: statement.clone(),
            source: "Brill Tagger".to_string(),
        });
    }*/


    return sentence_to_tag;
}




/// Apply lexical rules to a sentence `sentence_to_tag`
fn apply_lexical_rules(sentence_to_tag: &mut Vec<(String, Wordclass)>, lexical_ruleset: &Vec<LexicalRulespec>, possible_tags: &Vec<(String, Vec<Wordclass>)>, wc_mapping: &WordclassMap, max_iterations: i32) {

    let mut iterations = 0;
    loop {
        let mut rules_applied = 0;
        for (index, (word, tag)) in sentence_to_tag.clone().iter().enumerate() {
            for rule in lexical_ruleset {

                if !is_tag_contained_in_word_possible_tags(&possible_tags, &word, &rule.target_tag) { continue; }
                match lexical_rule_apply(sentence_to_tag, index as i32, rule, wc_mapping){
                    Some(true) => {
                        //println!("LEXICAL {:?} tagged {:?} -> tag {:?}: ",word, tag, &rule.target_tag);
                        rules_applied += 1},
                    _ => {},
                }
            }
        }
        //if are_tags_valid(&sentence_to_tag, &possible_tags) {return Some(true)}
        if iterations == max_iterations || rules_applied == 0 {return}
        iterations +=1;
    }

}


/// Continuously apply contextual rules to a sentence `sentence_to_tag` until each word's tag is in `possible_tags` or no rules were applied.
fn apply_contextual_rules(sentence_to_tag: &mut Vec<(String, Wordclass)>, possible_tags: &Vec<(String, Vec<Wordclass>)>, contextual_ruleset: &HashMap<Wordclass, Vec<ContextualRulespec>>, threshold:i32) -> Option<bool> {
    let mut iterations = 0;
    loop {
        let mut rules_applied = 0;
        for (index, (word, tag)) in sentence_to_tag.clone().iter().enumerate() {
            let valid_rules = contextual_ruleset.get(&tag);
            match valid_rules {
                Some(_valid_rules) => {
                    for rule in _valid_rules {
                        if !is_tag_contained_in_word_possible_tags(possible_tags, &word, &rule.target_tag) {continue;}
                        match contextual_rule_apply(sentence_to_tag, index as i32, rule.clone()) {
                            Some(true) => {
                                println!("CONTEXTUAL {:?} tagged {:?} -> tag {:?}: ",word, tag, &rule.target_tag);
                                rules_applied += 1},
                            _ => {},
                        }
                    }
                }
                // Some Wordclasses have no associated rules (e.g. CC) - in this case, the tag is kept.
                None => continue
            }
        }
        //if are_tags_valid(&sentence_to_tag, &possible_tags) {return Some(true)}
        if iterations == threshold || rules_applied == 0 {return Some(true)}
        iterations +=1;
    }

}


/// Function to take a `sentence` (&str), split whitespace and tokenize any contractions.
fn tokenize_sentence(sentence: &str) -> Vec<String> {
    sentence.split_whitespace().
        map(|word|find_contractions(String::from(word)).unwrap())
        .flatten()
        .collect()
}


/// Function to: given a tokenized `sentence` and mapping `wc_mapping`, retrieve the possible tags for each word.
pub fn get_possible_tags(sentence: Vec<String>, wc_mapping: &mut WordclassMap) -> Vec<(String, Vec<Wordclass>)> {
    sentence.iter()
        .map(|word| (word.as_str().to_owned(), wc_mapping.entry(word.as_str().parse().unwrap()).or_insert(vec![Wordclass::ANY]).to_owned()))
        .collect()
}


/// Function to alter the first tag of the word's possible tags. Retrieve this tag for each word.
fn retrieve_sentence_to_tag(sentence: Vec<(String, Vec<Wordclass>)>) -> Vec<(String, Wordclass)> {
    sentence
        .iter()
        .filter_map(|(word, tags)| tags.first().map(|first_tag| (word.to_owned(), first_tag.clone()))).collect()
}


/// Function to check if `possible_tag`s of a given `word` contain `target_tag`.
fn is_tag_contained_in_word_possible_tags(possible_tags: &Vec<(String, Vec<Wordclass>)>, word: &String, target_tag: &Wordclass) -> bool {
    let possible_tags_for_word =     possible_tags.iter()
        .find(|(first, _)| first == word)
        .map(|(_, second)| second).unwrap();

    possible_tags_for_word.contains(target_tag) || possible_tags_for_word.contains(&Wordclass::ANY)
}


/// Function to check if all tags in a `sentence` are contained in their list of `possible_tags`.
fn are_tags_valid(sentence: &Vec<(String, Wordclass)>, possible_tags: &Vec<(String, Vec<Wordclass>)>) -> bool {
    sentence.iter().all(|(word, tag)| {
        possible_tags
            .iter()
            .find(|(w, _)| w == word)
            .map_or(false, |(_, tags)|

                // This asserts, that for each word in the sentence, the assigned `tag` must exist in
                // the lexicon entry for the `word`. If the words lexicon entry contains `Wordclass::ANY`
                // then any tag is valid. Additionally, the final tag itself cannot be `Wordclass::ANY`.
                (tags.contains(tag) || tags.contains(&Wordclass::ANY)) && *tag != Wordclass::ANY)
    })
}

#[test]
fn test_tag_sentence() {
    // To do proper tests, need to know what the sentences should be tagged as!
    // Parse rulesets and lexicon.
    let lexical_ruleset: Vec<LexicalRulespec> = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();
    let contextual_ruleset: HashMap<Wordclass, Vec<ContextualRulespec>> = parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap();
    let mut wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();

    tag_sentence("All ravens are black", &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);
}