use std::fmt;
use super::rulespec_id::RulespecID;
use super::wordclass::{map_pos_tag, Wordclass};


/// Function to check if the tag at index - 1 is equal to `tag` in a sentence.
pub fn previous_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_, ref _tag)) if _tag == &tag => true,
        _ => false,
    }
}


/// Function to check if the tag at index - 1 is equal to `tag` in a sentence.
pub fn next_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    match sentence.get((current_index + 1) as usize) {
        Some((_, ref _tag)) if _tag == &tag => true,
        _ => false,
    }
}


/// Function to check if the word at index - 1 is equal to `word` in a sentence.
pub fn previous_word(sentence: Vec<(String, Wordclass)>, current_index: i32, word: &str) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_word, _)) if _word == &word => true,
        _ => false,
    }
}


/// Function to check if the tag at index +1 or index +2 is equal to `tag` in a sentence.
pub fn next_one_or_two_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    (1..=2).any(|offset| {
        sentence.get((current_index + offset) as usize).map_or(false, |&(_, ref _tag)| _tag == &tag)
    })
}


/// Function to check if the tag at index - 1 or index - 2 is equal to `tag` in a sentence.
pub fn previous_one_or_two_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    (1..=2).any(|offset| {
        sentence.get((current_index - offset) as usize).map_or(false, |&(_, ref _tag)| _tag == &tag)
    })
}


/// Function to check if the word at index - 1 or index - 2 or index - 3 is equal to `tag` in a sentence.
pub fn previous_one_or_two_or_three_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    (1..=3).any(|offset| {
        sentence.get((current_index - offset) as usize).map_or(false, |&(_, ref _tag)| _tag == &tag)
    })
}


/// Function to check if the tag at index +1, +2 or +3 is equal to `tag` in a sentence.
pub fn next_one_or_two_or_three_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    sentence.iter().skip((current_index + 1) as usize).take(3).any(|(_, t)| *t == tag)
}


/// Function to check current word, and tag 2 words after.
pub fn word_and_tag_2_after(sentence: Vec<(String, Wordclass)>, current_index: i32, word: &str, tag: Wordclass) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w, _)| w == &word) {
        sentence.get(current_index as usize + 2).map_or(false, |(_, t)| *t == tag)
    } else { false }
}


/// Function to check current word, and word 2 words after.
pub fn word_and_2_after(sentence: Vec<(String, Wordclass)>, current_index: i32, word_one: &str, word_two: &str) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w1, _)| w1 == &word_one) {
        sentence.get((current_index + 2) as usize).map_or(false, |(w2, _)| w2 == &word_two)
    } else { false }
}


/// Function to check if the word at index - 1 or index - 2 is equal to `word` in a sentence.
pub fn previous_one_or_two_word(sentence: Vec<(String, Wordclass)>, current_index: i32, word: &str) -> bool {
    (1..=2).any(|offset| {
        sentence.get((current_index - offset) as usize).map_or(false, |(_word, _)| _word == word)
    })
}


/// Function to check if the tag at index - 2 is equal to `tag` in a sentence.
pub fn prev_two_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    match sentence.get((current_index - 2) as usize) {
        Some((_, ref _tag)) if _tag == &tag => true,
        _ => false,
    }
}


/// Function to check if the word at index + 1 is equal to `word` in a sentence.
pub fn next_word (sentence: Vec<(String, Wordclass)>, current_index: i32, word: &str) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_word, _)) if _word == &word => true,
        _ => false,
    }
}


/// Function to check current word and tag of hte next word.
pub fn word_and_next_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, word_one: &str, next_tag: Wordclass) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w1, _)| w1 == &word_one) {
        sentence.get(current_index as usize + 2).map_or(false, |(_, _tag)| _tag == &next_tag)
    } else { false }
}


/// Function to check the surrounding tags of a word.
pub fn surrounding_tags(sentence: Vec<(String, Wordclass)>, current_index: i32, previous_tag: Wordclass, next_tag: Wordclass) -> bool {
    match sentence.get((current_index  - 1) as usize) {
        Some((_, ref _tag)) if _tag == &previous_tag => match sentence.get((current_index + 1) as usize) {
            Some((_, ref _tag)) if _tag == &next_tag => true,
            _ => false,
        },
        _ => false,
    }
}


