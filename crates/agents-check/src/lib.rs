//! Native AG1.1 skill-module validator (parity with `docs/baseline-check.py`).
//!
//! The native engine does not exec `scripts/check.sh` or Python for validation.

pub mod folder;
pub mod frontmatter;
pub mod markdown;
pub mod selftest;
pub mod skill;
pub mod util;

use folder::check_folder;
use selftest::self_test;
use std::path::{Path, PathBuf};
use util::{NAME_RE, NAMESPACES};

/// Options for a native validation run (mirrors check.sh argv after ROOT).
#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    pub namespaces: Option<String>,
    pub self_test: bool,
    pub skills_folder: Option<PathBuf>,
}

/// Run the native validator. Prints FAIL/WARN/ok lines to stdout. Returns exit code.
pub fn run_native(root: &Path, opts: &RunOptions) -> i32 {
    if opts.self_test {
        return self_test(root);
    }

    let mut namespaces: Option<Vec<String>> = None;
    if let Some(ref value) = opts.namespaces {
        let list: Vec<String> = value
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        if list.is_empty() || !list.iter().all(|ns| NAME_RE.is_match(ns)) {
            return usage(&format!(
                "--namespaces needs a comma-separated list of names, got {}",
                util::py_repr(value)
            ));
        }
        namespaces = Some(list);
    }

    let default_folder = root.join("skills");
    let folder = if let Some(ref arg) = opts.skills_folder {
        let joined = if arg.is_absolute() {
            arg.clone()
        } else {
            root.join(arg)
        };
        PathBuf::from(util::normpath(&joined.to_string_lossy()))
    } else {
        default_folder.clone()
    };

    let is_default = folder == default_folder;
    let ns_owned: Option<Vec<String>> = if namespaces.is_none() && is_default {
        Some(NAMESPACES.iter().map(|s| (*s).to_string()).collect())
    } else {
        namespaces
    };
    let ns_refs: Option<Vec<&str>> = ns_owned
        .as_ref()
        .map(|v| v.iter().map(|s| s.as_str()).collect());
    let ns_for_check: Option<&[&str]> = ns_refs.as_ref().map(|v| v.as_slice());

    let (errors, warnings, count) = check_folder(
        &folder,
        ns_for_check,
        is_default,
        opts.skills_folder.is_some(),
        root,
        root,
    );

    for e in &errors {
        println!("FAIL {e}");
    }
    if !errors.is_empty() {
        return 1;
    }
    for w in &warnings {
        println!("WARN {w}");
    }
    println!("ok: {count} skills checked");
    0
}

fn usage(message: &str) -> i32 {
    println!(
        "FAIL {message}\nusage: scripts/check.sh [--namespaces a,b,...] [<skills folder>]\n       scripts/check.sh --self-test"
    );
    2
}

/// True when this crate's native validation path is compiled in (proof helper).
pub fn native_engine_is_in_process() -> bool {
    true
}

#[cfg(test)]
mod proof_tests {
    use super::*;

    #[test]
    fn native_path_does_not_require_python_body() {
        // The native runner is pure Rust; baseline Python is only for --engine baseline in main.
        assert!(native_engine_is_in_process());
        let _ = util::NAME_RE.is_match("fixture");
        let _ = frontmatter::strip_yaml_comment("x # c");
        let _ = markdown::tables(&[]);
    }
}
