use std::fs::read_to_string;
use super::wordclass::{map_pos_tag, Wordclass};
use std::io::{Error, ErrorKind};
use super::rulespec_id::*;
use super::contextual_rulespec::*;
use std::collections::HashMap;

/// ...
pub fn parse_contextual_ruleset(path: &str) -> Result<HashMap<Wordclass, Vec<ContextualRulespec>>, Error>
{
    let mut result: HashMap<Wordclass, Vec<ContextualRulespec>> = HashMap::new();
    for line in read_to_string(path)?.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Brill's original contextual rules are in the form `TAG` `TAG` `NAME` followed by rule-specific parameters, which can be
        // additional tags, or string literals. Here, for each line, it is ensured that the first 3 of the sequence exist to ensure
        // they can be safely accessed.
        let source: &str = parts.first().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing source tag"))?;
        let target: &str = parts.get(1).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing target tag"))?;
        let rulestring: &str = parts.get(2).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing ruleset ID"))?;

        // Since `source` and `target` should map to POS tags, the rulespec ID should also map.
        let source_tag: Option<Wordclass> = map_pos_tag(source);
        let target_tag: Option<Wordclass> = map_pos_tag(target);

        match (source_tag, target_tag) {
            (Some(s), Some(t)) => {
                let ruleset_id: RulespecID = map_rulespec_id(rulestring)?;

                // Finally, any additional parameters are collected, before the structure is added to the vector.
                let parameters: Vec<String> = parts.iter().skip(3).map(|s| s.to_string()).collect();
                let new_rulespec = ContextualRulespec {
                    source_tag: s.clone(),
                    target_tag: t,
                    ruleset_id, parameters,
                };

                // Append the rule specification into the vector mapping of the source tag, meaning this rule applies to the source tag.
                result.entry(s).or_default().push(new_rulespec.clone());
            }
            _ => {
            }
        }
    }
    Ok(result)
}