/// Function to check current word and tag of hte next word.
pub fn word_and_two_tag_before(sentence: Vec<(String, Wordclass)>, current_index: i32, word: &str, tag: Wordclass) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w1, _)| w1 == &word) {
        sentence.get(current_index as usize - 2).map_or(false, |(_, _tag)| _tag == &tag)
    } else { false }
}



/// Function to check a right-bigram.
pub fn right_bigram(sentence: Vec<(String, Wordclass)>, current_index: i32, word_one: &str, word_two: &str) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w1, _)| w1 == &word_one) {
        sentence.get((current_index + 1) as usize).map_or(false, |(w2, _)| w2 == &word_two)
    } else { false }
}



/// Function to check a left-bigram.
pub fn left_bigram(sentence: Vec<(String, Wordclass)>, current_index: i32, word_one: &str, word_two: &str) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w1, _)| w1 == &word_one) {
        sentence.get((current_index - 1) as usize).map_or(false, |(w2, _)| w2 == &word_two)
    } else { false }
}


/// Function to check previous bigram tags
pub fn prev_bigram(sentence: Vec<(String, Wordclass)>, current_index: i32, class_one: Wordclass, class_two: Wordclass) -> bool {
    if sentence.get((current_index - 1) as usize).map_or(false, |(_, tag1)| tag1 == &class_one) {
        sentence.get((current_index - 2) as usize ).map_or(false, |(_, tag2)| tag2 == &class_two)
    } else { false }
}



/// Function to check the next bigram tags
pub fn next_bigram(sentence: Vec<(String, Wordclass)>, current_index: i32, class_one: Wordclass, class_two: Wordclass) -> bool {
    if sentence.get(current_index as usize + 1).map_or(false, |(_, tag1)| tag1 == &class_one) {
        sentence.get(current_index as usize + 2).map_or(false, |(_, tag2)| tag2 == &class_two)
    } else { false }
}



/// Function to check a left-bigram.
pub fn current_word(sentence: Vec<(String, Wordclass)>, current_index: i32, word: &str) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w1, _)| w1 == &word) { true }
    else { false }
}



/// Function to check word and previous tag
pub fn word_and_previous_tag(sentence: Vec<(String, Wordclass)>, current_index: i32, word: &str, tag: Wordclass) -> bool {
    if sentence.get(current_index as usize).map_or(false, |(w1, _)| w1 == &word) {
        sentence.get(current_index as usize - 1).map_or(false, |(_, _tag)| _tag == &tag)
    } else { false }
}



/// Function to check word and previous tag
pub fn next_two_tags(sentence: Vec<(String, Wordclass)>, current_index: i32, tag1: Wordclass) -> bool {
    if sentence.get((current_index + 2) as usize).map_or(false, |(_, _tag1)| _tag1 == &tag1) { true }
    else { false }
}



