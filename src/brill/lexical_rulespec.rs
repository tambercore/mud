use super::wordclass::{map_pos_tag, Wordclass};
use super::init_tagger::WordclassMap;
use super::lex_rulespec_id::{LexicalRuleID, LexicalRulespec};

/// Function to check if the word at `current_index` has suffix `suffix` and is not yet tagged.
pub fn has_suffix(sentence: &Vec<(String, Wordclass)>, current_index: i32, suffix: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some((word, Wordclass::ANY)) => word.ends_with(suffix),
        _ => false,
    }
}


/// Function to check if the word at `current_index` has suffix `suffix` and is tagged as `target_tag`.
pub fn f_has_suffix(sentence: &Vec<(String, Wordclass)>, current_index: i32, suffix: &str, source_tag: Wordclass) -> bool {
    match sentence.clone().get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some((word, ref tag)) => word.ends_with(suffix) && tag.to_owned() == source_tag,
        _ => false,
    }
}


/// Function to check if the word at `current_index` has suffix `prefix` and is not yet tagged.
pub fn has_prefix(sentence: &Vec<(String, Wordclass)>, current_index: i32, prefix: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some((word, Wordclass::ANY)) => word.starts_with(prefix),
        _ => false,
    }
}


/// Function to check if the word at `current_index` has suffix `prefix` and is tagged as `target_tag`.
pub fn f_has_prefix(sentence: &Vec<(String, Wordclass)>, current_index: i32, prefix: &str, source_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some((_, Wordclass::ANY)) => false,
        Some((word, ref tag)) => word.starts_with(prefix) && tag.to_owned() == source_tag,
        _ => false,
    }
}


/// Function to check if the word at `current_index` contains char `c` and is not yet tagged.
pub fn has_char(sentence: &Vec<(String, Wordclass)>, current_index: i32, c: char) -> bool {
    match sentence.get(current_index as usize) {
        Some((word, Wordclass::ANY)) => word.contains(c),
        _ => false,
    }
}


/// Function to check if the word at `current_index` contains char `c` and is tagged.
pub fn f_has_char(sentence: &Vec<(String, Wordclass)>, current_index: i32, c: char, source_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some((_, Wordclass::ANY)) => false,
        Some((word, ref tag)) => word.contains(c) && tag.to_owned() == source_tag,
        _ => false,
    }
}


/// Function to check if the word at `current_index` is still a word if `suffix` is added, and is not yet tagged.
pub fn add_suffix(sentence: &Vec<(String, Wordclass)>, current_index: i32, suffix: &str, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some((word, Wordclass::ANY)) => {
            let modified_word = word.to_string() + suffix;
            is_word_in_lexicon(modified_word, wc_mapping)
        },
        _ => false,
    }
}


/// Function to check if the word at `current_index` is still a word if `suffix` is added, and is tagged.
pub fn f_add_suffix(sentence: &Vec<(String, Wordclass)>, current_index: i32, suffix: &str, source_tag: Wordclass, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some((_, Wordclass::ANY)) => false,
        Some((word, ref tag)) => {
            let modified_word = word.to_string() + suffix;
            is_word_in_lexicon(modified_word, wc_mapping) && tag.to_owned() == source_tag
        },
        _ => false,
    }
}


/// Function to check if the word at `current_index` is still a word if `suffix` is deleted, and is not yet tagged.
pub fn delete_suffix(sentence: &Vec<(String, Wordclass)>, current_index: i32, suffix: &str, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some((word, Wordclass::ANY)) => {
            match word.strip_suffix(suffix) {
                Some(modified_word) => is_word_in_lexicon(String::from(modified_word), wc_mapping),
                _ => false
            }
        },
        _ => false,
    }
}


