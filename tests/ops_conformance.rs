//! OPS conformance, at the level of rough guidelines rather than exact
//! error classes: every example must survive the round-trip, every
//! `valid/` fixture must round-trip without failing, every `invalid/`
//! fixture must be rejected. Fixtures: `tests/fixtures/ops/tests`,
//! vendored from the open-productivity-standard repo (see
//! tests/fixtures/README.md).

mod common;

use common::{
    example_files, fixture_files, fixture_roundtrips, run_roundtrip_fails, run_roundtrip_passes,
};

#[test]
fn all_examples_roundtrip_to_an_identical_document() {
    let cases = fixture_roundtrips(example_files());
    assert_eq!(cases.len(), 10, "all example documents must be found");
    run_roundtrip_passes(&cases);
}

#[test]
fn all_valid_fixtures_roundtrip_without_failing() {
    let cases = fixture_roundtrips(fixture_files("valid"));
    assert_eq!(cases.len(), 23, "all valid fixtures must be found");
    run_roundtrip_passes(&cases);
}

#[test]
fn all_invalid_fixtures_are_rejected_by_the_roundtrip() {
    let cases = fixture_roundtrips(fixture_files("invalid"));
    assert_eq!(cases.len(), 21, "all invalid fixtures must be found");
    run_roundtrip_fails(&cases);
}