// Checks a given contextual rule.
pub fn contextual_rule_holds(sentence: Vec<(String, Wordclass)>, current_index: i32, rule: ContextualRulespec) -> Option<bool> {

    match rule.ruleset_id {

        RulespecID::PREVTAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => { Option::from(previous_tag(sentence, current_index, _wordclass)) }
                None => { Option::from(false) }
            }
        },

        RulespecID::PREVWD => {
            let param_original = rule.parameters.get(0)?;
            Option::from(previous_word(sentence, current_index, param_original))
        },

        RulespecID::PREV1OR2TAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => { Option::from(previous_one_or_two_tag(sentence, current_index, _wordclass)) }
                None => { Option::from(false) }
            }
        },

        RulespecID::PREV1OR2OR3TAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => { Option::from(previous_one_or_two_or_three_tag(sentence, current_index, _wordclass)) }
                None => { Option::from(false) }
            }
        },

        RulespecID::NEXT1OR2OR3TAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => Option::from(next_one_or_two_or_three_tag(sentence, current_index, _wordclass)),
                None         => Option::from(false),
            }
        },

        RulespecID::WDAND2TAGAFT => {
            let word_parameter = rule.parameters.get(0)?;
            let type_parameter = rule.parameters.get(1)?;
            let type_wordclass = map_pos_tag(type_parameter);
            match type_wordclass {
                Some(_wordclass) => { Option::from(word_and_tag_2_after(sentence, current_index, word_parameter, _wordclass)) }
                None         => Option::from(false),
            }
        },

        RulespecID::WDAND2AFT => {
            let first_word_parameter = rule.parameters.get(0)?;
            let second_word_parameter = rule.parameters.get(1)?;
            Option::from(word_and_2_after(sentence, current_index, first_word_parameter, second_word_parameter))
        },

        RulespecID::PREV1OR2WD => {
            let word_parameter = rule.parameters.get(0)?;
            Option::from(previous_one_or_two_word(sentence, current_index, word_parameter))
        },

        RulespecID::NEXT1OR2TAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => { Option::from(next_one_or_two_tag(sentence, current_index, _wordclass)) }
                None => { Option::from(false) }
            }
        },

        RulespecID::NEXTTAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => { Option::from(next_tag(sentence, current_index, _wordclass)) }
                None => { Option::from(false) }
            }
        },

        RulespecID::PREV2TAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => { Option::from(prev_two_tag(sentence, current_index, _wordclass)) }
                None => { Option::from(false) }
            }
        },

        RulespecID::NEXTWD => {
            let next_word_parameter = rule.parameters.get(0)?;
            Option::from(next_word(sentence, current_index, next_word_parameter))
        },

        RulespecID::WDNEXTTAG => {
            let word_parameter = rule.parameters.get(0)?;
            let type_parameter = rule.parameters.get(1)?;
            let type_wordclass = map_pos_tag(type_parameter);
            match type_wordclass {
                Some(_wordclass) => { Option::from(word_and_next_tag(sentence, current_index, word_parameter, _wordclass)) }
                None        => Option::from(false),
            }
        },

        RulespecID::SURROUNDTAG => {
            let type_parameter1 = rule.parameters.get(0)?;
            let type_parameter2 = rule.parameters.get(1)?;
            let type_wordclass1 = map_pos_tag(type_parameter1);
            let type_wordclass2 = map_pos_tag(type_parameter2);
            match (type_wordclass1, type_wordclass2) {
                (Some(wordclass1), Some(wordclass2)) => Option::from(surrounding_tags(sentence, current_index, wordclass1, wordclass2)),
                _ => Option::from(false),
            }
        },

        RulespecID::WDAND2TAGBFR => {
            let word_parameter = rule.parameters.get(0)?;
            let type_parameter = rule.parameters.get(1)?;
            let type_wordclass = map_pos_tag(type_parameter);
            match type_wordclass {
                Some(_wordclass) => { Option::from(word_and_two_tag_before(sentence, current_index, word_parameter, _wordclass)) }
                None        => Option::from(false),
            }
        },

        RulespecID::RBIGRAM => {
            let first_word_param = rule.parameters.get(0)?;
            let second_word_param = rule.parameters.get(1)?;
            Option::from(right_bigram(sentence, current_index, first_word_param, second_word_param))
        },


        RulespecID::PREVBIGRAM => {
            let type_parameter1 = rule.parameters.get(0)?;
            let type_parameter2 = rule.parameters.get(1)?;
            let type_wordclass1 = map_pos_tag(type_parameter1);
            let type_wordclass2 = map_pos_tag(type_parameter2);
            match (type_wordclass1, type_wordclass2) {
                (Some(wordclass1), Some(wordclass2)) => Option::from(prev_bigram(sentence, current_index, wordclass1, wordclass2)),
                _ => Option::from(false),
            }
        },

        RulespecID::CURWD => {
            let word_param = rule.parameters.get(0)?;
            Option::from(current_word(sentence, current_index, word_param))
        },

        RulespecID::WDPREVTAG => {
            let word_parameter = rule.parameters.get(0)?;
            let type_parameter = rule.parameters.get(1)?;
            let type_wordclass = map_pos_tag(type_parameter);
            match type_wordclass {
                Some(_wordclass) => { Option::from(word_and_previous_tag(sentence, current_index, word_parameter, _wordclass)) }
                None         => Option::from(false),
            }
        },

        RulespecID::NEXTBIGRAM => {
            let type_parameter1 = rule.parameters.get(0)?;
            let type_parameter2 = rule.parameters.get(1)?;
            let type_wordclass1 = map_pos_tag(type_parameter1);
            let type_wordclass2 = map_pos_tag(type_parameter2);
            match (type_wordclass1, type_wordclass2) {
                (Some(wordclass1), Some(wordclass2)) => Option::from(next_bigram(sentence, current_index, wordclass1, wordclass2)),
                _ => Option::from(false),
            }
        },

        RulespecID::NEXT2TAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Some(_wordclass) => { Option::from(next_two_tags(sentence, current_index, _wordclass)) }
                None => { Option::from(false) }
            }
        },


        RulespecID::LBIGRAM => {
            let first_word_param = rule.parameters.get(0)?;
            let second_word_param = rule.parameters.get(1)?;
            Option::from(left_bigram(sentence, current_index, first_word_param, second_word_param))
        }
    }
}