/// Function to check if the word at `current_index` is still a word if `suffix` is deleted, and is tagged.
pub fn f_delete_suffix(sentence: &Vec<(String, Wordclass)>, current_index: i32, suffix: &str, source_tag: Wordclass, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some((_, Wordclass::ANY)) => false,
        Some((word, ref tag)) => {
            match word.strip_suffix(suffix) {
                Some(modified_word) => is_word_in_lexicon(String::from(modified_word), wc_mapping) && tag.to_owned() == source_tag,
                _ => false
            }
        },
        _ => false,
    }
}


/// Function to check if the word at `current_index` is still a word if `prefix` is deleted, and is not yet tagged.
pub fn delete_prefix(sentence: &Vec<(String, Wordclass)>, current_index: i32, prefix: &str, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some((word, Wordclass::ANY)) => {
            match word.strip_prefix(prefix) {
                Some(modified_word) => is_word_in_lexicon(String::from(modified_word), wc_mapping),
                _ => false
            }
        },
        _ => false,
    }
}


/// Function to check if the word at `current_index` is still a word if `prefix` is deleted, and is tagged.
pub fn f_delete_prefix(sentence: &Vec<(String, Wordclass)>, current_index: i32, prefix: &str, source_tag: Wordclass, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some((_, Wordclass::ANY)) => false,
        Some((word, ref tag)) => {
            match word.strip_prefix(prefix) {
                Some(modified_word) => is_word_in_lexicon(String::from(modified_word), wc_mapping) && tag.to_owned() == source_tag,
                _ => false
            }
        },
        _ => false,
    }
}


/// Function to check if the word to the left of the word at `current_index` is `word` and is not yet tagged.
pub fn appears_to_left(sentence: &Vec<(String, Wordclass)>, current_index: i32, expected_word: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some((_, Wordclass::ANY)) => {
            match sentence.get((current_index - 1) as usize) {
                Some((word, _)) => word == expected_word,
                _ => false,
            }
        }
        _ => false
    }

}


/// Function to check if the word to the left of the word at `current_index` is `word` and is tagged.
pub fn f_appears_to_left(sentence: &Vec<(String, Wordclass)>, current_index: i32, expected_word: &str, source_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(_, ref tag)) => {
            match sentence.get((current_index - 1) as usize) {

                Some((word, _)) => word == expected_word && tag.to_owned() == source_tag,
                _ => false,
            }
        }
        _ => false
    }

}


/// Function to check if the word to the right of the word at `current_index` is `word` and is not yet tagged.
pub fn appears_to_right(sentence: &Vec<(String, Wordclass)>, current_index: i32, expected_word: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => {
            match sentence.get((current_index + 1) as usize) {
                Some((word, _)) => word == expected_word,
                _ => false,
            }
        }
        _ => false
    }

}


/// Function to check if the word to the right of the word at `current_index` is `word` and is tagged.
pub fn f_appears_to_right(sentence: &Vec<(String, Wordclass)>, current_index: i32, expected_word: &str, source_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(_, ref tag)) => {
            match sentence.get((current_index + 1) as usize) {

                Some((word, _)) => word == expected_word && tag.to_owned() == source_tag,
                _ => false,
            }
        }
        _ => false
    }

}


/// Function to check if `word` appears in the Wordclass mappings retrieved from the lexicon.
pub fn is_word_in_lexicon(word: String, wc_mapping: &WordclassMap) -> bool {
    match wc_mapping.get(&word) {
        Some(_) => true,
        _ => false
    }
}


