#!/usr/bin/env bash
# Check a folder of Agent Skills modules: front matter, folder contents,
# references, links and the mode table. Needs only python3 (3.9 or later).
#
#   scripts/check.sh [--namespaces a,b,...] [<skills folder>]
#   scripts/check.sh --self-test
#
# The folder is skills/ by default; a relative folder is resolved against the
# repository root, so give another repository's folder as an absolute path.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 -I - "$root" "$@" <<'PY'
import os, pathlib, re, shutil, sys, tempfile, urllib.parse

# This repository's namespaces. Source: the namespaces registry in
# wiki/program.md (owner, 2026-11-14); change both together.
NAMESPACES = ("council", "multiplex", "wiki", "workspace", "grill", "design-system", "frugal", "triage")
# The fields the Agent Skills specification lists for SKILL.md's front matter.
FIELDS = {"name", "description", "license", "compatibility", "metadata", "allowed-tools"}
NAME_RE = re.compile(r"^[a-z0-9]+(-[a-z0-9]+)*$")
SCHEME_RE = re.compile(r"^[A-Za-z][A-Za-z0-9+.-]*:")
INLINE_LINK_RE = re.compile(r"\]\(\s*(<[^>]*>|[^)\s]+)(?:\s+[\"'(][^)]*)?\s*\)", re.S)
REF_DEF_RE = re.compile(r"^\s{0,3}\[[^\]]+\]:\s*(<[^>]*>|\S+)")
FENCE_RE = re.compile(r"^\s{0,3}(`{3,}|~{3,})")
MODE_CELL_RE = re.compile(r"^`([^`]+)`$")
MAX_LINES = 500
HOME = os.path.expanduser("~")


def shown(path):
    """A path for output: relative to the repository root, else with ~ for home."""
    p = str(path)
    if p == ROOT or p.startswith(ROOT + os.sep):
        return os.path.relpath(p, ROOT)
    if p == HOME or p.startswith(HOME + os.sep):
        return "~" + p[len(HOME):]
    return p


def read_text(path, errors):
    try:
        return path.read_bytes().decode("utf-8")
    except UnicodeDecodeError:
        errors.append(f"{shown(path)}: not UTF-8")
        return None


def unquote(value):
    if len(value) >= 2 and value[0] == value[-1] == "'":
        return value[1:-1].replace("''", "'")
    if len(value) >= 2 and value[0] == value[-1] == '"':
        return value[1:-1]
    return value


def strip_yaml_comment(value):
    """Strip a YAML comment outside quotes, where # starts after whitespace."""
    quote = None
    i = 0
    while i < len(value):
        char = value[i]
        if quote == "'":
            if char == "'":
                if i + 1 < len(value) and value[i + 1] == "'":
                    i += 2
                    continue
                quote = None
        elif quote == '"':
            if char == "\\":
                i += 2
                continue
            if char == '"':
                quote = None
        elif char in "'\"":
            quote = char
        elif char == "#" and (i == 0 or value[i - 1].isspace()):
            return value[:i].rstrip()
        i += 1
    return value


def front_matter(text, where, errors):
    """Top-level fields of the front matter, or None if it has none.

    Enough YAML for SKILL.md: `key: value` lines at column 0, quoted or plain
    scalars continued on indented lines, block scalars (| and >) and indented
    mappings (metadata)."""
    lines = text.splitlines()
    if not lines or lines[0].strip() != "---":
        return None
    try:
        end = next(i for i in range(1, len(lines)) if lines[i].strip() == "---")
    except StopIteration:
        return None
    fields, key, block, parts = {}, None, None, []

    def close():
        if key is None:
            return
        if block is not None:
            joiner = "\n" if block.startswith("|") else " "
            fields[key] = joiner.join(p.strip() for p in parts).strip()
        elif parts and parts[0] == "":
            fields[key] = "\n".join(p.strip() for p in parts[1:])
        else:
            value = " ".join(p.strip() for p in parts).strip()
            value = strip_yaml_comment(value)
            fields[key] = unquote(value)

    for n, line in enumerate(lines[1:end], 2):
        if not line.strip() or line.lstrip().startswith("#"):
            if key is not None and block is not None:
                parts.append("")
            continue
        if line[0] in " \t":
            if key is None:
                errors.append(f"{where}:{n}: indented line before any field")
            else:
                parts.append(line)
            continue
        m = re.match(r"^([A-Za-z0-9_-]+):(?:\s+(.*))?$", line)
        if not m:
            errors.append(f"{where}:{n}: front matter line is not a `key: value` field")
            continue
        close()
        key, value = m.group(1), (m.group(2) or "").strip()
        if key in fields:
            errors.append(f"{where}:{n}: field {key!r} given twice")
        block = value if re.match(r"^[|>][+-]?$", value) else None
        parts = [] if block is not None else [value]
    close()
    return fields


