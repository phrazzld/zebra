# T004 · Refactor · P1: fix existing linting issues

## Analysis
This task involves fixing all linting issues identified by clippy across the codebase. I ran the clippy command to identify current linting issues:

```
cargo clippy --all-targets --all-features --workspace -- -D warnings
```

## Findings
All linting issues have already been resolved! Running clippy returns cleanly with no warnings or errors. The only output is a cargo-specific warning about profiles in non-root packages, which is not a code linting issue:

```
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/phaedrus/Development/zebra/zebra_wasm/Cargo.toml
workspace: /Users/phaedrus/Development/zebra/Cargo.toml
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
```

## Previous Work
Based on examining the codebase and documentation, the linting issues were fixed during the implementation of T006 (define and document clippy configuration and allowances). The following fixes were applied:

1. In `boringascii/src/lib.rs`:
   - Added a justified allowance for `clippy::impl_hash_borrow_with_str_and_bytes` with thorough documentation
   - Updated test assertions to use `is_none()` instead of comparing to `None`
   - Improved byte string literal syntax in tests

2. In `zebra_crypto/src/lib.rs`:
   - Replaced direct `ToString` implementation with `Display`
   - Optimized cloning of iterator items by moving `cloned()` after filtering
   - Removed unnecessary `to_string()` call in format arguments

3. In `zebra_storage/src/lib.rs`:
   - Added explicit `truncate` behavior to `OpenOptions` configurations

4. In feature flag configurations:
   - Added and properly configured feature flags for optional dependencies in various crates

All of these fixes were documented in `docs/CLIPPY_CONFIGURATION.md`.

## Summary
The task of fixing existing linting issues (T004) has already been completed as part of the work done for T006. The codebase now passes clippy checks with the strict configuration defined in clippy.toml.

No additional code changes are required for this ticket.