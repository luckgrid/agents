//! Markdown link and table extraction outside fenced code.

use crate::frontmatter::py_splitlines;
use crate::util::{
    blank_code_spans, py_repr, FENCE_RE, INLINE_LINK_RE, MODE_CELL_RE, REF_DEF_RE, TABLE_SEP_RE,
};

/// Lines of Markdown with fenced code blocks, and code spans unless told
/// to keep them, blanked out.
pub fn outside_code(text: &str, spans: bool) -> Vec<String> {
    let mut out = Vec::new();
    let mut fence: Option<String> = None;
    for line in py_splitlines(text) {
        let m = FENCE_RE.captures(line);
        if fence.is_some() {
            if let Some(ref caps) = m {
                let marker = caps.get(1).unwrap().as_str();
                let f = fence.as_ref().unwrap();
                if marker.as_bytes()[0] == f.as_bytes()[0] && marker.len() >= f.len() {
                    fence = None;
                }
            }
            out.push(String::new());
            continue;
        }
        if let Some(caps) = m {
            fence = Some(caps.get(1).unwrap().as_str().to_string());
            out.push(String::new());
            continue;
        }
        if spans {
            out.push(blank_code_spans(line));
        } else {
            out.push(line.to_string());
        }
    }
    out
}

/// Inline links within blank-line paragraphs, plus reference definitions.
pub fn links(lines: &[String]) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut start: usize = 1;
    let mut buf: Vec<String> = Vec::new();

    let flush = |start: &mut usize, buf: &mut Vec<String>, result: &mut Vec<(usize, String)>| {
        if buf.is_empty() {
            return;
        }
        let text = buf.join("\n");
        for caps in INLINE_LINK_RE.captures_iter(&text) {
            let whole = caps.get(0).unwrap();
            let target = caps.get(1).unwrap().as_str();
            let line_no = *start + text[..whole.start()].matches('\n').count();
            let target = if target.starts_with('<') && target.ends_with('>') {
                target[1..target.len() - 1].to_string()
            } else {
                target.to_string()
            };
            result.push((line_no, target));
        }
        *start += buf.len();
        buf.clear();
    };

    for line in lines {
        if line.trim().is_empty() {
            flush(&mut start, &mut buf, &mut result);
            start += 1;
            continue;
        }
        buf.push(line.clone());
    }
    flush(&mut start, &mut buf, &mut result);

    for (n, line) in lines.iter().enumerate() {
        if let Some(caps) = REF_DEF_RE.captures(line) {
            let target = caps.get(1).unwrap().as_str();
            let target = if target.starts_with('<') && target.ends_with('>') {
                target[1..target.len() - 1].to_string()
            } else {
                target.to_string()
            };
            result.push((n + 1, target));
        }
    }
    result
}

fn split_pipe_cells(s: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '|' && (i == 0 || chars[i - 1] != '\\') {
            cells.push(chars[start..i].iter().collect::<String>().trim().to_string());
            start = i + 1;
        }
        i += 1;
    }
    cells.push(chars[start..].iter().collect::<String>().trim().to_string());
    cells
}

/// Each Markdown table as (data rows, issues).
pub fn tables(lines: &[String]) -> Vec<(Vec<Vec<String>>, Vec<String>)> {
    let mut found = Vec::new();
    let mut i = 0;
    while i + 1 < lines.len() {
        let head = lines[i].trim();
        let sep = lines[i + 1].trim();
        if TABLE_SEP_RE.is_match(sep) && head.contains('|') {
            let header_cells = split_pipe_cells(head.trim_matches('|'));
            let sep_cells = split_pipe_cells(sep.trim_matches('|'));
            let column_count = header_cells.len();
            if sep_cells.len() != column_count {
                i += 1;
                continue;
            }
            let mut rows = Vec::new();
            let mut issues = Vec::new();
            i += 2;
            while i < lines.len() && lines[i].contains('|') {
                let row_line = lines[i].trim();
                let cells = split_pipe_cells(row_line.trim_matches('|'));
                let outer = row_line.starts_with('|') && row_line.ends_with('|');
                if cells.len() != column_count {
                    if outer {
                        issues.push(format!(
                            "mode-table has an invalid data row {}",
                            py_repr(row_line)
                        ));
                        i += 1;
                        continue;
                    }
                    break;
                }
                if !cells.is_empty() && MODE_CELL_RE.is_match(&cells[0]) {
                    rows.push(cells);
                    i += 1;
                    continue;
                }
                if outer {
                    issues.push(format!(
                        "mode-table has an invalid data row {}",
                        py_repr(row_line)
                    ));
                    i += 1;
                    continue;
                }
                break;
            }
            found.push((rows, issues));
        } else {
            i += 1;
        }
    }
    found
}