def front_matter_end(text):
    """The number of lines the front matter takes, its --- lines included."""
    lines = text.splitlines()
    if lines and lines[0].strip() == "---":
        for i in range(1, len(lines)):
            if lines[i].strip() == "---":
                return i + 1
    return 0


def outside_code(text, spans=True):
    """Lines of Markdown with fenced code blocks, and code spans unless told
    to keep them, blanked out."""
    out, fence = [], None
    for line in text.splitlines():
        m = FENCE_RE.match(line)
        if fence is not None:
            if m and m.group(1)[0] == fence[0] and len(m.group(1)) >= len(fence):
                fence = None
            out.append("")
            continue
        if m:
            fence = m.group(1)
            out.append("")
            continue
        out.append(line if not spans else re.sub(r"(`+)(?:(?!\1).)+?\1", lambda c: "`" + " " * (len(c.group(0)) - 2) + "`", line))
    return out


def links(lines):
    text = "\n".join(lines)
    for m in INLINE_LINK_RE.finditer(text):
        target = m.group(1)
        yield text.count("\n", 0, m.start()) + 1, target[1:-1] if target.startswith("<") else target
    for n, line in enumerate(lines, 1):
        m = REF_DEF_RE.match(line)
        if m:
            target = m.group(1)
            yield n, target[1:-1] if target.startswith("<") else target


def tables(lines):
    """Each Markdown table as a list of data rows, each a list of cells."""
    found, i = [], 0
    while i + 1 < len(lines):
        head, sep = lines[i].strip(), lines[i + 1].strip()
        if re.match(r"^\|?\s*:?-+:?\s*(\|\s*:?-+:?\s*)*\|?$", sep) and "|" in head:
            header_cells = re.split(r"(?<!\\)\|", head.strip().strip("|"))
            column_count = len(header_cells)
            rows, i = [], i + 2
            while i < len(lines) and "|" in lines[i]:
                row_line = lines[i].strip()
                cells = [c.strip() for c in re.split(r"(?<!\\)\|", row_line.strip("|"))]
                if len(cells) != column_count:
                    break
                # Mode-table data rows start with a backticked mode word.
                # Without that, same-width prose (piped or unpiped) stops the
                # table instead of becoming a fake row.
                if not cells or not MODE_CELL_RE.match(cells[0]):
                    break
                rows.append(cells)
                i += 1
            found.append(rows)
        else:
            i += 1
    return found


