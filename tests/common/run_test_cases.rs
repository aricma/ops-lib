use super::error_kind::ErrorKind;
use super::expectation::Expectation;
use super::round_trip_test_case::RoundTripTestCase;
use super::test_case::TestCase;
use ops::{Format, Task};

/// The generic case engine: run one table of test cases, delegating
/// the per-case check to `check`.
pub fn run_test_cases<G, E>(cases: &[TestCase<G, E>], check: impl Fn(&G, &E, &str)) {
    for c in cases {
        check(&c.given, &c.expected, &c.message);
    }
}

/// The parse check: valid documents must parse (optionally into an
/// exact model), invalid ones must reject with the expected error
/// class.
pub fn run_parse_cases(cases: &[TestCase<(Format, String), Expectation>]) {
    run_test_cases(cases, |given, expected, message| match expected {
        Expectation::Model(model) => {
            assert_eq!(ops::read(given.0, &given.1).unwrap(), *model, "{message}")
        }
        Expectation::Error(kind) => {
            let err = ops::read(given.0, &given.1).unwrap_err();
            assert_eq!(ErrorKind::from(&err), *kind, "{message}");
        }
    });
}

/// The write check: serializing a model and parsing it back must yield
/// the expected model — the OPS lossless contract per format.
pub fn run_write_roundtrip_cases(cases: &[TestCase<(Format, Task), Vec<Task>>]) {
    run_test_cases(cases, |given, expected, message| {
        let out = ops::write(given.0, std::slice::from_ref(&given.1)).unwrap();
        assert_eq!(ops::read(given.0, &out).unwrap(), *expected, "{message}");
    });
}

/// A round-trip that must reach a fixed point: parse → write produces
/// a document, and writing the re-parsed document must reproduce it
/// character for character (the OPS lossless contract).
pub fn run_roundtrip_passes(cases: &[RoundTripTestCase]) {
    for c in cases {
        let first = ops::read(c.given.0, &c.given.1)
            .unwrap_or_else(|e| panic!("{}: parse failed: {e}", c.message));
        let out1 = ops::write(c.given.0, &first)
            .unwrap_or_else(|e| panic!("{}: write failed: {e}", c.message));
        let second = ops::read(c.given.0, &out1)
            .unwrap_or_else(|e| panic!("{}: re-parse failed: {e}", c.message));
        let out2 = ops::write(c.given.0, &second)
            .unwrap_or_else(|e| panic!("{}: second write failed: {e}", c.message));
        assert_eq!(
            out2, out1,
            "{}: writing the re-parsed document changed the output",
            c.message
        );
    }
}

/// A round-trip that must fail at the parse step (invalid documents).
pub fn run_roundtrip_fails(cases: &[RoundTripTestCase]) {
    for c in cases {
        assert!(
            ops::read(c.given.0, &c.given.1).is_err(),
            "{}: must be rejected",
            c.message
        );
    }
}
