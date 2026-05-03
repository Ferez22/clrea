use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::suggest::WHITELIST;

const MAX_ENTRIES: usize = 500;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct History {
    #[serde(default)]
    pub entry: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub typo: String,
    pub correct: String,
    pub count: u32,
}

fn default_path() -> io::Result<PathBuf> {
    let dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no config dir"))?
        .join("clrea");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("history.toml"))
}

pub fn load_from(path: &Path) -> History {
    let Ok(text) = fs::read_to_string(path) else {
        return History::default();
    };
    toml::from_str(&text).unwrap_or_default()
}

pub fn save_to(path: &Path, h: &History) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = toml::to_string(h).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, text)
}

pub fn lookup_in(h: &History, typo: &str) -> Option<String> {
    h.entry
        .iter()
        .find(|e| e.typo == typo)
        .map(|e| e.correct.clone())
}

pub fn learn_into(h: &mut History, typo: &str, correct: &str) -> io::Result<()> {
    if !WHITELIST.contains(&correct) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{correct}' not in whitelist"),
        ));
    }
    if let Some(e) = h.entry.iter_mut().find(|e| e.typo == typo) {
        e.count = e.count.saturating_add(1);
        e.correct = correct.to_string();
    } else {
        h.entry.push(Entry {
            typo: typo.to_string(),
            correct: correct.to_string(),
            count: 1,
        });
    }
    if h.entry.len() > MAX_ENTRIES {
        h.entry.sort_by(|a, b| b.count.cmp(&a.count));
        h.entry.truncate(MAX_ENTRIES);
    }
    Ok(())
}

pub fn lookup(typo: &str) -> Option<String> {
    let p = default_path().ok()?;
    lookup_in(&load_from(&p), typo)
}

pub fn learn(typo: &str, correct: &str) -> io::Result<()> {
    let p = default_path()?;
    let mut h = load_from(&p);
    learn_into(&mut h, typo, correct)?;
    save_to(&p, &h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn learn_then_lookup() {
        let mut h = History::default();
        learn_into(&mut h, "clrea", "clear").unwrap();
        assert_eq!(lookup_in(&h, "clrea"), Some("clear".into()));
        assert_eq!(h.entry[0].count, 1);
    }

    #[test]
    fn learn_increments_count() {
        let mut h = History::default();
        learn_into(&mut h, "sl", "ls").unwrap();
        learn_into(&mut h, "sl", "ls").unwrap();
        learn_into(&mut h, "sl", "ls").unwrap();
        assert_eq!(h.entry.len(), 1);
        assert_eq!(h.entry[0].count, 3);
    }

    #[test]
    fn whitelist_rejects_unsafe() {
        let mut h = History::default();
        let err = learn_into(&mut h, "x", "rm").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        assert!(h.entry.is_empty());
    }

    #[test]
    fn whitelist_rejects_arbitrary_path() {
        let mut h = History::default();
        assert!(learn_into(&mut h, "x", "/bin/sh").is_err());
        assert!(learn_into(&mut h, "x", "sudo").is_err());
    }

    #[test]
    fn lookup_miss_returns_none() {
        let h = History::default();
        assert_eq!(lookup_in(&h, "anything"), None);
    }

    #[test]
    fn save_then_load_roundtrip() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("history.toml");

        let mut h = History::default();
        learn_into(&mut h, "clrea", "clear").unwrap();
        learn_into(&mut h, "sl", "ls").unwrap();
        save_to(&p, &h).unwrap();

        let h2 = load_from(&p);
        assert_eq!(h2.entry.len(), 2);
        assert_eq!(lookup_in(&h2, "clrea"), Some("clear".into()));
        assert_eq!(lookup_in(&h2, "sl"), Some("ls".into()));
    }

    #[test]
    fn load_missing_file_yields_empty() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("nope.toml");
        let h = load_from(&p);
        assert!(h.entry.is_empty());
    }

    #[test]
    fn load_corrupt_file_yields_empty() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("history.toml");
        fs::write(&p, "this = is not [valid").unwrap();
        let h = load_from(&p);
        assert!(h.entry.is_empty());
    }

    #[test]
    fn lru_truncates_at_max() {
        let mut h = History::default();
        for i in 0..(MAX_ENTRIES + 50) {
            learn_into(&mut h, &format!("typo{i}"), "ls").unwrap();
        }
        assert_eq!(h.entry.len(), MAX_ENTRIES);
    }
}