def check_skill(folder, namespaces):
    """Errors and warnings for one skill folder."""
    errors, warnings = [], []
    rel = shown(folder)
    if folder.is_symlink():
        return [f"{rel}: a symlink, not a skill folder"], warnings
    for dirpath, dirnames, filenames in os.walk(folder):
        for entry in sorted(dirnames + filenames):
            p = pathlib.Path(dirpath, entry)
            if p.is_symlink():
                errors.append(f"{shown(p)}: symlink in a skill folder")
            elif entry == "UPSTREAM.md":
                errors.append(f"{shown(p)}: UPSTREAM.md: skills here are written here, not copied")
    skill = folder / "SKILL.md"
    if not skill.is_file() or skill.is_symlink():
        errors.append(f"{rel}: no SKILL.md")
        return errors, warnings
    if not (folder / "CREDITS.md").is_file():
        errors.append(f"{rel}: no CREDITS.md")

    text = read_text(skill, errors)
    if text is None:
        return errors, warnings
    where = shown(skill)
    if len(text.splitlines()) > MAX_LINES:
        warnings.append(f"{where}: {len(text.splitlines())} lines (the specification suggests at most {MAX_LINES})")
    fm = front_matter(text, where, errors)
    if fm is None:
        errors.append(f"{where}: no front matter between --- lines")
    else:
        for field in sorted(set(fm) - FIELDS):
            errors.append(f"{where}: field {field!r} is not in the Agent Skills specification")
        name = fm.get("name", "")
        if name != folder.name:
            errors.append(f"{where}: name {name!r} does not match folder {folder.name!r}")
        if not NAME_RE.match(name) or len(name) > 64:
            errors.append(f"{where}: name {name!r} must be lowercase letters, digits and single hyphens, at most 64 characters")
        elif namespaces and not any(name == ns or name.startswith(ns + "-") for ns in namespaces):
            errors.append(f"{where}: name {name!r} is not in the namespaces {', '.join(namespaces)}")
        desc = fm.get("description", "")
        if not desc:
            errors.append(f"{where}: description missing")
        elif len(desc) > 1024:
            errors.append(f"{where}: description is {len(desc)} characters (max 1024)")
        if len(fm.get("compatibility", "")) > 500:
            errors.append(f"{where}: compatibility is {len(fm['compatibility'])} characters (max 500)")

    refs_dir = folder / "references"
    refs = []
    if refs_dir.is_dir() and not refs_dir.is_symlink():
        for p in sorted(refs_dir.iterdir()):
            if p.is_symlink():
                continue
            if p.is_dir():
                errors.append(f"{shown(p)}: references/ must be flat, no subfolders")
            elif p.suffix != ".md":
                errors.append(f"{shown(p)}: references/ holds only .md files")
            else:
                refs.append(p)

    linked_from_skill = set()
    sources = [(skill, text)] + [(r, read_text(r, errors)) for r in refs]
    base = os.path.normpath(str(folder))
    for source, body in sources:
        if body is None:
            continue
        lines = outside_code(body)
        if source == skill:
            lines = [""] * front_matter_end(text) + lines[front_matter_end(text):]
        for n, target in links(lines):
            if target.startswith("#") or SCHEME_RE.match(target):
                continue
            path = urllib.parse.unquote(target.split("#", 1)[0])
            resolved = os.path.normpath(os.path.join(str(source.parent), path))
            if path.startswith("/") or not resolved.startswith(base + os.sep):
                errors.append(f"{shown(source)}:{n}: link {target!r} leaves the skill's folder")
            elif not os.path.isfile(resolved):
                errors.append(f"{shown(source)}:{n}: link {target!r} does not resolve to a file")
            elif source == skill:
                linked_from_skill.add(resolved)
    for r in refs:
        if os.path.normpath(str(r)) not in linked_from_skill:
            errors.append(f"{shown(r)}: not linked from SKILL.md")

    if refs:
        stems = {r.stem for r in refs}
        best, best_hits = None, 0
        for rows in tables(outside_code(text, spans=False)):
            words = [MODE_CELL_RE.match(row[0]).group(1) for row in rows if row and MODE_CELL_RE.match(row[0])]
            hits = len(stems & set(words))
            if hits > best_hits:
                best, best_hits = rows, hits
        if best is None:
            errors.append(f"{where}: no mode table whose first cells are the references' mode words in backticks")
        else:
            seen = {}
            for row in best:
                m = MODE_CELL_RE.match(row[0]) if row else None
                word = m.group(1) if m else None
                if word not in stems:
                    errors.append(f"{where}: mode-table row {row[0] if row else ''!r} has no matching references/<mode>.md")
                else:
                    seen[word] = seen.get(word, 0) + 1
            for word in sorted(stems):
                if word not in seen:
                    errors.append(f"{where}: references/{word}.md has no mode-table row with first cell `{word}`")
                elif seen[word] > 1:
                    errors.append(f"{where}: mode word `{word}` has {seen[word]} rows")
    return errors, warnings


