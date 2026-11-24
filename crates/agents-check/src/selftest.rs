//! Embedded self-test corpus (parity with baseline-check.py).

use crate::folder::check_folder;
use crate::markdown::tables;
use crate::skill::check_skill;
use crate::util::{MAX_LINES, NAMESPACES};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

/// FIXTURES in baseline insertion order (stdout parity for compare).
fn fixtures_list() -> Vec<(String, Option<&'static str>)> {
    vec![
        ("fixture".into(), None),
        ("fixture-no-skill-md".into(), Some("no SKILL.md")),
        ("fixture-no-credits".into(), Some("no CREDITS.md")),
        (
            "fixture-no-front-matter".into(),
            Some("no front matter between --- lines"),
        ),
        (
            "fixture-bad-front-matter-line".into(),
            Some("front matter line is not a `key: value` field"),
        ),
        (
            "fixture-duplicate-field".into(),
            Some("field 'license' given twice"),
        ),
        (
            "fixture-unknown-field".into(),
            Some("field 'version' is not in the Agent Skills specification"),
        ),
        ("fixture-name-mismatch".into(), Some("does not match folder")),
        (
            "fixture--double-hyphen".into(),
            Some("must be lowercase letters, digits and single hyphens"),
        ),
        (
            format!("fixture-name-too-long-{}", "x".repeat(43)),
            Some("at most 64 characters"),
        ),
        (
            "wrong-namespace".into(),
            Some("is not in the namespaces fixture"),
        ),
        ("fixture-no-description".into(), Some("description missing")),
        (
            "fixture-description-too-long".into(),
            Some("description is 1025 characters (max 1024)"),
        ),
        (
            "fixture-compatibility-too-long".into(),
            Some("compatibility is 501 characters (max 500)"),
        ),
        (
            "fixture-upstream-file".into(),
            Some("UPSTREAM.md: skills here are written here, not copied"),
        ),
        (
            "fixture-reference-not-md".into(),
            Some("references/ holds only .md files"),
        ),
        (
            "fixture-reference-subfolder".into(),
            Some("references/ must be flat, no subfolders"),
        ),
        (
            "fixture-broken-link".into(),
            Some("does not resolve to a file"),
        ),
        (
            "fixture-link-outside".into(),
            Some("leaves the skill's folder"),
        ),
        (
            "fixture-unlinked-reference".into(),
            Some("not linked from SKILL.md"),
        ),
        (
            "fixture-no-mode-table".into(),
            Some("no mode table whose first cells are the references' mode words"),
        ),
        (
            "fixture-missing-mode-row".into(),
            Some("has no mode-table row with first cell `extra`"),
        ),
        (
            "fixture-extra-mode-row".into(),
            Some("has no matching references/<mode>.md"),
        ),
        (
            "fixture-duplicate-mode-row".into(),
            Some("mode word `check` has 2 rows"),
        ),
    ]
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), &to)?;
        }
    }
    Ok(())
}

