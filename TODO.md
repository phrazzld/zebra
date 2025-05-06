# Todo

## Code Quality Automation - Preparation
- [x] **T001 · Chore · P2: analyze current formatting violations**
    - **Context:** PLAN.md - Preparation / Risk Assessment (Implied)
    - **Action:**
        1. Run `cargo fmt --all --check` on the current `main` branch.
        2. Document the number of files needing formatting.
    - **Done‑when:**
        1. Command executed and results documented.
    - **Depends‑on:** none

- [x] **T002 · Chore · P2: analyze current linting violations**
    - **Context:** PLAN.md - Preparation / Risk Assessment (Implied)
    - **Action:**
        1. Run `cargo clippy --all-targets --all-features --workspace -- -D warnings` (or final strict command) on the current `main` branch.
        2. Document the types and counts of lint violations.
    - **Done‑when:**
        1. Command executed and results documented.
    - **Depends‑on:** none

- [x] **T003 · Refactor · P1: fix existing formatting issues**
    - **Context:** PLAN.md - Preparation / Risk Assessment (Implied)
    - **Action:**
        1. Run `cargo fmt --all` across the workspace based on findings from T001.
        2. Review and commit the formatting changes.
    - **Done‑when:**
        1. `cargo fmt --all --check` passes on the `main` branch using the config from T005.
    - **Depends‑on:** [T001, T005]

- [x] **T004 · Refactor · P1: fix existing linting issues**
    - **Context:** PLAN.md - Preparation / Risk Assessment (Implied)
    - **Action:**
        1. Address lint violations identified in T002 by fixing code.
        2. Justify and document any required `#[allow(...)]` suppressions per T006.
        3. Commit the fixes.
    - **Done‑when:**
        1. `cargo clippy --all-targets --all-features --workspace -- -D warnings` (or final strict command) passes on the `main` branch.
    - **Depends‑on:** [T002, T006]

## Code Quality Automation - Configuration
- [x] **T005 · Chore · P0: configure rustfmt settings**
    - **Context:** PLAN.md - Phase 1: Configure Formatting and Linting Tools
    - **Action:**
        1. Create or update `.rustfmt.toml` in the repository root.
        2. Set `edition = "2021"` and `max_width = 100`. Add other agreed-upon minimal settings.
        3. Commit the file.
    - **Done‑when:**
        1. `.rustfmt.toml` exists with the specified configuration.
    - **Verification:**
        1. Run `cargo fmt --all --check` passes on code known to be formatted correctly.
    - **Depends‑on:** none

- [x] **T006 · Chore · P0: define and document clippy configuration and allowances**
    - **Context:** PLAN.md - Phase 1: Configure Formatting and Linting Tools
    - **Action:**
        1. Define the strict clippy command (e.g., `cargo clippy --all-targets --all-features --workspace -- -D warnings`).
        2. Evaluate and document any necessary lint allowances based on project needs or T002 findings, providing justification for each.
        3. Decide if a `clippy.toml` is needed or if flags suffice for pre-commit/CI.
    - **Done‑when:**
        1. Standard clippy command is defined.
        2. Required lint allowances are documented with rationale.
    - **Verification:**
        1. Run the defined clippy command locally to confirm it applies the desired strictness level and allowances.
    - **Depends‑on:** [T002]

## Code Quality Automation - Tooling Setup
- [x] **T007 · Feature · P0: implement pre-commit hooks for rustfmt and clippy**
    - **Context:** PLAN.md - Phase 2: Implement Pre-commit Hooks
    - **Action:**
        1. Create `.pre-commit-config.yaml` in the repository root.
        2. Add hooks for `rustfmt` and `clippy`, ensuring arguments match T005/T006 configs.
        3. Include basic file checks (e.g., `check-yaml`, `end-of-file-fixer`, `trailing-whitespace`).
    - **Done‑when:**
        1. `.pre-commit-config.yaml` exists and is valid.
        2. `pre-commit run --all-files` executes successfully on compliant code.
    - **Verification:**
        1. Commit non-compliant code (fmt error); verify hook blocks commit.
        2. Commit non-compliant code (lint error); verify hook blocks commit.
        3. Commit compliant code; verify commit succeeds.
    - **Depends‑on:** [T005, T006]

- [x] **T008 · Test · P0: create CI job for rustfmt check**
    - **Context:** PLAN.md - Phase 3: Integrate into CI Pipeline (Format Job)
    - **Action:**
        1. Create/update `.github/workflows/rust_quality.yml`.
        2. Add a job (`format`) that runs `cargo fmt --all --check` using the config from T005.
        3. Configure triggers (e.g., `on: [push, pull_request]`).
    - **Done‑when:**
        1. Workflow file includes the `format` job.
        2. CI job executes on triggers and passes/fails correctly based on formatting.
    - **Verification:**
        1. Push branch with formatting error; verify CI job fails.
        2. Push branch with correct formatting; verify CI job passes.
    - **Depends‑on:** [T005]