/// Checks a given lexical rule.
pub fn lexical_rule_holds(sentence: &mut Vec<(String, Wordclass)>, current_index: i32, rule: &LexicalRulespec, wc_mapping: &WordclassMap) -> Option<bool> {



    match rule.ruleset_id {
        LexicalRuleID::HASSUF => {

            let suffix: &str = rule.parameters.get(0)?;
            //println!("applying rule: {:?}", rule);
            Option::from(has_suffix(&sentence, current_index, suffix))

        }
        LexicalRuleID::FCHAR => {
            let c = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_has_char(&sentence, current_index, c.parse().unwrap(), _wordclass)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::ADDSUF => {
            let suffix = rule.parameters.get(0)?;
            Option::from(add_suffix(&sentence, current_index, suffix, wc_mapping))
        }
        LexicalRuleID::FGOODRIGHT => {
            let expected_word = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_appears_to_right(&sentence, current_index, expected_word, _wordclass)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::DELETEPREF => {
            let prefix = rule.parameters.get(0)?;
            Option::from(delete_prefix(&sentence, current_index, prefix, wc_mapping))
        }
        LexicalRuleID::FGOODLEFT => {
            let expected_word = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_appears_to_left(&sentence, current_index, expected_word, _wordclass)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::GOODLEFT => {
            let expected_word = rule.parameters.get(0)?;
            Option::from(appears_to_left(&sentence, current_index, expected_word))
        }
        LexicalRuleID::GOODRIGHT => {
            let expected_word = rule.parameters.get(0)?;
            Option::from(appears_to_right(&sentence, current_index, expected_word))
        }
        LexicalRuleID::FDELETESUF => {
            let suffix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_delete_suffix(&sentence, current_index, suffix, _wordclass, wc_mapping)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::CHAR => {
            let c = rule.parameters.get(0)?;
            Option::from(has_char(&sentence, current_index, c.parse().unwrap()))
        }
        LexicalRuleID::FDELETEPREF => {
            let prefix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_delete_prefix(&sentence, current_index, prefix, _wordclass, wc_mapping)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FADDSUF => {
            let suffix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_add_suffix(&sentence, current_index, suffix, _wordclass, wc_mapping)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FHASSUF => {
            let suffix: &str = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_has_suffix(&sentence, current_index, suffix, _wordclass)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FHASPREF => {
            let suffix: &str = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Some(_wordclass) => { Option::from(f_has_prefix(&sentence, current_index, suffix, _wordclass)) }
                None => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::DELETESUF => {
            let suffix = rule.parameters.get(0)?;
            Option::from(delete_suffix(&sentence, current_index, suffix, wc_mapping))
        }
    }
}


/// Applies a given lexical rule.
pub fn lexical_rule_apply(sentence: &mut Vec<(String, Wordclass)>, current_index: i32, rule: &LexicalRulespec, wc_mapping: &WordclassMap) -> Option<bool> {

    let uindex: usize = current_index as usize;

    /// If a word is numeric, it is type WORDCLASS::NUM
    if sentence.get(uindex)?.0.parse::<i64>().is_ok() || sentence.get(current_index as usize)?.0.parse::<f64>().is_ok() {
        //println!("yes")
        sentence[uindex].1 = Wordclass::NUM;
        Option::from(true);
    }

    // Run Lexical Rule
    match lexical_rule_holds(sentence, current_index, rule, &wc_mapping) {
        Some(true) => {
            let new_tag = rule.clone().target_tag;
            sentence[uindex].1 = new_tag;
            Option::from(true)
        }
        _ => Option::from(false),
    }
}


#[test]
fn test_lexical_rule_apply() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();


    let mut sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("AKJSHING"), Wordclass::ANY),
        (String::from("brown"), Wordclass::JJ),
    ];

    println!("sentence before: {:?}", sentence);

    let lexical_ruleset: Vec<LexicalRulespec> = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();

    for rule in lexical_ruleset {
        //println!("rule: {:?}", rule);
        lexical_rule_apply(&mut sentence, 1, &rule, &wc_mapping);
    }

    /*let rule_fhassuf = LexicalRulespec {
        ruleset_id: LexicalRuleID::HASSUF,
        target_tag: Wordclass::NN,
        parameters: vec![String::from("JJ"), "ick".parse().unwrap()],
    };*/


    //assert!(lexical_rule_apply(&mut sentence, 1, &rule_fhassuf, &wc_mapping).unwrap());

    println!("sentence after: {:?}", sentence);


}

/// Test to check the appears_to_left function correctly returns true as expected.
#[test]
fn test_appears_to_left_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::ANY),
        (String::from("quick"), Wordclass::ANY),
        (String::from("brown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(appears_to_left(&sentence, 1, "The"));
    assert!(appears_to_left(&sentence, 2, "quick"));

}


#[test]
fn test_appears_to_left_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!appears_to_left(&sentence, 1, "The"));
    assert!(!appears_to_left(&sentence, 2, "none"));

}


#[test]
fn test_f_appears_to_left_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_appears_to_left(&sentence, 1, "The", Wordclass::JJ));
    assert!(f_appears_to_left(&sentence, 2, "quick", Wordclass::JJ));

}


#[test]
fn test_f_appears_to_left_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quickest"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_appears_to_left(&sentence, 1, "The", Wordclass::ANY));
    assert!(!f_appears_to_left(&sentence, 2, "none", Wordclass::JJ));

}


