# Clippy Configuration for ZebraSign

This document defines the clippy linting configuration for the ZebraSign project, including the standard command to use, allowed lints, and the justification for any allowances.

## Standard Clippy Command

The standard clippy command for this project is:

```bash
cargo clippy --all-targets --all-features --workspace -- -D warnings
```

This command:
- Checks all targets (libraries, binaries, tests, examples, benches)
- Checks all features
- Checks all crates in the workspace
- Treats all warnings as errors (`-D warnings`)

This command should be used consistently across all environments, including local development, pre-commit hooks, and CI pipelines.

## Configuration

The project uses a `clippy.toml` file in the repository root to configure specific lint settings. This approach was chosen over command-line flags for several reasons:

1. **Consistency**: Ensures the same configuration is used regardless of how clippy is invoked
2. **Documentation**: Provides a central place to document allowances and their justifications
3. **Maintainability**: Makes it easier to update and review the linting configuration

## Lint Allowances

Per our Development Philosophy, lint suppressions are **strongly discouraged**. Allowances are granted only in exceptional cases where:

1. The lint produces a false positive
2. The flagged pattern is actually the most correct solution for our specific case
3. The suggested alternative would reduce code quality or correctness

### Current Allowances

| Lint | Location | Justification |
|------|----------|---------------|
| `clippy::impl_hash_borrow_with_str_and_bytes` | `boringascii/src/lib.rs` | The `BoringAscii` type needs to be usable in hash-based collections while also supporting borrowing as both `&str` and `&[u8]` for API convenience. This is safe because `BoringAscii`'s constructor ensures that only valid ASCII is stored, making the `&str` and `&[u8]` views semantically equivalent for hashing purposes. The type is carefully constructed to ensure this invariant is maintained. |

## Common Linting Issues Fixed

During the initial review, several linting issues were identified and fixed to improve code quality:

1. **In `boringascii/src/lib.rs`**:
   - Fixed the `Hash` implementation with `Borrow<str>` and `Borrow<[u8]>` conflict by adding a justified allowance
   - Updated test assertions to use `is_none()` instead of comparing to `None`
   - Improved byte string literal syntax in tests

2. **In `zebra_crypto/src/lib.rs`**:
   - Replaced direct `ToString` implementation with `Display`
   - Optimized cloning of iterator items by moving `cloned()` after filtering
   - Removed unnecessary `to_string()` call in format arguments

3. **In `zebra_storage/src/lib.rs`**:
   - Added explicit `truncate` behavior to `OpenOptions` configurations

4. **In `zebra_wasm/Cargo.toml` and related files**:
   - Added and configured proper feature flags for optional dependencies

## Review Process for New Allowances

If a new allowance is needed, follow this process:

1. **Attempt to fix the issue** first without suppression
2. If suppression is necessary, document in the code:
   - Use the `// ALLOWANCE: Reason...` comment format
   - Explain why the allowance is necessary
   - Reference any related tickets or decisions
3. Update this document and `clippy.toml` with the new allowance
4. Include the changes in code review

## Verification

To verify the clippy configuration is working correctly:

```bash
# Run clippy with all settings
cargo clippy --all-targets --all-features --workspace -- -D warnings

# The command should complete successfully with no errors
```
