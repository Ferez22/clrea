use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use strsim::levenshtein;

use crate::suggest::WHITELIST;

const DEFAULT_RULES: &str = include_str!("../assets/rules.toml");
const MAX_DISTANCE: usize = 2;

#[derive(Deserialize, Default, Debug)]
pub struct RulesFile {
    #[serde(flatten)]
    pub entries: HashMap<String, RuleEntry>,
}

#[derive(Deserialize, Default, Debug)]
pub struct RuleEntry {
    #[serde(default)]
    pub typos: Vec<String>,
}

fn user_rules_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("clrea").join("rules.toml"))
}

pub fn parse(text: &str) -> RulesFile {
    toml::from_str(text).unwrap_or_default()
}

pub fn merge(into: &mut RulesFile, other: RulesFile) {
    for (k, v) in other.entries {
        into.entries.entry(k).or_default().typos.extend(v.typos);
    }
}

fn load() -> RulesFile {
    let mut rules = parse(DEFAULT_RULES);
    if let Some(path) = user_rules_path() {
        if let Ok(text) = fs::read_to_string(&path) {
            merge(&mut rules, parse(&text));
        }
    }
    rules
}

pub fn match_in(rules: &RulesFile, typo: &str) -> Option<String> {
    for (correct, entry) in &rules.entries {
        if entry.typos.iter().any(|t| t == typo) {
            return Some(correct.clone());
        }
    }

    // Don't fuzzy-match a real command to another short command
    // (e.g. "ls" → "cd" at distance 2).
    if WHITELIST.contains(&typo) {
        return None;
    }

    let mut best: Option<(usize, &str)> = None;
    for cmd in WHITELIST {
        let d = levenshtein(typo, cmd);
        if d == 0 || d > MAX_DISTANCE {
            continue;
        }
        if best.map_or(true, |(bd, _)| d < bd) {
            best = Some((d, cmd));
        }
    }
    best.map(|(_, c)| c.to_string())
}

pub fn match_typo(typo: &str) -> Option<String> {
    match_in(&load(), typo)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> RulesFile {
        parse(DEFAULT_RULES)
    }

    #[test]
    fn explicit_typo_matches() {
        let r = defaults();
        assert_eq!(match_in(&r, "clrea"), Some("clear".into()));
        assert_eq!(match_in(&r, "claer"), Some("clear".into()));
        assert_eq!(match_in(&r, "sl"), Some("ls".into()));
        assert_eq!(match_in(&r, "dc"), Some("cd".into()));
    }

    #[test]
    fn levenshtein_fallback() {
        let r = RulesFile::default();
        assert_eq!(match_in(&r, "clearr"), Some("clear".into())); // distance 1
        // "ld" is distance 1 from both "cd" and "ls"; either acceptable.
        let m = match_in(&r, "ld");
        assert!(matches!(m.as_deref(), Some("cd") | Some("ls")));
    }

    #[test]
    fn exact_command_no_match() {
        let r = RulesFile::default();
        assert_eq!(match_in(&r, "clear"), None);
        assert_eq!(match_in(&r, "ls"), None);
        assert_eq!(match_in(&r, "cd"), None);
    }

    #[test]
    fn cross_short_command_not_matched() {
        // "ls" and "cd" are distance 2 apart but should not cross-match.
        let r = RulesFile::default();
        assert_eq!(match_in(&r, "ls"), None);
        assert_eq!(match_in(&r, "cd"), None);
    }

    #[test]
    fn far_typo_unknown() {
        let r = RulesFile::default();
        assert_eq!(match_in(&r, "xyzzy"), None);
        assert_eq!(match_in(&r, "vim"), None);
    }

    #[test]
    fn user_rules_merge() {
        let mut r = defaults();
        let user = parse(r#"
            [ls]
            typos = ["lll"]
        "#);
        merge(&mut r, user);
        assert_eq!(match_in(&r, "lll"), Some("ls".into()));
        // defaults still work
        assert_eq!(match_in(&r, "clrea"), Some("clear".into()));
    }

    #[test]
    fn malformed_toml_yields_empty() {
        let r = parse("not valid = = toml [[[");
        assert!(r.entries.is_empty());
    }
}
