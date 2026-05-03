use std::process::ExitCode;

use crate::{history, rules};

pub const WHITELIST: &[&str] = &["clear", "ls", "cd"];

pub fn run(typo: &str) -> ExitCode {
    let correct = history::lookup(typo).or_else(|| rules::match_typo(typo));
    match correct {
        Some(c) => {
            println!("ask\t{c}");
            ExitCode::from(1)
        }
        None => ExitCode::from(2),
    }
}
