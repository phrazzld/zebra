# Todo

## Dependency Management
- [x] **T001 · Bugfix · P0: Verify and secure dioxus-free-icons dependency source**
    - **Context:** REMEDIATION_PLAN.md - Critical Priority 1: Dependency source change
    - **Action:**
        1. Contact PR author to confirm intent for switching `dioxus-free-icons` source from `benwr` to `SimonBaars`.
        2. Audit `SimonBaars/dioxus-free-icons` repository:
           - Determine if it's a fork of the original repository
           - Assess repository health (activity, issues, PRs)
           - Review commit history and code diffs
           - Check for suspicious code or significant deviations
        3. Run `cargo audit` to check for known vulnerabilities related to this dependency.
        4. Pin the dependency to a specific verified commit hash in `zebra_desktop/Cargo.toml` with explanatory comment.
        5. Run `cargo update -p dioxus-free-icons` to update `Cargo.lock`.
    - **Done‑when:**
        1. Intent of source change is understood and documented.
        2. Repository audit is completed and findings documented.
        3. Dependency is pinned to specific commit in `Cargo.toml` with justification comment.
        4. `Cargo.lock` is updated accordingly.
        5. Project builds successfully and `cargo audit` reports no new critical vulnerabilities.
        6. UI features using icons function correctly.
    - **Verification:**
        1. Review `Cargo.toml` for pinned dependency and clear justification.
        2. Run `cargo build` and `cargo test` successfully.
        3. Run `cargo audit` and confirm no new critical vulnerabilities.
        4. Manually test UI components using icons.
    - **Depends‑on:** none

## CI/CD
- [x] **T002 · Feature · P0: Add cargo test job to CI workflow**
    - **Context:** REMEDIATION_PLAN.md - Critical Priority 2: Add test job to CI
    - **Action:**
        1. Edit `.github/workflows/rust_quality.yml` to add a new `test` job.
        2. Configure job to:
           - Checkout code
           - Setup Rust toolchain
           - Run `cargo test --all-targets --all-features --workspace`
        3. Ensure this job is added to branch protection rules.
    - **Done‑when:**
        1. `test` job is configured correctly in `rust_quality.yml`.
        2. CI executes tests on PRs and commits.
        3. Test job properly reports pass/fail status.
        4. Branch protection settings require test job to pass.
    - **Verification:**
        1. Push an update to a branch and confirm the job runs.
        2. Intentionally create a failing test to verify CI fails.
        3. Check branch protection settings.
    - **Depends‑on:** none

- [ ] **T003 · Chore · P2: Add dependency caching to CI workflow**
    - **Context:** REMEDIATION_PLAN.md - Medium Priority 5: CI caching
    - **Action:**
        1. Edit `.github/workflows/rust_quality.yml` to add caching to all jobs.
        2. Add `actions/cache@v3` steps to `format`, `lint`, and `test` jobs.
        3. Configure cache paths for:
           - `~/.cargo/bin/`
           - `~/.cargo/registry/index/`
           - `~/.cargo/registry/cache/`
           - `~/.cargo/git/db/`
           - `target/`
        4. Set cache key using `${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}`
    - **Done‑when:**
        1. Cache steps are added to all CI jobs.
        2. Subsequent CI runs show reduced execution time.
    - **Verification:**
        1. Observe cache hits in GitHub Actions logs.
        2. Verify build times decrease in subsequent runs.
    - **Depends‑on:** [T002]

## Documentation
- [x] **T004 · Bugfix · P1: Fix Rust variable naming convention in CLAUDE.md**
    - **Context:** REMEDIATION_PLAN.md - High Priority 3: CLAUDE.md inaccuracies
    - **Action:**
        1. Edit `CLAUDE.md`.
        2. Change `- Naming: Follow Rust standard camelCase for variables, PascalCase for types`
        3. To `- Naming: Follow Rust standard snake_case for variables, function names, and module names; PascalCase for types (structs, enums, traits); SCREAMING_SNAKE_CASE for constants and statics.`
    - **Done‑when:**
        1. `CLAUDE.md` correctly describes Rust naming conventions.
    - **Verification:**
        1. Review updated document for correctness.
    - **Depends‑on:** none

