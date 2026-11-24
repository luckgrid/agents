//! Skills-folder validation and root leftover checks.

use crate::skill::check_skill;
use crate::util::{lexists, shown};
use std::path::Path;

/// Errors, warnings and the number of skills checked for a skills folder.
///
/// `display_root` is the repository root used for path display (Python global ROOT).
/// `leftover_root` is where upstream leftovers are checked when `root_checks` is set.
pub fn check_folder(
    folder: &Path,
    namespaces: Option<&[&str]>,
    root_checks: bool,
    required: bool,
    leftover_root: &Path,
    display_root: &Path,
) -> (Vec<String>, Vec<String>, usize) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut count = 0usize;

    if root_checks {
        for leftover in ["upstream", "upstream.toml", ".gitmodules"] {
            if lexists(&leftover_root.join(leftover)) {
                errors.push(format!(
                    "{leftover}: the upstream machinery was removed in M0"
                ));
            }
        }
    }

    if !lexists(folder) {
        if required {
            errors.push(format!("{}: no such folder", shown(folder, display_root)));
        }
        return (errors, warnings, count);
    }

    let is_symlink = folder
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);
    if is_symlink || !folder.is_dir() {
        errors.push(format!("{}: not a folder", shown(folder, display_root)));
        return (errors, warnings, count);
    }

    let mut children: Vec<_> = std::fs::read_dir(folder)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    children.sort();
    for child in children {
        let is_symlink = child
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);
        if is_symlink || child.is_dir() {
            let (e, w) = check_skill(&child, namespaces, display_root);
            errors.extend(e);
            warnings.extend(w);
            count += 1;
        }
    }
    (errors, warnings, count)
}
