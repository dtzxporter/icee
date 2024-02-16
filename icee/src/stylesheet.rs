use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Rule;
use crate::RuleState;
use crate::Rules;

/// A stylesheet for the iced ui crate.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StyleSheet {
    pub(crate) rules: Vec<Rule>,
}

impl StyleSheet {
    /// Attempts to load and parse a stylesheet at the given path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = std::fs::read(&path)?;
        let file = String::from_utf8(file)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Self::parse(file)
    }

    /// Attempts to parse a stylesheet.
    pub fn parse<S: AsRef<str>>(style: S) -> Result<Self, std::io::Error> {
        let stylesheet: Vec<Rule> = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(style.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        Ok(Self { rules: stylesheet })
    }

    /// Accumulates the rules for the given widget type and optional id.
    pub(crate) fn rules(&self, widget: &'static str, id: Option<&'static str>) -> Rules {
        let mut rules: BTreeMap<RuleState, Rule> = BTreeMap::new();

        let mut any: Rule = Rule::default();

        for rule in &self.rules {
            if let Some(rule_widget) = &rule.widget {
                if rule_widget != widget {
                    continue;
                }
            }

            if let Some(id) = id {
                if let Some(id_widget) = &rule.id {
                    if id_widget != id {
                        continue;
                    }
                }
            } else if rule.id.is_some() {
                continue;
            }

            if matches!(rule.state, RuleState::Any) {
                any.merge(rule);
            } else {
                match rules.entry(rule.state) {
                    Entry::Vacant(insert) => {
                        let mut merged = any.clone();
                        merged.merge(rule);

                        insert.insert(merged);
                    }
                    Entry::Occupied(mut occupied) => {
                        let mut merged = any.clone();
                        merged.merge(occupied.get());
                        merged.merge(rule);

                        *occupied.get_mut() = merged;
                    }
                }
            }
        }

        rules.insert(RuleState::Any, any);

        Rules {
            rules: Arc::new(rules),
        }
    }
}
