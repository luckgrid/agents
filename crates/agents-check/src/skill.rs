//! Per-skill validation.

use crate::frontmatter::{front_matter, front_matter_end};
use crate::markdown::{links, outside_code, tables};
use crate::util::{
    normpath, path_is_under, py_repr, read_text, shown, unquote_url, FIELDS, MAX_LINES,
    MODE_CELL_RE, NAME_RE, SCHEME_RE,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Errors and warnings for one skill folder.
pub fn check_skill(
    folder: &Path,
    namespaces: Option<&[&str]>,
    root: &Path,
) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let rel = shown(folder, root);

    if folder
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
    {
        return (vec![format!("{rel}: a symlink, not a skill folder")], warnings);
    }

    // Walk like os.walk: dirs then files, sorted combined per directory.
    walk_skill_entries(folder, root, &mut errors);

    let skill = folder.join("SKILL.md");
    let skill_is_symlink = skill
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);
    if !skill.is_file() || skill_is_symlink {
        errors.push(format!("{rel}: no SKILL.md"));
        return (errors, warnings);
    }
    if !folder.join("CREDITS.md").is_file() {
        errors.push(format!("{rel}: no CREDITS.md"));
    }

    let text = match read_text(&skill, root, &mut errors) {
        Some(t) => t,
        None => return (errors, warnings),
    };
    let where_ = shown(&skill, root);
    let line_count = crate::frontmatter::py_splitlines(&text).len();
    if line_count > MAX_LINES {
        warnings.push(format!(
            "{where_}: {line_count} lines (the specification suggests at most {MAX_LINES})"
        ));
    }

    let fm = front_matter(&text, &where_, &mut errors);
    if fm.is_none() {
        errors.push(format!("{where_}: no front matter between --- lines"));
    } else {
        let fm = fm.unwrap();
        let mut unknown: Vec<_> = fm.keys().filter(|k| !FIELDS.contains(k.as_str())).cloned().collect();
        unknown.sort();
        for field in unknown {
            errors.push(format!(
                "{where_}: field {} is not in the Agent Skills specification",
                py_repr(&field)
            ));
        }
        let name = fm.get("name").map(|s| s.as_str()).unwrap_or("");
        let folder_name = folder
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if name != folder_name {
            errors.push(format!(
                "{where_}: name {} does not match folder {}",
                py_repr(name),
                py_repr(folder_name)
            ));
        }
        if !NAME_RE.is_match(name) || name.len() > 64 {
            errors.push(format!(
                "{where_}: name {} must be lowercase letters, digits and single hyphens, at most 64 characters",
                py_repr(name)
            ));
        } else if let Some(ns) = namespaces {
            if !ns.is_empty()
                && !ns
                    .iter()
                    .any(|n| name == *n || name.starts_with(&format!("{n}-")))
            {
                errors.push(format!(
                    "{where_}: name {} is not in the namespaces {}",
                    py_repr(name),
                    ns.join(", ")
                ));
            }
        }
        let desc = fm.get("description").map(|s| s.as_str()).unwrap_or("");
        if desc.is_empty() {
            errors.push(format!("{where_}: description missing"));
        } else if desc.len() > 1024 {
            errors.push(format!(
                "{where_}: description is {} characters (max 1024)",
                desc.len()
            ));
        }
        if let Some(compat) = fm.get("compatibility") {
            if compat.len() > 500 {
                errors.push(format!(
                    "{where_}: compatibility is {} characters (max 500)",
                    compat.len()
                ));
            }
        }
    }

    let refs_dir = folder.join("references");
    let mut refs: Vec<PathBuf> = Vec::new();
    let refs_is_symlink = refs_dir
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);
    if refs_dir.is_dir() && !refs_is_symlink {
        let mut entries: Vec<_> = std::fs::read_dir(&refs_dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();
        entries.sort();
        for p in entries {
            if p.symlink_metadata()
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false)
            {
                continue;
            }
            if p.is_dir() {
                errors.push(format!(
                    "{}: references/ must be flat, no subfolders",
                    shown(&p, root)
                ));
            } else if p.extension().and_then(|e| e.to_str()) != Some("md") {
                errors.push(format!(
                    "{}: references/ holds only .md files",
                    shown(&p, root)
                ));
            } else {
                refs.push(p);
            }
        }
    }

    let mut linked_from_skill: HashSet<String> = HashSet::new();
    let mut sources: Vec<(PathBuf, Option<String>)> = vec![(skill.clone(), Some(text.clone()))];
    for r in &refs {
        sources.push((r.clone(), read_text(r, root, &mut errors)));
    }
    let base = normpath(&folder.to_string_lossy());

    for (source, body) in &sources {
        let Some(body) = body else { continue };
        let mut lines = outside_code(body, true);
        if source == &skill {
            let fm_end = front_matter_end(&text);
            for line in lines.iter_mut().take(fm_end) {
                *line = String::new();
            }
        }
        for (n, target) in links(&lines) {
            if target.starts_with('#') || SCHEME_RE.is_match(&target) {
                continue;
            }
            let path_part = target.split_once('#').map(|(a, _)| a).unwrap_or(&target);
            let path = unquote_url(path_part);
            let joined = source
                .parent()
                .unwrap_or(Path::new("."))
                .join(&path);
            let resolved = normpath(&joined.to_string_lossy());
            if path.starts_with('/') || !path_is_under(&resolved, &base) {
                errors.push(format!(
                    "{}:{n}: link {} leaves the skill's folder",
                    shown(source, root),
                    py_repr(&target)
                ));
            } else if !Path::new(&resolved).is_file() {
                errors.push(format!(
                    "{}:{n}: link {} does not resolve to a file",
                    shown(source, root),
                    py_repr(&target)
                ));
            } else if source == &skill {
                linked_from_skill.insert(resolved);
            }
        }
    }

    for r in &refs {
        let rn = normpath(&r.to_string_lossy());
        if !linked_from_skill.contains(&rn) {
            errors.push(format!("{}: not linked from SKILL.md", shown(r, root)));
        }
    }

    if !refs.is_empty() {
        let stems: HashSet<String> = refs
            .iter()
            .filter_map(|r| r.file_stem().and_then(|s| s.to_str()).map(String::from))
            .collect();
        let mut best: Option<Vec<Vec<String>>> = None;
        let mut best_hits = 0usize;
        let mut best_issues: Vec<String> = Vec::new();
        for (rows, issues) in tables(&outside_code(&text, false)) {
            let words: Vec<String> = rows
                .iter()
                .filter_map(|row| {
                    row.first().and_then(|c| {
                        MODE_CELL_RE
                            .captures(c)
                            .map(|m| m.get(1).unwrap().as_str().to_string())
                    })
                })
                .collect();
            let word_set: HashSet<_> = words.iter().cloned().collect();
            let hits = stems.intersection(&word_set).count();
            if hits > best_hits {
                best = Some(rows);
                best_hits = hits;
                best_issues = issues;
            }
        }
        if best.is_none() {
            errors.push(format!(
                "{where_}: no mode table whose first cells are the references' mode words in backticks"
            ));
        } else {
            let best = best.unwrap();
            for issue in best_issues {
                errors.push(format!("{where_}: {issue}"));
            }
            let mut seen: HashMap<String, usize> = HashMap::new();
            for row in &best {
                let word = row.first().and_then(|c| {
                    MODE_CELL_RE
                        .captures(c)
                        .map(|m| m.get(1).unwrap().as_str().to_string())
                });
                match word {
                    Some(ref w) if stems.contains(w) => {
                        *seen.entry(w.clone()).or_insert(0) += 1;
                    }
                    _ => {
                        let cell = row.first().map(|s| s.as_str()).unwrap_or("");
                        errors.push(format!(
                            "{where_}: mode-table row {} has no matching references/<mode>.md",
                            py_repr(cell)
                        ));
                    }
                }
            }
            let mut stem_list: Vec<_> = stems.iter().cloned().collect();
            stem_list.sort();
            for word in stem_list {
                match seen.get(&word) {
                    None => {
                        errors.push(format!(
                            "{where_}: references/{word}.md has no mode-table row with first cell `{word}`"
                        ));
                    }
                    Some(&n) if n > 1 => {
                        errors.push(format!(
                            "{where_}: mode word `{word}` has {n} rows"
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    (errors, warnings)
}

fn walk_skill_entries(folder: &Path, root: &Path, errors: &mut Vec<String>) {
    let mut stack = vec![folder.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else {
            continue;
        };
        let mut names: Vec<(String, PathBuf, bool)> = Vec::new();
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let path = entry.path();
            let is_dir = entry
                .file_type()
                .map(|ft| ft.is_dir())
                .unwrap_or(false);
            // Don't follow symlink dirs (like os.walk followlinks=False): still
            // list them as entries to check for symlink error.
            names.push((name, path, is_dir));
        }
        names.sort_by(|a, b| a.0.cmp(&b.0));
        for (entry, p, is_dir) in names {
            let is_symlink = p
                .symlink_metadata()
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);
            if is_symlink {
                errors.push(format!("{}: symlink in a skill folder", shown(&p, root)));
            } else if entry == "UPSTREAM.md" {
                errors.push(format!(
                    "{}: UPSTREAM.md: skills here are written here, not copied",
                    shown(&p, root)
                ));
            }
            if is_dir && !is_symlink {
                stack.push(p);
            }
        }
    }
}
