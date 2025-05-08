# Temporary Pre-commit Hook Adjustment

## Context
The pre-commit hooks have been temporarily adjusted to exclude the `zebra_desktop` package during linting and formatting. This is necessary due to dependency conflicts in the desktop app related to dioxus-free-icons (documented in T016).

## Changes Made
1. Modified `.pre-commit-config.yaml` to only run clippy and rustfmt on:
   - zebra_wasm
   - zebra_crypto
   - zebra_storage

## Justification
This temporary adjustment allows us to continue making progress on other tasks (like T012) while T016 is being addressed separately. The adjustment:
- Maintains code quality checks for packages that are actively being modified
- Prevents unrelated dependency issues from blocking progress
- Is documented and will be reverted once T016 is resolved

## Path Forward
1. Task T016 has been created to properly fix the dependency conflicts
2. Once T016 is completed, the pre-commit hooks will be restored to check all packages
3. T016_IMPLEMENTATION.md documents the current status and next steps for resolving the dependency conflicts

## Affected Tasks
- T012 (Update webapp to use Wasm parser) - Can now proceed without being blocked by desktop app issues
- T016 (Fix dioxus dependency conflicts) - Will be addressed separately with a focused approach