pub fn contextual_rule_apply(sentence: &mut Vec<(String, Wordclass)>, current_index: i32, rule: ContextualRulespec) -> Option<bool> {
    // Check if Contextual Rule can be run
    let uindex: usize = current_index as usize;
    let check_pair = sentence.get(uindex)?;
    if check_pair.1 != rule.source_tag {
        return Option::from(false);
    }

    // Run Contextual Rule
    match contextual_rule_holds(sentence.to_owned(), current_index, rule.clone()) {
        Some(true) => {
            let new_tag = rule.clone().target_tag;
            sentence[uindex].1 = new_tag;
            Option::from(true)
        }
        _ => Option::from(false),
    }
}



#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ContextualRulespec {
    pub source_tag: Wordclass,
    pub target_tag: Wordclass,
    pub ruleset_id: RulespecID,
    pub parameters: Vec<String>,
}



impl fmt::Display for ContextualRulespec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuleContextual {{{:?} -> {:?} if {} passes with parameters: [{}] }}",
               self.source_tag, self.target_tag, self.ruleset_id, self.parameters.join(", ")
        )
    }
}


#[test]
fn test_contextual_rule() {
    let mut sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];

    let rule: ContextualRulespec = ContextualRulespec {
        source_tag: Wordclass::JJ,
        target_tag: Wordclass::FW,
        ruleset_id: RulespecID::SURROUNDTAG,
        parameters: vec!["NN".parse().unwrap(), "NN".parse().unwrap()],
    };

    for (w, c) in sentence.clone() {
        println!("{} {}", w, c);
    }

    contextual_rule_apply(sentence.as_mut(), 2, rule, );

    for (w, c) in sentence {
        println!("{} {}", w, c);
    }
}

#[test]
fn test_previous_one_or_two_tag_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(previous_one_or_two_tag(sentence.clone(), 3, Wordclass::JJ));
    assert!(previous_one_or_two_tag(sentence.clone(), 4, Wordclass::JJ));
}



#[test]
fn test_previous_one_or_two_tag_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!previous_one_or_two_tag(sentence.clone(), 2, Wordclass::NN));
    assert!(!previous_one_or_two_tag(sentence.clone(), 1, Wordclass::NN));
}



#[test]
fn test_previous_one_or_two_tag_out_of_bounds() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
    ];
    assert!(!previous_one_or_two_tag(sentence.clone(), 1, Wordclass::NN));
    assert!(!previous_one_or_two_tag(sentence.clone(), 0, Wordclass::DT));
}



#[test]
fn test_previous_one_or_two_or_three_tag_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(previous_one_or_two_or_three_tag(sentence.clone(), 4, Wordclass::JJ));
    assert!(previous_one_or_two_or_three_tag(sentence.clone(), 5, Wordclass::JJ));
}



#[test]
fn test_previous_one_or_two_or_three_tag_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 3, Wordclass::NN));
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 2, Wordclass::NN));
}



