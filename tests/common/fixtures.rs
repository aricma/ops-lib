//! Fixture paths, loading, and round-trip case building.

use super::round_trip_test_case::RoundTripTestCase;
use ops::Format;
use std::path::{Path, PathBuf};

pub const OPS_TESTS: &str = "tests/fixtures/ops/tests";
pub const OPS_EXAMPLES: &str = "tests/fixtures/ops/examples";
pub const LOCAL: &str = "tests/fixtures/local";

/// Read one fixture from a fixture directory constant.
pub fn fixture(dir: &str, name: &str) -> String {
    std::fs::read_to_string(format!("{dir}/{name}")).unwrap()
}

/// Map a fixture file name to its format.
pub fn format_of(path: &Path) -> Format {
    match path.extension().and_then(|e| e.to_str()).unwrap() {
        "json" => Format::Json,
        "yaml" | "yml" => Format::Yaml,
        "md" | "markdown" => Format::Markdown,
        "xml" => Format::Xml,
        "jsonl" | "ndjson" => Format::Jsonl,
        "csv" => Format::Csv,
        other => panic!("unknown fixture extension: {other}"),
    }
}

pub fn files_under(dir: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            out.extend(files_under(&path.to_string_lossy()));
        } else {
            out.push(path);
        }
    }
    out
}

/// The valid or invalid fixture paths from the OPS set (JSON + JSONL).
pub fn fixture_files(kind: &str) -> Vec<PathBuf> {
    files_under(OPS_TESTS)
        .into_iter()
        .filter(|p| {
            p.to_string_lossy().contains(&format!("/{kind}/"))
                && matches!(
                    p.extension().and_then(|e| e.to_str()),
                    Some("json" | "jsonl")
                )
        })
        .collect()
}

/// The example documents (everything under `ops/examples`).
pub fn example_files() -> Vec<PathBuf> {
    files_under(OPS_EXAMPLES)
        .into_iter()
        .filter(|p| p.file_name().is_some_and(|n| n != "README.md"))
        .collect()
}

/// Turn fixture paths into round-trip cases.
pub fn fixture_roundtrips(paths: Vec<PathBuf>) -> Vec<RoundTripTestCase> {
    paths
        .into_iter()
        .map(|path| RoundTripTestCase {
            message: path.file_name().unwrap().to_str().unwrap().to_string(),
            given: (format_of(&path), std::fs::read_to_string(&path).unwrap()),
        })
        .collect()
}
