// Tauri commands take their deserialized IPC arguments by value.
#![allow(clippy::needless_pass_by_value)]

use std::fmt::Write as _;

use serde::{Deserialize, Serialize};
use sidex_context::format::toon::serialize_array;

/// A single result from the context search index, used for formatted output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSearchResult {
    pub file: String,
    pub line: usize,
    pub name: String,
    pub kind: String,
    pub score: f32,
    pub snippet: String,
}

/// Format context search results in the specified output format.
///
/// Supported formats: `"toon"` (default), `"scf"` (compact tabular), `"json"`.
#[tauri::command]
pub fn context_search_toon(
    results: Vec<ContextSearchResult>,
    format: Option<String>,
) -> Result<String, String> {
    let fmt = format.unwrap_or_else(|| "toon".to_string());

    match fmt.as_str() {
        "scf" => {
            let mut out = String::new();
            let _ = writeln!(out, "@search[{}] file|line|name|kind|score", results.len());
            for r in &results {
                let _ = writeln!(
                    out,
                    "{}|{}|{}|{}|{:.2}",
                    r.file, r.line, r.name, r.kind, r.score
                );
            }
            Ok(out)
        }
        "json" => serde_json::to_string(&results).map_err(|e| e.to_string()),
        _ => Ok(serialize_array("results", &results)),
    }
}

/// A diagnostic entry (error/warning) associated with a file location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticEntry {
    pub file: String,
    pub line: usize,
    pub severity: String,
    pub message: String,
    pub code: String,
}

/// Format diagnostic entries into TOON serialization for context injection.
// `Result` is kept for a consistent Tauri IPC error channel across the
// format commands in this file, even though this one never fails.
#[allow(clippy::unnecessary_wraps)]
#[tauri::command]
pub fn format_diagnostics_toon(diagnostics: Vec<DiagnosticEntry>) -> Result<String, String> {
    Ok(serialize_array("diagnostics", &diagnostics))
}

/// A file-tree entry with path, size, and kind (file/directory).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
    pub kind: String,
}

/// Format a flat file-tree listing into TOON serialization for context injection.
#[allow(clippy::unnecessary_wraps)]
#[tauri::command]
pub fn format_file_tree_toon(entries: Vec<FileEntry>) -> Result<String, String> {
    Ok(serialize_array("files", &entries))
}