#[test]
fn test_appears_to_right_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::ANY),
        (String::from("quick"), Wordclass::ANY),
        (String::from("brown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(appears_to_right(&sentence, 1, "brown"));
    assert!(appears_to_right(&sentence, 2, "lazy"));

}


#[test]
fn test_appears_to_right_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!appears_to_right(&sentence, 1, "brown"));
    assert!(!appears_to_right(&sentence, 2, "none"));

}


#[test]
fn test_f_appears_to_right_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_appears_to_right(&sentence, 1, "brown", Wordclass::JJ));
    assert!(f_appears_to_right(&sentence, 2, "fox", Wordclass::JJ));

}


#[test]
fn test_f_appears_to_right_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_appears_to_right(&sentence, 1, "quick", Wordclass::ANY));
    assert!(!f_appears_to_right(&sentence, 2, "none", Wordclass::JJ));

}


#[test]
fn test_delete_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::ANY),
        (String::from("quickest"), Wordclass::ANY),
        (String::from("brown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(delete_suffix(&sentence, 1, "est", &wc_mapping));
    assert!(delete_suffix(&sentence, 2, "n", &wc_mapping));

}


#[test]
fn test_delete_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quickest"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!delete_suffix(&sentence, 1, "est", &wc_mapping));
    assert!(!delete_suffix(&sentence, 2, "own", &wc_mapping));

}


#[test]
fn test_delete_f_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quickest"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_delete_suffix(&sentence, 1, "est", Wordclass::JJ, &wc_mapping));
    assert!(f_delete_suffix(&sentence, 2, "n", Wordclass::JJ, &wc_mapping));

}


#[test]
fn test_delete_f_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_delete_suffix(&sentence, 1, "est", Wordclass::ANY, &wc_mapping));
    assert!(!f_delete_suffix(&sentence, 2, "own", Wordclass::JJ, &wc_mapping));

}


#[test]
fn test_delete_prefix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::ANY),
        (String::from("unquick"), Wordclass::ANY),
        (String::from("unbrown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(delete_prefix(&sentence, 1, "un", &wc_mapping));
    assert!(delete_prefix(&sentence, 2, "un", &wc_mapping));

}


#[test]
fn test_delete_prefix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("unquick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!delete_prefix(&sentence, 1, "un", &wc_mapping));
    assert!(!delete_prefix(&sentence, 2, "aaa", &wc_mapping));
    assert!(!delete_prefix(&sentence, 2, "bro", &wc_mapping));

}


#[test]
fn test_delete_f_prefix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("unquick"), Wordclass::JJ),
        (String::from("unbrown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_delete_prefix(&sentence, 1, "un", Wordclass::JJ, &wc_mapping));
    assert!(f_delete_prefix(&sentence, 2, "un", Wordclass::JJ, &wc_mapping));

}


#[test]
fn test_delete_f_prefix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("unquick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_delete_prefix(&sentence, 1, "un", Wordclass::ANY, &wc_mapping));
    assert!(!f_delete_prefix(&sentence, 2, "zzz", Wordclass::JJ, &wc_mapping));
    assert!(!f_delete_prefix(&sentence, 2, "bro", Wordclass::JJ, &wc_mapping));

}


#[test]
fn test_add_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::ANY),
        (String::from("quick"), Wordclass::ANY),
        (String::from("brown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(add_suffix(&sentence, 1, "est", &wc_mapping));
    assert!(add_suffix(&sentence, 2, "ed", &wc_mapping));

}


#[test]
fn test_add_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!add_suffix(&sentence, 1, "est", &wc_mapping));
    assert!(!add_suffix(&sentence, 2, "zzz", &wc_mapping));

}


