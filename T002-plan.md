# T002 · Chore · P2: analyze current linting violations

## Analysis
This is a simple task to assess the current state of linting compliance across the project. The task requires:
1. Running the clippy linting tool with strict settings
2. Documenting the types and counts of lint violations found

## Implementation Plan
1. Run `cargo clippy --all-targets --all-features --workspace -- -D warnings` on the codebase
2. Capture and analyze the output of the clippy command
3. Document the violations by type and file
4. Count the total number of lint violations
5. Update the task in TODO.md

## Results

The codebase currently fails the clippy check completely due to a critical error in the `boringascii` crate. The issue is with clippy's strict warning about the implementation of `Hash` along with both `Borrow<str>` and `Borrow<[u8]>`.

### Major Issues

1. **boringascii/src/lib.rs** (Critical error - blocks compilation)
   - **Error**: `clippy::impl_hash_borrow_with_str_and_bytes`
   - **Description**: The crate implements `Hash` and both `Borrow<str>` and `Borrow<[u8]>`, which clippy considers unsafe since the hash implementations for strings and byte arrays differ.
   - **Line**: 13 (in the derive macro)
   - **Clippy message**: "the semantics of `Borrow<T>` around `Hash` can't be satisfied when both `Borrow<str>` and `Borrow<[u8]>` are implemented"
   - **Suggested fix**: Either remove one of the `Borrow` implementations or remove the `Hash` trait derivation.

Since the `boringascii` crate is a dependency for other crates in the workspace, the error causes compilation to fail for all crates, preventing a complete analysis of other potential linting issues.

### Dependency Chain Impact

The issue in `boringascii` is blocking the analysis of other crates as they depend on it:

- `zebra_crypto` depends on `boringascii`
- `zebra_storage` depends on `boringascii` and `zebra_crypto`
- `zebra_desktop` depends on the above crates

This means that we need to fix the issue in `boringascii` first before we can perform a thorough lint analysis on the other crates.

## Summary
- **Total critical errors**: 1
- **Total warnings**: Cannot be determined (blocked by the critical error)
- **Affected files**: 1 (boringascii/src/lib.rs)
- **Action needed**: Fix the `Hash` implementation in `boringascii` crate to allow full linting analysis of the codebase

The current linting state suggests that the primary focus should be on addressing the specific critical error that is blocking full analysis. Once this is resolved, a more comprehensive lint analysis can be performed.