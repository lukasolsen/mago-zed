use crate::diagnostics::build_mago_diagnostic;
use regex::Regex;
use std::sync::LazyLock;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

static HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(warning|error)\[([^\]]+)\]: (.+)$").expect("valid header regex")
});
static LOCATION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*┌─.*?:(\d+):(\d+)$").expect("valid location regex"));
static ANNOTATION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*│\s*(?:[\^-]+\s+)+(.*)$").expect("valid annotation regex")
});
static CARET_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*│.*?(\^+)").expect("valid caret regex"));
static HELP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*=\s+(Help:.*)").expect("valid help regex"));

/// Parses Mago textual output into LSP diagnostics for one command phase.
#[must_use]
pub fn parse_mago_output(stdout: &str, command_type: &str) -> Vec<Diagnostic> {
    let mut diagnostics = vec![];

    let headers: Vec<_> = HEADER_RE.captures_iter(stdout).collect();

    for (index, cap) in headers.iter().enumerate() {
        let severity = match &cap[1] {
            "error" => DiagnosticSeverity::ERROR,
            _ => DiagnosticSeverity::WARNING,
        };

        let code = cap[2].to_string();
        let mut full_message = cap[3].to_string();
        let Some(block_match) = cap.get(0) else {
            continue;
        };
        let block_start = block_match.start();
        let block_end = headers
            .get(index + 1)
            .and_then(|next| next.get(0))
            .map_or(stdout.len(), |next| next.start());
        let body = &stdout[block_start..block_end];

        let (line, col) = LOCATION_RE.captures(body).map_or((0, 0), |location| {
            (
                location[1].parse::<u32>().unwrap_or(1) - 1,
                location[2].parse::<u32>().unwrap_or(1) - 1,
            )
        });

        let error_length = CARET_RE
            .captures(body)
            .and_then(|capture| u32::try_from(capture[1].len()).ok())
            .unwrap_or(1);

        for ann_cap in ANNOTATION_RE.captures_iter(body) {
            let note = ann_cap[1].trim();
            if !note.is_empty() {
                full_message.push_str("\n\nNote: ");
                full_message.push_str(note);
            }
        }

        if let Some(help_cap) = HELP_RE.captures(body) {
            full_message.push_str("\n\n");
            full_message.push_str(help_cap[1].trim());
        }

        diagnostics.push(build_mago_diagnostic(
            line,
            col,
            error_length,
            severity,
            code,
            full_message,
            command_type,
        ));
    }

    diagnostics
}
