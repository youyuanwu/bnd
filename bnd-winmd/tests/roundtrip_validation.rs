//! Tests for type-reference validation — verifies that bnd-winmd catches
//! unresolved Named types before emitting the winmd, instead of letting
//! windows-bindgen fail later with a cryptic "type not found" panic.

use std::path::Path;
use std::sync::LazyLock;

/// The unresolved fixture should fail with a validation error.
static UNRESOLVED_RESULT: LazyLock<Result<Vec<u8>, String>> = LazyLock::new(|| {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/unresolved/unresolved.toml");
    bnd_winmd::generate(&path).map_err(|e| format!("{e:#}"))
});

#[test]
fn unresolved_type_reference_is_caught() {
    let err = UNRESOLVED_RESULT
        .as_ref()
        .expect_err("should fail due to unresolved type reference");

    // The error should mention the missing type name.
    assert!(
        err.contains("DefinedElsewhere"),
        "error should mention the unresolved type name 'DefinedElsewhere', got:\n{err}"
    );

    // The error should mention the function that references it.
    assert!(
        err.contains("use_external"),
        "error should mention the referencing function 'use_external', got:\n{err}"
    );

    // The error should contain the actionable hint.
    assert!(
        err.contains("traverse"),
        "error should mention 'traverse' list as a fix, got:\n{err}"
    );
}

#[test]
fn unresolved_does_not_report_known_types() {
    let err = UNRESOLVED_RESULT
        .as_ref()
        .expect_err("should fail due to unresolved type reference");

    // KnownStruct is defined in the traversed header — should NOT appear.
    assert!(
        !err.contains("KnownStruct"),
        "error should NOT mention 'KnownStruct' (it's properly traversed), got:\n{err}"
    );
}
