//! Tokenizes `tests/ui/*.stderr` fixtures into HTML for embedding in rustdoc.
//!
//! Output is written to `$OUT_DIR/diagnostics/<name>.html`. Source files
//! reference rendered fixtures with:
//!
//! ```ignore
//! #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/<name>.html"))]
//! ```
//!
//! The companion stylesheet lives at `docs/diagnostics.html` and is injected
//! into the rustdoc `<head>` via `--html-in-header`.

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let ui_dir = manifest_dir.join("tests").join("ui");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("diagnostics");
    fs::create_dir_all(&out_dir).expect("create OUT_DIR/diagnostics");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", ui_dir.display());

    let entries = match fs::read_dir(&ui_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("stderr") {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path.display());
        let stderr = fs::read_to_string(&path).expect("read stderr fixture");
        let html = render(&stderr);
        let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
        fs::write(out_dir.join(format!("{stem}.html")), html).expect("write diagnostic html");
    }
}

fn render(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2 + 64);
    out.push_str("\n\n<pre class=\"rustc-diag\">");
    let mut level: &'static str = "error";
    let mut first = true;
    for line in input.split('\n') {
        if !first {
            out.push('\n');
        }
        first = false;
        render_line(line, &mut level, &mut out);
    }
    while out.ends_with('\n') {
        out.pop();
    }
    out.push_str("</pre>\n\n");
    out
}

fn render_line(line: &str, level: &mut &'static str, out: &mut String) {
    if line.is_empty() {
        return;
    }
    if try_header(line, level, out) {
        return;
    }
    if try_location(line, out) {
        return;
    }
    if try_continuation(line, level, out) {
        return;
    }
    if line.trim_end() == "..." {
        push_span(out, "rd-gutter", line);
        return;
    }
    if try_diff(line, out) {
        return;
    }
    if try_gutter(line, *level, out) {
        return;
    }
    push_escaped(out, line);
}

fn level_class(level: &str) -> &'static str {
    match level {
        "error" => "rd-error",
        "warn" => "rd-warn",
        "note" => "rd-note",
        "help" => "rd-help",
        _ => "rd-error",
    }
}

fn try_header(line: &str, level: &mut &'static str, out: &mut String) -> bool {
    let (kw, lvl): (&str, &'static str) = if line.starts_with("error") {
        ("error", "error")
    } else if line.starts_with("warning") {
        ("warning", "warn")
    } else if line.starts_with("note") {
        ("note", "note")
    } else if line.starts_with("help") {
        ("help", "help")
    } else {
        return false;
    };
    let rest = &line[kw.len()..];
    let (code, after_code) = if rest.starts_with('[') {
        match rest.find(']') {
            Some(end) => (Some(&rest[..=end]), &rest[end + 1..]),
            None => return false,
        }
    } else {
        (None, rest)
    };
    if !after_code.starts_with(':') {
        return false;
    }
    *level = lvl;
    let class = level_class(lvl);
    out.push_str("<span class=\"");
    out.push_str(class);
    out.push_str("\">");
    out.push_str(kw);
    if let Some(c) = code {
        push_escaped(out, c);
    }
    out.push_str("</span>");
    push_escaped(out, after_code);
    true
}

fn try_location(line: &str, out: &mut String) -> bool {
    let trimmed = line.trim_start();
    let lead_len = line.len() - trimmed.len();
    let (marker, rest) = if let Some(r) = trimmed.strip_prefix("--> ") {
        ("-->", r)
    } else if let Some(r) = trimmed.strip_prefix("::: ") {
        (":::", r)
    } else {
        return false;
    };
    out.push_str(&line[..lead_len]);
    push_span(out, "rd-loc", marker);
    out.push(' ');
    push_escaped(out, rest);
    true
}

fn try_continuation(line: &str, level: &mut &'static str, out: &mut String) -> bool {
    let trimmed = line.trim_start();
    let lead_len = line.len() - trimmed.len();
    let rest = match trimmed.strip_prefix("= ") {
        Some(r) => r,
        None => return false,
    };
    let (kw, lvl): (&str, &'static str) = if rest.starts_with("note:") {
        ("note:", "note")
    } else if rest.starts_with("help:") {
        ("help:", "help")
    } else {
        return false;
    };
    *level = lvl;
    out.push_str(&line[..lead_len]);
    push_span(out, "rd-gutter", "=");
    out.push(' ');
    push_span(out, level_class(lvl), kw);
    push_escaped(out, &rest[kw.len()..]);
    true
}

