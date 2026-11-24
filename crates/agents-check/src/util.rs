//! Shared helpers and constants (parity with baseline-check.py).

use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub const NAMESPACES: &[&str] = &[
    "council",
    "orchestration",
    "wiki",
    "workspace",
    "grill",
    "design-system",
    "frugal",
    "triage",
];

pub static FIELDS: LazyLock<std::collections::HashSet<&'static str>> = LazyLock::new(|| {
    [
        "name",
        "description",
        "license",
        "compatibility",
        "metadata",
        "allowed-tools",
    ]
    .into_iter()
    .collect()
});

pub static NAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap());
pub static SCHEME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z][A-Za-z0-9+.-]*:").unwrap());
pub static INLINE_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Title opener is ", ', or ( — same as baseline INLINE_LINK_RE.
    Regex::new(r#"(?s)\]\(\s*(<[^>]*>|[^)\s]+)(?:\s+["'(][^)]*)?\s*\)"#).unwrap()
});
pub static REF_DEF_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s{0,3}\[[^\]]+\]:\s*(<[^>]*>|\S+)").unwrap());
pub static FENCE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s{0,3}(`{3,}|~{3,})").unwrap());
pub static MODE_CELL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^`([^`]+)`$").unwrap());
pub static FIELD_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([A-Za-z0-9_-]+):(?:\s+(.*))?$").unwrap());
pub static BLOCK_SCALAR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[|>][+-]?$").unwrap());
pub static TABLE_SEP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\|?\s*:?-+:?\s*(\|\s*:?-+:?\s*)*\|?$").unwrap()
});
pub const MAX_LINES: usize = 500;

/// Blank out Markdown code spans (parity with baseline CODE_SPAN_RE sub).
/// Keeps the outer backticks; replaces the interior with spaces.
pub fn blank_code_spans(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '`' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        let mut n = 0;
        while i + n < chars.len() && chars[i + n] == '`' {
            n += 1;
        }
        // Find a closing run of exactly n backticks (non-greedy first match).
        let mut j = i + n;
        let mut found = None;
        while j + n <= chars.len() {
            if chars[j] == '`' {
                let mut m = 0;
                while j + m < chars.len() && chars[j + m] == '`' {
                    m += 1;
                }
                if m == n {
                    found = Some(j);
                    break;
                }
                j += m.max(1);
            } else {
                j += 1;
            }
        }
        if let Some(end) = found {
            let total = end + n - i;
            out.push('`');
            out.extend(std::iter::repeat(' ').take(total.saturating_sub(2)));
            out.push('`');
            i = end + n;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// Python-like `repr` for strings (prefer single quotes).
pub fn py_repr(s: &str) -> String {
    if !s.contains('\'') {
        format!("'{s}'")
    } else if !s.contains('"') {
        format!("\"{s}\"")
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}

/// A path for output: relative to the repository root, else with ~ for home.
pub fn shown(path: &Path, root: &Path) -> String {
    let p = path.to_string_lossy();
    let root_s = root.to_string_lossy();
    if p.as_ref() == root_s.as_ref()
        || p.starts_with(&(root_s.to_string() + std::path::MAIN_SEPARATOR_STR))
    {
        return path
            .strip_prefix(root)
            .map(|r| r.to_string_lossy().into_owned())
            .unwrap_or_else(|_| p.into_owned());
    }
    let home = dirs_home();
    if let Some(home) = home {
        let home_s = home.to_string_lossy();
        if p.as_ref() == home_s.as_ref() {
            return "~".into();
        }
        if p.starts_with(&(home_s.to_string() + std::path::MAIN_SEPARATOR_STR)) {
            return format!("~{}", &p[home_s.len()..]);
        }
    }
    p.into_owned()
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

pub fn read_text(path: &Path, root: &Path, errors: &mut Vec<String>) -> Option<String> {
    match std::fs::read(path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Some(s),
            Err(_) => {
                errors.push(format!("{}: not UTF-8", shown(path, root)));
                None
            }
        },
        Err(_) => {
            errors.push(format!("{}: not UTF-8", shown(path, root)));
            None
        }
    }
}

/// urllib.parse.unquote (percent-decode; `+` stays `+`).
pub fn unquote_url(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let h1 = from_hex(bytes[i + 1]);
            let h2 = from_hex(bytes[i + 2]);
            if let (Some(a), Some(b)) = (h1, h2) {
                out.push((a << 4) | b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Normpath like Python's os.path.normpath (POSIX).
pub fn normpath(path: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    let absolute = path.starts_with('/');
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if absolute {
                    let _ = stack.pop();
                } else if stack.last().map(|s| *s != "..").unwrap_or(false) {
                    let _ = stack.pop();
                } else {
                    stack.push("..");
                }
            }
            other => stack.push(other),
        }
    }
    let mut out = if absolute {
        format!("/{}", stack.join("/"))
    } else {
        stack.join("/")
    };
    if out.is_empty() {
        out = if absolute { "/" } else { "." }.into();
    }
    // Collapse // except leading
    out
}

pub fn path_is_under(resolved: &str, base: &str) -> bool {
    resolved.starts_with(&(base.to_string() + "/"))
}

pub fn lexists(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}