#[test]
fn test_add_f_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_add_suffix(&sentence, 1, "est", Wordclass::JJ, &wc_mapping));
    assert!(f_add_suffix(&sentence, 2, "ed", Wordclass::JJ, &wc_mapping));

}


#[test]
fn test_add_f_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_add_suffix(&sentence, 1, "est", Wordclass::ANY, &wc_mapping));
    assert!(!f_add_suffix(&sentence, 2, "zzz", Wordclass::JJ, &wc_mapping));

}


#[test]
fn test_word_in_lexicon() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    assert!(is_word_in_lexicon(String::from("apple"), &wc_mapping));
    assert!(is_word_in_lexicon(String::from("banana"), &wc_mapping));
}


#[test]
fn test_word_not_in_lexicon() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    assert!(!is_word_in_lexicon(String::from("abcde"), &wc_mapping));
    assert!(!is_word_in_lexicon(String::from(""), &wc_mapping));
}


#[test]
fn test_has_suffix_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::ANY),
        (String::from("brown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(has_suffix(&sentence, 1, "ick"));
    assert!(has_suffix(&sentence, 2, "rown"));

}


#[test]
fn test_has_suffix_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!has_suffix(&sentence, 1, "ick"));
    assert!(!has_suffix(&sentence, 2, "abcd"));

}


#[test]
fn test_has_f_suffix_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_has_suffix(&sentence, 1, "ick", Wordclass::JJ));
    assert!(f_has_suffix(&sentence, 2, "rown", Wordclass::JJ));

}


#[test]
fn test_has_f_suffix_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_has_suffix(&sentence, 1, "ick", Wordclass::ANY));
    assert!(!f_has_suffix(&sentence, 2, "abcd", Wordclass::JJ));

}


#[test]
fn test_has_prefix_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::ANY),
        (String::from("brown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(has_prefix(&sentence, 1, "qui"));
    assert!(has_prefix(&sentence, 2, "bro"));

}


#[test]
fn test_has_prefix_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!has_prefix(&sentence, 1, "qui"));
    assert!(!has_prefix(&sentence, 2, "abcd"));

}


#[test]
fn test_has_f_prefix_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_has_prefix(&sentence, 1, "qui", Wordclass::JJ));
    assert!(f_has_prefix(&sentence, 2, "bro", Wordclass::JJ));

}


#[test]
fn test_has_f_prefix_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_has_prefix(&sentence, 1, "qui", Wordclass::ANY));
    assert!(!f_has_prefix(&sentence, 2, "abcd", Wordclass::JJ));

}


#[test]
fn test_has_char_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::ANY),
        (String::from("brown"), Wordclass::ANY),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(has_char(&sentence, 1, 'q'));
    assert!(has_char(&sentence, 2, 'n'));

}


#[test]
fn test_has_char_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!has_char(&sentence, 1, 'q'));
    assert!(!has_char(&sentence, 2, 'k'));

}


#[test]
fn test_f_has_char_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(f_has_char(&sentence, 1, 'q', Wordclass::JJ));
    assert!(f_has_char(&sentence, 2, 'n', Wordclass::JJ));

}


#[test]
fn test_f_has_char_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!f_has_char(&sentence, 1, 'q', Wordclass::ANY));
    assert!(!f_has_char(&sentence, 2, 'k', Wordclass::JJ));
}