- [x] **T005 · Bugfix · P1: Fix Rust error handling guidance in CLAUDE.md**
    - **Context:** REMEDIATION_PLAN.md - High Priority 3: CLAUDE.md inaccuracies
    - **Action:**
        1. Edit `CLAUDE.md`.
        2. Change `- Error handling: Use std::io::Error with appropriate ErrorKind`
        3. To `- Error handling: Use Result<T, E> for recoverable errors. Define custom error types (e.g., enums or structs implementing std::error::Error), potentially using crates like thiserror for libraries or anyhow for application-level error handling. Propagate errors using the ? operator.`
    - **Done‑when:**
        1. `CLAUDE.md` reflects idiomatic Rust error handling practices.
    - **Verification:**
        1. Review updated document for correctness.
    - **Depends‑on:** none

- [ ] **T006 · Bugfix · P1: Fix Rust version in CLAUDE.md**
    - **Context:** REMEDIATION_PLAN.md - High Priority 3: CLAUDE.md inaccuracies
    - **Action:**
        1. Check the MSRV in root `Cargo.toml` and `README.md`.
        2. Edit `CLAUDE.md` to update the line `- Use Rust Edition 2021 (minimum Rust 1.65)`.
        3. Replace with correct MSRV, e.g. `- Use Rust Edition 2021 (MSRV: 1.70.0 - ensure consistency with the project's root Cargo.toml).`
    - **Done‑when:**
        1. `CLAUDE.md` states correct and consistent Rust version/MSRV.
    - **Verification:**
        1. Confirm alignment with README.md and root Cargo.toml.
    - **Depends‑on:** none

- [ ] **T007 · Chore · P3: Add rationale to branch protection documentation**
    - **Context:** REMEDIATION_PLAN.md - Low Priority 8: BRANCH_PROTECTION.md
    - **Action:**
        1. Edit `.github/docs/BRANCH_PROTECTION.md`.
        2. Add a paragraph connecting the branch protection rules to the project's code quality philosophy.
    - **Done‑when:**
        1. Rationale is added to `BRANCH_PROTECTION.md`.
    - **Verification:**
        1. Review updated document for clarity and context.
    - **Depends‑on:** none

- [ ] **T008 · Chore · P3: Add doc tests to CONTRIBUTING.md**
    - **Context:** REMEDIATION_PLAN.md - Low Priority 9: CONTRIBUTING.md
    - **Action:**
        1. Edit `CONTRIBUTING.md`.
        2. Add "Doc tests (`/// # Examples` in Rust code, run via `cargo test`)" to the test types section.
    - **Done‑when:**
        1. Doc tests are mentioned in `CONTRIBUTING.md`.
    - **Verification:**
        1. Review updated document.
    - **Depends‑on:** none

- [ ] **T009 · Chore · P3: Add conventional commits note to CONTRIBUTING.md**
    - **Context:** REMEDIATION_PLAN.md - Low Priority 9: CONTRIBUTING.md
    - **Action:**
        1. Edit `CONTRIBUTING.md`.
        2. Add note about adhering to Conventional Commits and potential future tooling enforcement.
    - **Done‑when:**
        1. Conventional Commits information is added to `CONTRIBUTING.md`.
    - **Verification:**
        1. Review updated document.
    - **Depends‑on:** none

- [ ] **T010 · Chore · P3: Mention CI enforcement in README.md**
    - **Context:** REMEDIATION_PLAN.md - Low Priority 10: README.md
    - **Action:**
        1. Edit `README.md`.
        2. In prerequisites section, add note that checks are enforced by CI.
    - **Done‑when:**
        1. CI enforcement clarification is added to `README.md`.
    - **Verification:**
        1. Review updated document.
    - **Depends‑on:** none

