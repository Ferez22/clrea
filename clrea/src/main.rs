mod suggest;
mod history;
mod rules;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "clrea", version, about = "Typo-correcting shell helper")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Suggest a correction for a typo. Exit codes: 0=auto, 1=ask, 2=unknown.
    Suggest { typo: String },
    /// Record that `typo` should map to `correct`.
    Learn { typo: String, correct: String },
    /// Print shell integration snippet.
    Init { shell: String },
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Suggest { typo } => suggest::run(&typo),
        Cmd::Learn { typo, correct } => {
            if let Err(e) = history::learn(&typo, &correct) {
                eprintln!("clrea: {e}");
                return std::process::ExitCode::from(2);
            }
            std::process::ExitCode::SUCCESS
        }
        Cmd::Init { shell } => {
            match shell.as_str() {
                "zsh" => print!("{}", include_str!("../shell/clrea.zsh")),
                "bash" => print!("{}", include_str!("../shell/clrea.bash")),
                _ => {
                    eprintln!("clrea: unsupported shell '{shell}'");
                    return std::process::ExitCode::from(2);
                }
            }
            std::process::ExitCode::SUCCESS
        }
    }
}
