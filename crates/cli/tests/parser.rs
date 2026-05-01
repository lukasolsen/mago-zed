use cli::parse_mago_output;
use tower_lsp::lsp_types::{DiagnosticSeverity, NumberOrString};

#[test]
fn test_parse_mago_analyze_output() {
    let raw_output = r#"
error[possibly-invalid-argument]: The first value for `require` might have the wrong type.
  ┌─ examples/test.php:7:9
  │
7 │ require $_GET["template"]; // RFI
  │ ------- ^^^^^^^^^^^^^^^^^ This is `non-empty-array<int|non-empty-string, array<int|non-empty-string, mixed>|string>|string`, which only sometimes overlaps with `string`
  │ │
  │ `require` called here
  │
  = Help: Add a type check to ensure the value is what you expect.

error: found 1 issues: 1 error(s)
"#;

    let diagnostics = parse_mago_output(raw_output, "analyze");

    assert_eq!(diagnostics.len(), 1);

    let diag = &diagnostics[0];

    assert_eq!(diag.severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(diag.range.start.line, 6);
    assert_eq!(diag.range.start.character, 8);

    assert!(diag.message.contains("The first value for `require` might have the wrong type."));
    assert!(diag.message.contains("Note: This is `non-empty-array"));
    assert!(diag.message.contains("Help: Add a type check"));

    assert_eq!(diag.code, Some(NumberOrString::String("possibly-invalid-argument".to_string())));
    assert_eq!(diag.source, Some("mago analyze".to_string()));
}

#[test]
fn test_parse_mago_lint_output() {
    let raw_output = r"
warning[strict-types]: Missing `declare(strict_types=1);` statement at the beginning of the file.
  ┌─ examples/test.php:1:1
  │
1 │ <?php
  │ ^^^^^
  │
   = The `strict_types` directive enforces strict type checking, which can prevent subtle bugs.
   = Help: Add `declare(strict_types=1);` at the top of your file.
";

    let diagnostics = parse_mago_output(raw_output, "lint");

    assert_eq!(diagnostics.len(), 1);
    let diag = &diagnostics[0];

    assert_eq!(diag.severity, Some(DiagnosticSeverity::WARNING));
    assert_eq!(diag.range.start.line, 0);
    assert_eq!(diag.range.start.character, 0);
    assert!(diag.message.contains("Missing `declare(strict_types=1);`"));
    assert!(diag.message.contains("Help: Add `declare(strict_types=1);`"));
}