## Web Application Refactoring
- [ ] **T011 · Feature · P1: Implement line parsing in Rust/Wasm**
    - **Context:** REMEDIATION_PLAN.md - High Priority 4: Move parsing to zebra_wasm
    - **Action:**
        1. Edit `zebra_wasm/src/lib.rs` (or create a new module).
        2. Define data structure (e.g., `ParsedLineInfo`) with Serialize and wasm_bindgen attributes.
        3. Implement parsing function with proper error handling.
        4. Export function to JavaScript via wasm_bindgen.
        5. Add unit tests for the Rust parsing function.
    - **Done‑when:**
        1. Rust parsing function is implemented with proper error handling.
        2. Function is exported via wasm_bindgen.
        3. Unit tests exist and pass.
        4. `wasm-pack build` succeeds.
    - **Verification:**
        1. Run Rust unit tests with various inputs.
        2. Build Wasm package successfully.
    - **Depends‑on:** none

- [ ] **T012 · Refactor · P1: Update webapp to use Wasm parser**
    - **Context:** REMEDIATION_PLAN.md - High Priority 4: Move parsing to zebra_wasm
    - **Action:**
        1. Edit `zebra_webapp/index.html` (or relevant JavaScript file).
        2. Remove existing `parseString` JavaScript function.
        3. Update code to call the new Wasm function.
        4. Add proper error handling for the Result returned by Wasm.
    - **Done‑when:**
        1. JavaScript code calls Wasm function instead of original parsing function.
        2. Error handling is implemented.
        3. Web app still functions correctly with the refactored code.
    - **Verification:**
        1. Build and run web application.
        2. Test with various inputs (valid and invalid) to ensure proper behavior.
    - **Depends‑on:** [T011]

## Development Environment
- [ ] **T013 · Chore · P2: Update pre-commit hooks for Rust**
    - **Context:** REMEDIATION_PLAN.md - Medium Priority 6: Pre-commit hooks
    - **Action:**
        1. Edit `.pre-commit-config.yaml`.
        2. Remove old `doublify/pre-commit-rust` hook.
        3. Add modern hooks for rustfmt and clippy, using either:
           - Local hooks with `language: system`
           - Or a maintained mirror repo
        4. Run `pre-commit install --install-hooks`.
    - **Done‑when:**
        1. Old hook is removed.
        2. New hooks are added and properly configured.
        3. Hooks run successfully on commit.
    - **Verification:**
        1. Make a formatting error and verify commit is blocked.
        2. Make a linting error and verify commit is blocked.
        3. Fix errors and verify commit succeeds.
    - **Depends‑on:** none

- [ ] **T014 · Chore · P2: Remove redundant Cargo.lock entry from .gitignore**
    - **Context:** REMEDIATION_PLAN.md - Medium Priority 7: .gitignore redundancy
    - **Action:**
        1. Edit `.gitignore`.
        2. Remove the line `!Cargo.lock`.
    - **Done‑when:**
        1. Redundant line is removed.
        2. `Cargo.lock` is still tracked by git.
    - **Verification:**
        1. Run `git status` to confirm `Cargo.lock` is still tracked.
    - **Depends‑on:** none

## Future Work
- [ ] **T015 · Research · P3: Research conventional commits enforcement tooling**
    - **Context:** REMEDIATION_PLAN.md - Low Priority 11: Conventional Commits
    - **Action:**
        1. Research tools for enforcing Conventional Commits format.
        2. Document options (e.g., commitlint, husky, pre-commit hooks).
        3. Create proposal for future implementation.
    - **Done‑when:**
        1. Research findings and recommendations are documented.
    - **Verification:**
        1. Review research document.
    - **Depends‑on:** none

### Clarifications & Assumptions
- [ ] **Issue:** Confirm the exact MSRV (Minimum Supported Rust Version) for the project.
    - **Context:** Needed for T006
    - **Blocking?:** no
- [ ] **Issue:** Should we consider forking `dioxus-free-icons` under the project's organization if the SimonBaars fork is only needed temporarily?
    - **Context:** Alternative approach for T001
    - **Blocking?:** no
