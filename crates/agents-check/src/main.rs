//! AG1.1 skill-module validator CLI.
//!
//! Default engine is the native Rust port. `--engine baseline` still runs the
//! extracted Python body from `docs/baseline-check.py` for comparison.

use agents_check::{run_native, RunOptions};
use clap::error::ErrorKind;
use clap::{Parser, ValueEnum};
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BASELINE_PY: &str = include_str!("../docs/baseline-check.py");

#[derive(Debug, Clone, ValueEnum)]
enum Engine {
    /// Extracted baseline (identical logic to scripts/check.sh Python body).
    Baseline,
    /// Native Rust port (default). Does not exec check.sh or Python.
    Native,
}

#[derive(Parser, Debug)]
#[command(
    name = "agents-check",
    about = "AG1.1 skill-module validator (parity with scripts/check.sh)",
    disable_version_flag = true
)]
struct Args {
    /// Comma-separated namespaces (default: repository registry).
    #[arg(long = "namespaces", value_name = "LIST")]
    namespaces: Option<String>,

    /// Run the embedded self-test corpus.
    #[arg(long = "self-test")]
    self_test: bool,

    /// Validation engine.
    #[arg(long = "engine", value_enum, default_value_t = Engine::Native)]
    engine: Engine,

    /// Skills folder (default: skills/ under repository root).
    #[arg(value_name = "SKILLS_FOLDER")]
    skills_folder: Option<PathBuf>,
}

fn find_repo_root() -> PathBuf {
    if let Ok(p) = env::var("AGENTS_CHECK_ROOT") {
        return PathBuf::from(p);
    }
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join("scripts/check.sh").is_file() {
        return cwd;
    }
    let mut cur = cwd;
    for _ in 0..8 {
        if cur.join("scripts/check.sh").is_file() {
            return cur;
        }
        if !cur.pop() {
            break;
        }
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn run_baseline(root: &Path, argv: &[String]) -> i32 {
    let mut child = Command::new("python3")
        .arg("-I")
        .arg("-")
        .arg(root)
        .args(argv)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn python3 -I for baseline engine");
    {
        let mut stdin = child.stdin.take().expect("stdin");
        stdin
            .write_all(BASELINE_PY.as_bytes())
            .expect("write baseline script");
    }
    match child.wait() {
        Ok(status) => status.code().unwrap_or(1),
        Err(_) => 1,
    }
}

fn usage_unknown(option: &str) -> ! {
    // Match scripts/check.sh unknown-option wording (exit 2).
    eprintln!("FAIL unknown option {option}");
    eprintln!("usage: agents-check [--namespaces a,b,...] [<skills folder>]");
    eprintln!("       agents-check --self-test");
    std::process::exit(2);
}

fn parse_args() -> Args {
    match Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            if err.kind() == ErrorKind::UnknownArgument {
                let unknown = env::args()
                    .skip(1)
                    .find(|a| {
                        if a == "--" {
                            return false;
                        }
                        let name = a.split_once('=').map(|(n, _)| n).unwrap_or(a.as_str());
                        matches!(name.chars().next(), Some('-'))
                            && !matches!(
                                name,
                                "--namespaces"
                                    | "--self-test"
                                    | "--engine"
                                    | "--help"
                                    | "-h"
                            )
                    })
                    .unwrap_or_else(|| "unknown".into());
                usage_unknown(&unknown);
            }
            err.exit();
        }
    }
}

fn main() {
    let args = parse_args();
    let root = find_repo_root();

    match args.engine {
        Engine::Native => {
            let opts = RunOptions {
                namespaces: args.namespaces,
                self_test: args.self_test,
                skills_folder: args.skills_folder,
            };
            std::process::exit(run_native(&root, &opts));
        }
        Engine::Baseline => {
            let mut py_argv: Vec<String> = Vec::new();
            if let Some(ns) = &args.namespaces {
                py_argv.push("--namespaces".into());
                py_argv.push(ns.clone());
            }
            if args.self_test {
                py_argv.push("--self-test".into());
            } else if let Some(folder) = &args.skills_folder {
                py_argv.push(folder.display().to_string());
            }
            let code = run_baseline(&root, &py_argv);
            std::process::exit(code);
        }
    }
}
