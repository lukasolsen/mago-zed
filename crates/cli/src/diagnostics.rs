use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

#[must_use]
pub fn build_mago_diagnostic(
    line: u32,
    col: u32,
    error_length: u32,
    severity: DiagnosticSeverity,
    code: String,
    message: String,
    command_type: &str,
) -> Diagnostic {
    Diagnostic {
        range: Range::new(Position::new(line, col), Position::new(line, col + error_length)),
        severity: Some(severity),
        code: Some(NumberOrString::String(code)),
        source: Some(format!("mago {command_type}")),
        message,
        ..Default::default()
    }
}