def check_folder(folder, namespaces, root_checks, required, root):
    """Errors, warnings and the number of skills checked for a skills folder."""
    errors, warnings, count = [], [], 0
    if root_checks:
        for leftover in ("upstream", "upstream.toml", ".gitmodules"):
            if os.path.lexists(os.path.join(root, leftover)):
                errors.append(f"{leftover}: the upstream machinery was removed in M0")
    if not os.path.lexists(folder):
        if required:
            errors.append(f"{shown(folder)}: no such folder")
        return errors, warnings, count
    if folder.is_symlink() or not folder.is_dir():
        errors.append(f"{shown(folder)}: not a folder")
        return errors, warnings, count
    for child in sorted(folder.iterdir()):
        if child.is_symlink() or child.is_dir():
            e, w = check_skill(child, namespaces)
            errors += e
            warnings += w
            count += 1
    return errors, warnings, count


# The self-test's fixtures in scripts/fixtures/: each folder and the one FAIL
# line it must give (None: it must pass), checked with the namespace fixture.
FIXTURES = {
    "fixture": None,
    "fixture-no-skill-md": "no SKILL.md",
    "fixture-no-credits": "no CREDITS.md",
    "fixture-no-front-matter": "no front matter between --- lines",
    "fixture-bad-front-matter-line": "front matter line is not a `key: value` field",
    "fixture-duplicate-field": "field 'license' given twice",
    "fixture-unknown-field": "field 'version' is not in the Agent Skills specification",
    "fixture-name-mismatch": "does not match folder",
    "fixture--double-hyphen": "must be lowercase letters, digits and single hyphens",
    "fixture-name-too-long-" + "x" * 43: "at most 64 characters",
    "wrong-namespace": "is not in the namespaces fixture",
    "fixture-no-description": "description missing",
    "fixture-description-too-long": "description is 1025 characters (max 1024)",
    "fixture-compatibility-too-long": "compatibility is 501 characters (max 500)",
    "fixture-upstream-file": "UPSTREAM.md: skills here are written here, not copied",
    "fixture-reference-not-md": "references/ holds only .md files",
    "fixture-reference-subfolder": "references/ must be flat, no subfolders",
    "fixture-broken-link": "does not resolve to a file",
    "fixture-link-outside": "leaves the skill's folder",
    "fixture-unlinked-reference": "not linked from SKILL.md",
    "fixture-no-mode-table": "no mode table whose first cells are the references' mode words",
    "fixture-missing-mode-row": "has no mode-table row with first cell `extra`",
    "fixture-extra-mode-row": "has no matching references/<mode>.md",
    "fixture-duplicate-mode-row": "mode word `check` has 2 rows",
}