/// Run the check on the fixtures; print pass/FAIL lines; return exit code.
pub fn self_test(root: &Path) -> i32 {
    let fixtures = root.join("scripts").join("fixtures");
    let failures: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let passed: RefCell<usize> = RefCell::new(0usize);
    let tmp = std::env::temp_dir().join(format!(
        "agents-check-selftest-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let tmp_s = tmp.to_string_lossy().into_owned();
    let _ = fs::remove_dir_all(&tmp);
    if fs::create_dir_all(&tmp).is_err() {
        eprintln!("FAIL self-test: could not create temp dir");
        return 1;
    }

    let replace_tmp = |s: &str| -> String { s.replace(&tmp_s, "<tmp>") };

    let expect = |case: &str,
                  errors: Vec<String>,
                  warnings: Vec<String>,
                  wanted: Option<&str>,
                  warn: Option<&str>| {
        let errors: Vec<String> = errors.iter().map(|e| replace_tmp(e)).collect();
        let warnings: Vec<String> = warnings.iter().map(|w| replace_tmp(w)).collect();
        let ok = if wanted.is_none() {
            errors.is_empty()
                && (warn.is_none() || warnings.iter().any(|w| w.contains(warn.unwrap())))
        } else {
            errors.len() == 1 && errors[0].contains(wanted.unwrap())
        };
        if ok {
            *passed.borrow_mut() += 1;
            let shown_line = if !errors.is_empty() {
                format!("FAIL {}", errors[0])
            } else if warn.is_some() {
                format!("WARN {}", warnings[0])
            } else {
                "passes".into()
            };
            println!("pass {case}: {shown_line}");
        } else {
            let mut got = errors.clone();
            got.extend(warnings);
            failures.borrow_mut().push(format!(
                "self-test {case}: expected {:?}, got {:?}",
                wanted.or(warn).unwrap_or("no FAIL line"),
                got
            ));
        }
    };

    let fixture_ns: &[&str] = &["fixture"];
    let fixtures_expected = fixtures_list();

    let present: HashSet<String> = if fixtures.is_dir() {
        fs::read_dir(&fixtures)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect()
    } else {
        HashSet::new()
    };

    let expected_keys: HashSet<String> = fixtures_expected.iter().map(|(k, _)| k.clone()).collect();
    // Match Python: sorted(present - set(FIXTURES))
    let mut unexpected: Vec<_> = present.difference(&expected_keys).cloned().collect();
    unexpected.sort();
    for name in unexpected {
        failures.borrow_mut().push(format!(
            "self-test: scripts/fixtures/{name} has no expected result in FIXTURES"
        ));
    }

    for (name, wanted) in &fixtures_expected {
        if !present.contains(name) {
            failures
                .borrow_mut()
                .push(format!("self-test: scripts/fixtures/{name} is missing"));
            continue;
        }
        let (e, w) = check_skill(&fixtures.join(name), Some(fixture_ns), root);
        expect(name, e, w, *wanted, None);
    }

    let valid = fixtures.join("fixture");
    let copy_case = |case: &str| -> PathBuf {
        let dest = tmp.join(case).join("skills").join("fixture");
        copy_dir(&valid, &dest).expect("copy fixture");
        dest
    };

    {
        let d = copy_case("symlink-inside");
        let _ = symlink("check.md", d.join("references").join("link.md"));
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect(
            "symlink-inside",
            e,
            w,
            Some("symlink in a skill folder"),
            None,
        );
    }
    {
        let d = copy_case("symlinked-skill");
        let link = d.parent().unwrap().join("fixture-link");
        let _ = symlink("fixture", &link);
        let (e, w) = check_skill(&link, Some(fixture_ns), root);
        expect(
            "symlinked-skill",
            e,
            w,
            Some("a symlink, not a skill folder"),
            None,
        );
    }
    {
        let d = copy_case("not-utf-8");
        let skill = d.join("SKILL.md");
        let mut bytes = fs::read(&skill).unwrap();
        bytes.extend_from_slice(b"\xff\n");
        fs::write(&skill, bytes).unwrap();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect("not-utf-8", e, w, Some("not UTF-8"), None);
    }
    {
        let d = copy_case("long-skill-md");
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open(d.join("SKILL.md"))
            .unwrap();
        let block = "\nA line.\n".repeat(250);
        f.write_all(block.as_bytes()).ok();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        let warn_msg = format!("lines (the specification suggests at most {MAX_LINES})");
        expect("long-skill-md", e, w, None, Some(&warn_msg));
    }
    {
        let d = copy_case("multiline-link");
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open(d.join("SKILL.md"))
            .unwrap();
        write!(f, "\n[missing](\nreferences/missing.md\n)\n").ok();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect(
            "multiline-link",
            e,
            w,
            Some("does not resolve to a file"),
            None,
        );
    }
    {
        let d = copy_case("yaml-inline-comments");
        let skill = d.join("SKILL.md");
        let mut skill_text = fs::read_to_string(&skill).unwrap();
        skill_text = skill_text.replace("name: fixture", "name: \"fixture\" # module name");
        skill_text = skill_text.replace(
            "description: A module for scripts/check.sh --self-test, not a real skill. Nothing installs it.",
            "description: \"A # module for scripts/check.sh --self-test.\" # fixture",
        );
        fs::write(&skill, skill_text).unwrap();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect("yaml-inline-comments", e, w, None, None);
    }
    {
        let d = copy_case("yaml-comment-only-description");
        let skill = d.join("SKILL.md");
        let skill_text = fs::read_to_string(&skill).unwrap().replace(
            "description: A module for scripts/check.sh --self-test, not a real skill. Nothing installs it.",
            "description: # deliberately absent",
        );
        fs::write(&skill, skill_text).unwrap();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect(
            "yaml-comment-only-description",
            e,
            w,
            Some("description missing"),
            None,
        );
    }
    {
        let d = copy_case("mode-table-without-outer-pipes");
        let skill = d.join("SKILL.md");
        let skill_text = fs::read_to_string(&skill).unwrap().replace(
            "| Mode | Reference | Use |\n| --- | --- | --- |\n| `check` | [check](references/check.md) | When the self-test runs |",
            "Mode | Reference | Use\n--- | --- | ---\n`check` | [check](references/check.md) | When the self-test runs",
        );
        fs::write(&skill, skill_text).unwrap();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect("mode-table-without-outer-pipes", e, w, None, None);
    }
    {
        let d = copy_case("mode-table-mixed-pipes");
        let skill = d.join("SKILL.md");
        let skill_text = fs::read_to_string(&skill).unwrap().replace(
            "| `check` | [check](references/check.md) | When the self-test runs |",
            "`check` | [check](references/check.md) | When the self-test runs",
        );
        fs::write(&skill, skill_text).unwrap();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect("mode-table-mixed-pipes", e, w, None, None);
    }

    let parsed = tables(&[
        "| Mode | Reference |".into(),
        "| --- | --- |".into(),
        "| `check` | check.md |".into(),
        "Use A | B for comparison".into(),
    ]);
    if parsed.len() != 1 || parsed[0].0.len() != 1 || !parsed[0].1.is_empty() {
        failures.borrow_mut().push(format!(
            "self-test table-followed-by-prose: expected one row no issues, got {parsed:?}"
        ));
    } else {
        *passed.borrow_mut() += 1;
        println!("pass table-followed-by-prose: prose excluded");
    }

    let parsed = tables(&[
        "| Mode | Reference |".into(),
        "| --- | --- |".into(),
        "`check` | check.md".into(),
        "Use A | B for comparison".into(),
    ]);
    if parsed.len() != 1 || parsed[0].0.len() != 1 || !parsed[0].1.is_empty() {
        failures.borrow_mut().push(format!(
            "self-test mixed-table-followed-by-prose: expected one row no issues, got {parsed:?}"
        ));
    } else {
        *passed.borrow_mut() += 1;
        println!("pass mixed-table-followed-by-prose: prose excluded");
    }

    let parsed = tables(&[
        "Mode | Reference".into(),
        "--- | ---".into(),
        "`check` | check.md".into(),
        "Use A | B for comparison".into(),
    ]);
    if parsed.len() != 1 || parsed[0].0.len() != 1 || !parsed[0].1.is_empty() {
        failures.borrow_mut().push(format!(
            "self-test unpiped-table-followed-by-prose: expected one row no issues, got {parsed:?}"
        ));
    } else {
        *passed.borrow_mut() += 1;
        println!("pass unpiped-table-followed-by-prose: prose excluded");
    }

    let parsed = tables(&[
        "| Mode | Reference |".into(),
        "| --- | --- |".into(),
        "| `check` | check.md |".into(),
        "| ghost | missing.md |".into(),
    ]);
    if parsed.len() != 1
        || parsed[0].0.len() != 1
        || !parsed[0].1.iter().any(|x| x.contains("invalid data row"))
    {
        failures.borrow_mut().push(format!(
            "self-test table-malformed-extra-row: expected one row plus invalid-row issue, got {parsed:?}"
        ));
    } else {
        *passed.borrow_mut() += 1;
        println!("pass table-malformed-extra-row: invalid row reported");
    }

    let parsed = tables(&[
        "| Mode | Reference |".into(),
        "| --- | --- |".into(),
        "| `check` | check.md |".into(),
        "| too | many | cells | here |".into(),
    ]);
    if parsed.len() != 1
        || parsed[0].0.len() != 1
        || !parsed[0].1.iter().any(|x| x.contains("invalid data row"))
    {
        failures.borrow_mut().push(format!(
            "self-test table-malformed-wrong-width: expected one row plus invalid-row issue, got {parsed:?}"
        ));
    } else {
        *passed.borrow_mut() += 1;
        println!("pass table-malformed-wrong-width: wrong-width row reported");
    }

    let parsed = tables(&[
        "| Mode | Reference | Use |".into(),
        "| --- | --- |".into(),
        "| `check` | check.md | when |".into(),
    ]);
    if !parsed.is_empty() {
        failures.borrow_mut().push(format!(
            "self-test table-header-sep-mismatch: expected no table, got {parsed:?}"
        ));
    } else {
        *passed.borrow_mut() += 1;
        println!("pass table-header-sep-mismatch: not a table");
    }

    {
        let d = copy_case("yaml-plain-apostrophe-comment");
        let skill = d.join("SKILL.md");
        let skill_text = fs::read_to_string(&skill).unwrap().replace(
            "description: A module for scripts/check.sh --self-test, not a real skill. Nothing installs it.",
            "description: don't strip the apostrophe # trailing comment",
        );
        fs::write(&skill, skill_text).unwrap();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect("yaml-plain-apostrophe-comment", e, w, None, None);
    }
    {
        let d = copy_case("multiline-link-across-paragraph");
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open(d.join("SKILL.md"))
            .unwrap();
        write!(f, "\n[missing](\n\nreferences/missing.md\n)\n").ok();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect("multiline-link-across-paragraph", e, w, None, None);
    }
    {
        let d = copy_case("mode-table-malformed-row");
        let skill = d.join("SKILL.md");
        let skill_text = fs::read_to_string(&skill).unwrap().replace(
            "| `check` | [check](references/check.md) | When the self-test runs |",
            "| `check` | [check](references/check.md) | When the self-test runs |\n| ghost | missing.md | When broken |",
        );
        fs::write(&skill, skill_text).unwrap();
        let (e, w) = check_skill(&d, Some(fixture_ns), root);
        expect(
            "mode-table-malformed-row",
            e,
            w,
            Some("invalid data row"),
            None,
        );
    }

    {
        let empty = tmp.join("empty").join("skills");
        fs::create_dir_all(&empty).unwrap();
        let result = check_folder(&empty, None, false, true, empty.parent().unwrap(), root);
        expect("empty-folder", result.0.clone(), result.1.clone(), None, None);
        if result.2 != 0 {
            failures.borrow_mut().push(format!(
                "self-test empty-folder: {} skills checked, not 0",
                result.2
            ));
        }
    }
    {
        let absent = tmp.join("absent");
        let result = check_folder(
            &absent.join("skills"),
            Some(NAMESPACES),
            true,
            false,
            &absent,
            root,
        );
        expect("absent-default-folder", result.0, result.1, None, None);
        let result = check_folder(&absent.join("skills"), None, false, true, &absent, root);
        expect(
            "missing-explicit-folder",
            result.0,
            result.1,
            Some("no such folder"),
            None,
        );
    }
    {
        let f = tmp.join("a-file");
        fs::write(&f, "not a folder\n").unwrap();
        let result = check_folder(&f, None, false, true, &tmp, root);
        expect(
            "folder-is-a-file",
            result.0,
            result.1,
            Some("not a folder"),
            None,
        );
    }
    for leftover in ["upstream", "upstream.toml", ".gitmodules"] {
        let case = format!("root-{leftover}");
        let r = tmp.join(format!("leftover-{}", leftover.trim_matches('.')));
        fs::create_dir_all(r.join("skills")).unwrap();
        if leftover == "upstream" {
            fs::create_dir(r.join(leftover)).unwrap();
        } else {
            fs::write(r.join(leftover), "").unwrap();
        }
        let result = check_folder(&r.join("skills"), Some(NAMESPACES), true, false, &r, root);
        let wanted = format!("{leftover}: the upstream machinery was removed in M0");
        expect(&case, result.0, result.1, Some(&wanted), None);
    }
    {
        let d = copy_case("default-namespaces");
        let result = check_folder(
            d.parent().unwrap(),
            Some(NAMESPACES),
            true,
            false,
            d.parent().unwrap().parent().unwrap(),
            root,
        );
        expect(
            "default-namespaces",
            result.0,
            result.1,
            Some("is not in the namespaces council"),
            None,
        );
    }

    let _ = fs::remove_dir_all(&tmp);

    let failures = failures.into_inner();
    let passed = passed.into_inner();
    for e in &failures {
        println!("FAIL {e}");
    }
    if !failures.is_empty() {
        return 1;
    }
    println!("ok: self-test, {passed} cases as expected");
    0
}
