//! YAML front-matter parsing (SKILL.md subset).

use crate::util::{py_repr, BLOCK_SCALAR_RE, FIELD_LINE_RE};
use std::collections::HashMap;

pub fn unquote(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() >= 2 && chars[0] == '\'' && chars[chars.len() - 1] == '\'' {
        let inner: String = chars[1..chars.len() - 1].iter().collect();
        return inner.replace("''", "'");
    }
    if chars.len() >= 2 && chars[0] == '"' && chars[chars.len() - 1] == '"' {
        return chars[1..chars.len() - 1].iter().collect();
    }
    value.to_string()
}

/// Strip a YAML comment outside quotes, where # starts after whitespace.
pub fn strip_yaml_comment(value: &str) -> String {
    if value.is_empty() {
        return value.to_string();
    }
    let chars: Vec<char> = value.chars().collect();
    if chars[0] != '\'' && chars[0] != '"' {
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '#' && (i == 0 || chars[i - 1].is_whitespace()) {
                return chars[..i].iter().collect::<String>().trim_end().to_string();
            }
            i += 1;
        }
        return value.to_string();
    }
    let mut quote: Option<char> = None;
    let mut i = 0;
    while i < chars.len() {
        let char = chars[i];
        match quote {
            Some('\'') => {
                if char == '\'' {
                    if i + 1 < chars.len() && chars[i + 1] == '\'' {
                        i += 2;
                        continue;
                    }
                    quote = None;
                }
            }
            Some('"') => {
                if char == '\\' {
                    i += 2;
                    continue;
                }
                if char == '"' {
                    quote = None;
                }
            }
            None => {
                if char == '\'' || char == '"' {
                    quote = Some(char);
                } else if char == '#' && (i == 0 || chars[i - 1].is_whitespace()) {
                    return chars[..i].iter().collect::<String>().trim_end().to_string();
                }
            }
            Some(_) => unreachable!(),
        }
        i += 1;
    }
    value.to_string()
}

/// Python str.splitlines() — omits a trailing empty segment from a final newline.
pub fn py_splitlines(text: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = Vec::new();
    if text.is_empty() {
        return lines;
    }
    let mut rest = text;
    loop {
        if rest.is_empty() {
            break;
        }
        if let Some(idx) = rest.find('\n') {
            let line = rest[..idx].strip_suffix('\r').unwrap_or(&rest[..idx]);
            lines.push(line);
            rest = &rest[idx + 1..];
        } else if let Some(idx) = rest.find('\r') {
            lines.push(&rest[..idx]);
            rest = &rest[idx + 1..];
        } else {
            lines.push(rest);
            break;
        }
    }
    lines
}

/// Top-level fields of the front matter, or None if it has none.
pub fn front_matter(
    text: &str,
    where_: &str,
    errors: &mut Vec<String>,
) -> Option<HashMap<String, String>> {
    let lines = py_splitlines(text);
    if lines.is_empty() || lines[0].trim() != "---" {
        return None;
    }
    let end = match lines.iter().enumerate().skip(1).find(|(_, l)| l.trim() == "---") {
        Some((i, _)) => i,
        None => return None,
    };

    let mut fields: HashMap<String, String> = HashMap::new();
    let mut key: Option<String> = None;
    let mut block: Option<String> = None;
    let mut parts: Vec<String> = Vec::new();

    let close = |key: &mut Option<String>,
                 block: &mut Option<String>,
                 parts: &mut Vec<String>,
                 fields: &mut HashMap<String, String>| {
        let Some(k) = key.take() else {
            return;
        };
        if let Some(b) = block.take() {
            let joiner = if b.starts_with('|') { "\n" } else { " " };
            let joined = parts
                .iter()
                .map(|p| p.trim())
                .collect::<Vec<_>>()
                .join(joiner);
            fields.insert(k, joined.trim().to_string());
        } else if parts.first().map(|p| p.as_str()) == Some("") {
            let joined = parts
                .iter()
                .skip(1)
                .map(|p| p.trim())
                .collect::<Vec<_>>()
                .join("\n");
            fields.insert(k, joined);
        } else {
            let value = parts
                .iter()
                .map(|p| p.trim())
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            let value = strip_yaml_comment(&value);
            fields.insert(k, unquote(&value));
        }
        parts.clear();
    };

    for (idx, line) in lines[1..end].iter().enumerate() {
        let n = idx + 2;
        let stripped = line.trim();
        if stripped.is_empty() || line.trim_start().starts_with('#') {
            if key.is_some() && block.is_some() {
                parts.push(String::new());
            }
            continue;
        }
        let first = line.chars().next().unwrap_or('\0');
        if first == ' ' || first == '\t' {
            if key.is_none() {
                errors.push(format!("{where_}:{n}: indented line before any field"));
            } else {
                parts.push((*line).to_string());
            }
            continue;
        }
        let Some(caps) = FIELD_LINE_RE.captures(line) else {
            errors.push(format!(
                "{where_}:{n}: front matter line is not a `key: value` field"
            ));
            continue;
        };
        close(&mut key, &mut block, &mut parts, &mut fields);
        let new_key = caps.get(1).unwrap().as_str().to_string();
        let value = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
        if fields.contains_key(&new_key) {
            errors.push(format!(
                "{where_}:{n}: field {} given twice",
                py_repr(&new_key)
            ));
        }
        if BLOCK_SCALAR_RE.is_match(value) {
            block = Some(value.to_string());
            parts.clear();
        } else {
            block = None;
            parts = vec![value.to_string()];
        }
        key = Some(new_key);
    }
    close(&mut key, &mut block, &mut parts, &mut fields);
    Some(fields)
}

/// The number of lines the front matter takes, its --- lines included.
pub fn front_matter_end(text: &str) -> usize {
    let lines = py_splitlines(text);
    if !lines.is_empty() && lines[0].trim() == "---" {
        for i in 1..lines.len() {
            if lines[i].trim() == "---" {
                return i + 1;
            }
        }
    }
    0
}