fn try_diff(line: &str, out: &mut String) -> bool {
    let trimmed = line.trim_start();
    let lead_len = line.len() - trimmed.len();
    let (num, after) = match trimmed.split_once(' ') {
        Some(p) => p,
        None => return false,
    };
    if num.is_empty() || !num.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let (sign, body, class) = if let Some(b) = after.strip_prefix("- ") {
        ("-", b, "rd-remove")
    } else if let Some(b) = after.strip_prefix("+ ") {
        ("+", b, "rd-add")
    } else {
        return false;
    };
    out.push_str(&line[..lead_len]);
    push_span(out, "rd-gutter", num);
    out.push(' ');
    out.push_str("<span class=\"");
    out.push_str(class);
    out.push_str("\">");
    out.push_str(sign);
    out.push(' ');
    push_escaped(out, body);
    out.push_str("</span>");
    true
}

fn try_gutter(line: &str, level: &'static str, out: &mut String) -> bool {
    let pipe_pos = match line.find('|') {
        Some(p) => p,
        None => return false,
    };
    let gutter = &line[..pipe_pos];
    if !gutter.bytes().all(|b| b == b' ' || b.is_ascii_digit()) {
        return false;
    }
    let after = &line[pipe_pos + 1..];
    let line_no_present = gutter.bytes().any(|b| b.is_ascii_digit());

    out.push_str("<span class=\"rd-gutter\">");
    out.push_str(gutter);
    out.push('|');
    out.push_str("</span>");

    render_after_gutter(after, level, line_no_present, out);
    true
}

fn render_after_gutter(after: &str, level: &'static str, has_line_no: bool, out: &mut String) {
    if after.is_empty() {
        return;
    }
    let level_cls = level_class(level);

    if has_line_no {
        let bytes = after.as_bytes();
        if bytes.len() >= 3
            && bytes[0] == b' '
            && (bytes[1] == b'/' || bytes[1] == b'|' || bytes[1] == b'\\')
            && bytes[2] == b' '
        {
            out.push(' ');
            push_span(out, level_cls, &after[1..2]);
            out.push(' ');
            push_escaped(out, &after[3..]);
            return;
        }
        push_escaped(out, after);
        return;
    }

    if after.bytes().all(|b| b == b' ') {
        out.push_str(after);
        return;
    }
    render_annotation(after, level_cls, out);
}

fn render_annotation(rest: &str, level_cls: &'static str, out: &mut String) {
    let bytes = rest.as_bytes();
    let mut i = 0;
    let mut last_class: Option<&'static str> = None;

    while i < bytes.len() {
        let c = bytes[i];
        if c == b' ' {
            let start = i;
            while i < bytes.len() && bytes[i] == b' ' {
                i += 1;
            }
            out.push_str(&rest[start..i]);
            continue;
        }
        if let Some(cls) = symbol_class(c, level_cls) {
            let start = i;
            while i < bytes.len() && symbol_class(bytes[i], level_cls) == Some(cls) {
                i += 1;
            }
            push_span(out, cls, &rest[start..i]);
            last_class = Some(cls);
            continue;
        }
        let label_class = last_class.unwrap_or("rd-secondary");
        push_span(out, label_class, &rest[i..]);
        i = bytes.len();
    }
}

fn symbol_class(b: u8, level_cls: &'static str) -> Option<&'static str> {
    match b {
        b'^' | b'_' | b'/' | b'\\' | b'|' => Some(level_cls),
        b'-' => Some("rd-secondary"),
        b'+' | b'~' => Some("rd-add"),
        _ => None,
    }
}

fn push_span(out: &mut String, class: &str, content: &str) {
    out.push_str("<span class=\"");
    out.push_str(class);
    out.push_str("\">");
    push_escaped(out, content);
    out.push_str("</span>");
}

fn push_escaped(out: &mut String, s: &str) {
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(c),
        }
    }
}