def self_test():
    """Run the check on the fixtures, and on cases Git can't hold, built in a
    temporary folder: each must give exactly its expected FAIL line."""
    fixtures = pathlib.Path(ROOT, "scripts", "fixtures")
    failures, passed = [], 0

    def expect(case, result, wanted, warn=None):
        nonlocal passed
        errors, warnings = result[0], result[1]
        errors = [e.replace(tmp, "<tmp>") for e in errors]
        if wanted is None:
            ok = not errors and (warn is None or any(warn in w for w in warnings))
        else:
            ok = len(errors) == 1 and wanted in errors[0]
        if ok:
            passed += 1
            shown_line = f"FAIL {errors[0]}" if errors else ("WARN " + warnings[0].replace(tmp, "<tmp>") if warn else "passes")
            print(f"pass {case}: {shown_line}")
        else:
            failures.append(f"self-test {case}: expected {wanted or warn or 'no FAIL line'!r}, got {errors + warnings!r}")

    tmp = tempfile.mkdtemp()
    try:
        present = {p.name for p in fixtures.iterdir()} if fixtures.is_dir() else set()
        for name in sorted(present - set(FIXTURES)):
            failures.append(f"self-test: scripts/fixtures/{name} has no expected result in FIXTURES")
        for name, wanted in FIXTURES.items():
            if name not in present:
                failures.append(f"self-test: scripts/fixtures/{name} is missing")
                continue
            expect(name, check_skill(fixtures / name, ("fixture",)), wanted)

        valid = fixtures / "fixture"
        def copy(case):
            dest = pathlib.Path(tmp, case, "skills", "fixture")
            shutil.copytree(valid, dest)
            return dest
        d = copy("symlink-inside")
        os.symlink("check.md", d / "references" / "link.md")
        expect("symlink-inside", check_skill(d, ("fixture",)), "symlink in a skill folder")
        d = copy("symlinked-skill")
        os.symlink("fixture", d.parent / "fixture-link")
        expect("symlinked-skill", check_skill(d.parent / "fixture-link", ("fixture",)), "a symlink, not a skill folder")
        d = copy("not-utf-8")
        (d / "SKILL.md").write_bytes((d / "SKILL.md").read_bytes() + b"\xff\n")
        expect("not-utf-8", check_skill(d, ("fixture",)), "not UTF-8")
        d = copy("long-skill-md")
        with open(d / "SKILL.md", "a", encoding="utf-8") as f:
            f.write("\nA line.\n" * 250)
        expect("long-skill-md", check_skill(d, ("fixture",)), None, warn=f"lines (the specification suggests at most {MAX_LINES})")

        d = copy("multiline-link")
        with open(d / "SKILL.md", "a", encoding="utf-8") as f:
            f.write("\n[missing](\nreferences/missing.md\n)\n")
        expect("multiline-link", check_skill(d, ("fixture",)), "does not resolve to a file")
        d = copy("yaml-inline-comments")
        skill_text = (d / "SKILL.md").read_text(encoding="utf-8")
        skill_text = skill_text.replace("name: fixture", 'name: "fixture" # module name')
        skill_text = skill_text.replace("description: A module for scripts/check.sh --self-test, not a real skill. Nothing installs it.", 'description: "A # module for scripts/check.sh --self-test." # fixture')
        (d / "SKILL.md").write_text(skill_text, encoding="utf-8")
        expect("yaml-inline-comments", check_skill(d, ("fixture",)), None)
        d = copy("yaml-comment-only-description")
        skill_text = (d / "SKILL.md").read_text(encoding="utf-8").replace(
            "description: A module for scripts/check.sh --self-test, not a real skill. Nothing installs it.",
            "description: # deliberately absent",
        )
        (d / "SKILL.md").write_text(skill_text, encoding="utf-8")
        expect("yaml-comment-only-description", check_skill(d, ("fixture",)), "description missing")
        d = copy("mode-table-without-outer-pipes")
        skill_text = (d / "SKILL.md").read_text(encoding="utf-8")
        skill_text = skill_text.replace(
            "| Mode | Reference | Use |\n| --- | --- | --- |\n| `check` | [check](references/check.md) | When the self-test runs |",
            "Mode | Reference | Use\n--- | --- | ---\n`check` | [check](references/check.md) | When the self-test runs",
        )
        (d / "SKILL.md").write_text(skill_text, encoding="utf-8")
        expect("mode-table-without-outer-pipes", check_skill(d, ("fixture",)), None)
        d = copy("mode-table-mixed-pipes")
        skill_text = (d / "SKILL.md").read_text(encoding="utf-8")
        skill_text = skill_text.replace(
            "| `check` | [check](references/check.md) | When the self-test runs |",
            "`check` | [check](references/check.md) | When the self-test runs",
        )
        (d / "SKILL.md").write_text(skill_text, encoding="utf-8")
        expect("mode-table-mixed-pipes", check_skill(d, ("fixture",)), None)
        parsed = tables(["| Mode | Reference |", "| --- | --- |", "| `check` | check.md |", "Use A | B for comparison"])
        if len(parsed) != 1 or len(parsed[0]) != 1:
            failures.append(f"self-test table-followed-by-prose: expected one row, got {parsed!r}")
        else:
            passed += 1
            print("pass table-followed-by-prose: prose excluded")
        parsed = tables(["| Mode | Reference |", "| --- | --- |", "`check` | check.md", "Use A | B for comparison"])
        if len(parsed) != 1 or len(parsed[0]) != 1:
            failures.append(f"self-test mixed-table-followed-by-prose: expected one row, got {parsed!r}")
        else:
            passed += 1
            print("pass mixed-table-followed-by-prose: prose excluded")
        parsed = tables(["Mode | Reference", "--- | ---", "`check` | check.md", "Use A | B for comparison"])
        if len(parsed) != 1 or len(parsed[0]) != 1:
            failures.append(f"self-test unpiped-table-followed-by-prose: expected one row, got {parsed!r}")
        else:
            passed += 1
            print("pass unpiped-table-followed-by-prose: prose excluded")

        empty = pathlib.Path(tmp, "empty", "skills")
        empty.mkdir(parents=True)
        result = check_folder(empty, None, False, True, str(empty.parent))
        expect("empty-folder", result, None)
        if result[2] != 0:
            failures.append(f"self-test empty-folder: {result[2]} skills checked, not 0")
        absent = pathlib.Path(tmp, "absent")
        expect("absent-default-folder", check_folder(absent / "skills", NAMESPACES, True, False, str(absent)), None)
        expect("missing-explicit-folder", check_folder(absent / "skills", None, False, True, str(absent)), "no such folder")
        f = pathlib.Path(tmp, "a-file")
        f.write_text("not a folder\n")
        expect("folder-is-a-file", check_folder(f, None, False, True, tmp), "not a folder")
        for leftover in ("upstream", "upstream.toml", ".gitmodules"):
            r = pathlib.Path(tmp, "leftover-" + leftover.strip("."))
            (r / "skills").mkdir(parents=True)
            (r / leftover).mkdir() if leftover == "upstream" else (r / leftover).write_text("")
            expect(f"root-{leftover}", check_folder(r / "skills", NAMESPACES, True, False, str(r)), f"{leftover}: the upstream machinery was removed in M0")
        d = copy("default-namespaces")
        expect("default-namespaces", check_folder(d.parent, NAMESPACES, True, False, str(d.parent.parent)), "is not in the namespaces council")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    for e in failures:
        print(f"FAIL {e}")
    if failures:
        sys.exit(1)
    print(f"ok: self-test, {passed} cases as expected")
    sys.exit(0)