- [x] **T009 · Test · P0: create CI job for clippy check**
    - **Context:** PLAN.md - Phase 3: Integrate into CI Pipeline (Lint Job)
    - **Action:**
        1. Update `.github/workflows/rust_quality.yml`.
        2. Add a job (`lint`) that runs the strict clippy command defined in T006.
        3. Configure triggers (e.g., `on: [push, pull_request]`).
    - **Done‑when:**
        1. Workflow file includes the `lint` job.
        2. CI job executes on triggers and passes/fails correctly based on lint violations.
    - **Verification:**
        1. Push branch with lint violation; verify CI job fails.
        2. Push branch with compliant code; verify CI job passes.
    - **Depends‑on:** [T006]

- [x] **T010 · Chore · P1: enable branch protection rules for CI checks**
    - **Context:** PLAN.md - Phase 3: Integrate into CI Pipeline (Enforcement)
    - **Action:**
        1. Configure GitHub repository settings for the `main` branch.
        2. Require the `format` and `lint` status checks (from T008, T009) to pass before merging.
    - **Done‑when:**
        1. Branch protection rules require `format` and `lint` checks to pass for `main`.
    - **Verification:**
        1. Create a PR failing a required check; verify merge is blocked.
        2. Create a PR passing all required checks; verify merge is allowed.
    - **Depends‑on:** [T008, T009]

## Code Quality Automation - Documentation & Testing
- [x] **T011 · Chore · P1: update README.md with setup instructions**
    - **Context:** PLAN.md - Phase 4: Update Documentation
    - **Action:**
        1. Add/Update a "Development Prerequisites" section in `README.md`.
        2. Include instructions for installing Rust, `pre-commit`, and running `pre-commit install`.
    - **Done‑when:**
        1. `README.md` contains clear, accurate setup instructions.
    - **Verification:**
        1. Follow instructions on a clean clone; verify tools install and hooks activate correctly.
    - **Depends‑on:** [T007]

- [x] **T012 · Chore · P1: update CONTRIBUTING.md with code quality standards**
    - **Context:** PLAN.md - Phase 4: Update Documentation
    - **Action:**
        1. Create or update `CONTRIBUTING.md`.
        2. Document the mandatory `rustfmt` and `clippy` standards, the strictness level, enforcement mechanisms (hooks, CI), and how to fix common issues.
        3. Explain the policy on lint suppressions (`#[allow(...)]`) based on T006.
    - **Done‑when:**
        1. `CONTRIBUTING.md` clearly details the code quality standards and procedures.
    - **Depends‑on:** [T005, T006, T007, T008, T009]

- [ ] **T013 · Test · P1: test pre-commit hook functionality**
    - **Context:** PLAN.md - Testing Strategy
    - **Action:**
        1. Attempt commits with deliberate formatting errors; verify rejection.
        2. Attempt commits with deliberate lint violations; verify rejection.
        3. Attempt commits with compliant code; verify success.
    - **Done‑when:**
        1. Pre-commit hooks reliably block non-compliant code and allow compliant code.
    - **Verification:**
        1. Manual testing covering different violation scenarios.
    - **Depends‑on:** [T003, T004, T007]

- [ ] **T014 · Test · P1: test CI pipeline functionality**
    - **Context:** PLAN.md - Testing Strategy
    - **Action:**
        1. Create a PR with formatting errors; verify `format` CI job fails.
        2. Create a PR with lint violations; verify `lint` CI job fails.
        3. Create a PR with compliant code; verify both CI jobs pass.
    - **Done‑when:**
        1. CI pipeline correctly identifies and reports formatting/linting issues.
    - **Verification:**
        1. Check GitHub Actions logs and PR status checks for expected outcomes.
    - **Depends‑on:** [T003, T004, T008, T009]

- [ ] **T015 · Test · P2: verify configuration consistency between pre-commit and CI**
    - **Context:** PLAN.md - Testing Strategy
    - **Action:**
        1. Compare `rustfmt` settings used locally (T005) and in CI (T008).
        2. Compare `clippy` command/flags used locally (T007) and in CI (T009).
        3. Run checks on identical code locally and in CI to ensure identical results.
    - **Done‑when:**
        1. Configurations are confirmed to be identical or differences are justified and understood.
        2. Local and CI checks produce the same pass/fail results for the same code.
    - **Depends‑on:** [T007, T008, T009]

## Code Quality Automation - Rollout
- [ ] **T016 · Chore · P2: communicate changes and provide rollout support**
    - **Context:** PLAN.md - Rollout
    - **Action:**
        1. Announce the new code quality standards and tooling to the development team.
        2. Point team members to documentation (T011, T012) for setup and guidelines.
        3. Be available to assist with `pre-commit` setup or troubleshooting.
    - **Done‑when:**
        1. Team is informed of the new standards and how to comply.
    - **Depends‑on:** [T010, T011, T012]

---

### Clarifications & Assumptions
- [ ] **Issue:** Which specific clippy lints need to be allowed/suppressed project-wide, and what is the justification?
    - **Context:** PLAN.md "Define Clippy Configuration"; Required for T004, T006, T007, T009, T012.
    - **Blocking?:** yes (for completing T006 and subsequent dependent tasks accurately)
- [ ] **Issue:** Are there specific files or directories (e.g., auto-generated code, vendor code) that should be excluded from formatting and/or linting checks?
    - **Context:** PLAN.md (General Scope); Affects T003, T004, T007, T008, T009.
    - **Blocking?:** no (can proceed with full checks initially, but clarification needed for optimal setup)