#[test]
fn test_previous_one_or_two_or_three_tag_out_of_bounds() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
    ];
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 1, Wordclass::NN));
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 0, Wordclass::DT));
}



#[test]
fn test_next_one_or_two_or_three_tag_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(next_one_or_two_or_three_tag(sentence.clone(), 1, Wordclass::JJ));
    assert!(next_one_or_two_or_three_tag(sentence.clone(), 0, Wordclass::JJ));
}



#[test]
fn test_next_one_or_two_or_three_tag_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 2, Wordclass::DT));
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 3, Wordclass::DT));
}



#[test]
fn test_next_one_or_two_or_three_tag_out_of_bounds() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
    ];
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 2, Wordclass::NN));
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 1, Wordclass::NN));
}



#[test]
fn test_word_and_tag_2_after_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
        (String::from("jumps"), Wordclass::VB),
    ];
    assert!(word_and_tag_2_after(sentence.clone(), 0, "The", Wordclass::JJ));
    assert!(word_and_tag_2_after(sentence.clone(), 1, "quick", Wordclass::NN));
}



#[test]
fn test_word_and_tag_2_after_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
        (String::from("jumps"), Wordclass::VB),
    ];
    assert!(!word_and_tag_2_after(sentence.clone(), 0, "The", Wordclass::NN));
    assert!(!word_and_tag_2_after(sentence.clone(), 1, "quick", Wordclass::VB));
}



#[test]
fn test_word_and_tag_2_after_out_of_bounds() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
    ];
    assert!(!word_and_tag_2_after(sentence.clone(), 2, "brown", Wordclass::NN));
    assert!(!word_and_tag_2_after(sentence.clone(), 1, "quick", Wordclass::NN));
}



#[test]
fn test_word_and_2_after_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
        (String::from("jumps"), Wordclass::VB),
    ];
    assert!(word_and_2_after(sentence.clone(), 0, "The", "brown"));
    assert!(word_and_2_after(sentence.clone(), 1, "quick", "fox"));
}



#[test]
fn test_word_and_2_after_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
        (String::from("jumps"), Wordclass::VB),
    ];
    assert!(!word_and_2_after(sentence.clone(), 0, "The", "fox"));
    assert!(!word_and_2_after(sentence.clone(), 1, "quick", "jumps"));
}


#[test]
fn test_word_and_2_after_out_of_bounds() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
    ];
    assert!(!word_and_2_after(sentence.clone(), 1, "quick", "brown"));
    assert!(!word_and_2_after(sentence.clone(), 0, "The", "quick"));
}


#[test]
fn test_previous_one_or_two_word_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(previous_one_or_two_word(sentence.clone(), 1, "The"));
    assert!(previous_one_or_two_word(sentence.clone(), 3, "brown"));
}


#[test]
fn test_previous_one_or_two_word_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("fox"), Wordclass::NN),
    ];
    assert!(!previous_one_or_two_word(sentence.clone(), 3, "The"));
    assert!(!previous_one_or_two_word(sentence.clone(), 2, "fox"));
}


#[test]
fn test_previous_one_or_two_word_out_of_bounds() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
    ];
    assert!(!previous_one_or_two_word(sentence.clone(), 0, "quick"));
}

#[test]
fn test_prev_two_tag_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(prev_two_tag(sentence.clone(), 2, Wordclass::DT));
    assert!(prev_two_tag(sentence.clone(), 3, Wordclass::JJ));

}

#[test]
fn test_prev_two_tag_not_found() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
        (String::from("lazy"), Wordclass::JJ),
        (String::from("dog"), Wordclass::NN),
    ];
    assert!(!prev_two_tag(sentence.clone(), 3, Wordclass::NN));
    assert!(!prev_two_tag(sentence.clone(), 2, Wordclass::NN));

}

#[test]
fn test_prev_two_tag_out_of_bounds() {
    let sentence = vec![
        (String::from("The"), Wordclass::DT),
        (String::from("quick"), Wordclass::JJ),
        (String::from("brown"), Wordclass::JJ),
    ];
    assert!(!prev_two_tag(sentence.clone(), 0, Wordclass::NN));
    assert!(!prev_two_tag(sentence.clone(), 1, Wordclass::NN));

}