def usage(message):
    print(f"FAIL {message}\nusage: scripts/check.sh [--namespaces a,b,...] [<skills folder>]\n       scripts/check.sh --self-test")
    sys.exit(2)


ROOT = sys.argv[1]
args, namespaces, folder_arg = sys.argv[2:], None, None
if args == ["--self-test"]:
    self_test()
while args:
    a = args.pop(0)
    if a == "--namespaces" or a.startswith("--namespaces="):
        value = a.split("=", 1)[1] if "=" in a else (args.pop(0) if args else "")
        namespaces = tuple(ns for ns in value.split(",") if ns)
        if not namespaces or not all(NAME_RE.match(ns) for ns in namespaces):
            usage(f"--namespaces needs a comma-separated list of names, got {value!r}")
    elif a.startswith("-"):
        usage(f"unknown option {a}")
    elif folder_arg is None:
        folder_arg = a
    else:
        usage("give at most one skills folder")

default_folder = pathlib.Path(ROOT, "skills")
folder = pathlib.Path(os.path.normpath(os.path.join(ROOT, folder_arg))) if folder_arg else default_folder
is_default = folder == default_folder
if namespaces is None and is_default:
    namespaces = NAMESPACES
errors, warnings, count = check_folder(folder, namespaces, is_default, folder_arg is not None, ROOT)
for e in errors:
    print(f"FAIL {e}")
if errors:
    sys.exit(1)
for w in warnings:
    print(f"WARN {w}")
print(f"ok: {count} skills checked")
PY
