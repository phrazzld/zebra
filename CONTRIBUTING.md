# Contributing to ZebraSign

Thank you for your interest in contributing to ZebraSign! This document outlines the code quality standards and procedures that all contributors are expected to follow. Maintaining high code quality is a core value of this project, as it ensures reliability, security, and maintainability of our cryptographic software.

## Code Quality Philosophy

ZebraSign prioritizes code that is:
- **Correct:** Functional, secure, and free of bugs
- **Readable:** Easy to understand by other developers
- **Maintainable:** Well-structured and documented
- **Testable:** Designed with testing in mind

We enforce these values through automated tools, strict guidelines, and thorough code reviews.

## Development Prerequisites

Before contributing, make sure you have set up your development environment properly:
- See the [README.md](README.md#development-prerequisites) for instructions on installing Rust, pre-commit hooks, and other prerequisites.

## Code Style

### Rust Formatting (rustfmt)

- **Mandatory Usage:** Code formatting using `rustfmt` is **REQUIRED** for all Rust code.
- **Configuration:** We use a custom `.rustfmt.toml` configuration with the following key settings:
  ```toml
  edition = "2021"
  max_width = 100
  tab_spaces = 4
  hard_tabs = false
  reorder_imports = true
  use_field_init_shorthand = true
  newline_style = "Unix"
  ```
- **Running Locally:** Format your code with:
  ```bash
  cargo fmt --all
  ```
- **Verification:** Check your code formatting without modifying files:
  ```bash
  cargo fmt --all --check
  ```
- **Enforcement:** Formatting is automatically checked by:
  - Pre-commit hooks (blocking non-compliant commits)
  - CI pipeline (failing PR builds with formatting issues)

## Linting

### Rust Linting (clippy)

- **Mandatory Usage:** Static analysis using `clippy` is **REQUIRED** for all Rust code.
- **Standard Command:** The project enforces a strict set of lints:
  ```bash
  cargo clippy --all-targets --all-features --workspace -- -D warnings
  ```
- **Strictness Level:** We treat warnings as errors (`-D warnings`) to maintain high code quality.
- **Policy on Lint Suppressions:**
  - `#[allow(...)]` suppressions are **STRONGLY DISCOURAGED**
  - Suppressions are only permitted in exceptional circumstances with thorough justification
  - Any suppressions must be documented in `clippy.toml` with a detailed explanation
  - All suppressions must be reviewed and approved during code review

### Current Allowed Suppressions

Currently, there is only one allowed suppression in the codebase:

- **clippy::impl_hash_borrow_with_str_and_bytes:**
  - Allowed in: `boringascii/src/lib.rs`
  - Justification: The `BoringAscii` type needs to be usable in hash-based collections while also supporting borrowing as both `&str` and `&[u8]` for API convenience. This is safe because `BoringAscii`'s constructor ensures that only valid ASCII is stored, making the `&str` and `&[u8]` views semantically equivalent for hashing purposes. The type is carefully constructed to ensure this invariant is maintained.

## Testing Standards

- **Coverage Expectations:** All new code should be thoroughly tested.
- **Test Types:**
  - **Unit Tests:** Functions and modules in isolation
  - **Integration Tests:** Components working together
  - **Property Tests:** For validating cryptographic code properties
  - **Doc Tests:** Examples in documentation that double as tests (`/// # Examples` in Rust code, run via `cargo test`)
- **Naming Conventions:**
  - Test modules should use the `#[cfg(test)]` attribute
  - Test functions should be descriptive and follow `snake_case` naming
  - Use the pattern `test_<functionality>_<scenario>` for clarity
- **Best Practices:**
  - Test both happy paths and edge cases
  - Mock only external dependencies, never internal collaborators
  - Cryptographic code should include verification against known test vectors
  - Include runnable examples in documentation for public APIs

## Enforcement Mechanisms

### Pre-commit Hooks

ZebraSign uses the `pre-commit` framework to enforce code quality standards at commit time.

- **Setup:** Run `pre-commit install` after cloning the repository
- **Configuration:** See `.pre-commit-config.yaml` in the repository root
- **Enforced Checks:**
  - **File Formatting:**
    - Trailing whitespace removal
    - End-of-file newline enforcement
    - YAML and TOML syntax validation
    - Merge conflict detection
  - **Rust Checks:**
    - rustfmt code formatting
    - clippy linting with strict settings

### CI Pipeline

The project uses GitHub Actions to enforce code quality on all PRs and pushes to main.

- **Configuration:** See `.github/workflows/rust_quality.yml`
- **Jobs:**
  - **format:** Checks Rust code formatting using rustfmt
  - **lint:** Runs clippy with strict settings
- **Branch Protection:**
  - The `main` branch requires these status checks to pass before merging
  - Pull requests must resolve all formatting and linting issues before they can be merged

## Pull Request Guidelines

### PR Process

1. Create a feature branch from `main`
2. Implement your changes following the code quality standards
3. Run local checks (formatting, linting, tests)
4. Submit a pull request to `main`
5. Address any feedback from code reviews
6. Ensure all CI checks pass

### PR Requirements

- **Code Quality:** All code must pass formatting and linting checks
- **Tests:** New features should include tests
- **Documentation:** Update documentation as needed
- **Commit Messages:** Follow [Conventional Commits](https://www.conventionalcommits.org/) style
  - Conventional Commits enable automated versioning and changelog generation
  - The project may implement tooling in the future to enforce this standard
  - Contributors are encouraged to use tools like commitlint, git hooks, or IDE plugins to help follow the standard

## Common Issues and Solutions

### Formatting Issues

- **Problem:** rustfmt fails with errors
- **Solution:** Run `cargo fmt --all` to automatically fix formatting issues

### Linting Issues

- **Problem:** clippy reports warnings or errors
- **Solution:**
  - Fix the issues according to clippy's suggestions
  - If you believe a lint should be suppressed, discuss it in the PR comments with detailed justification

### Pre-commit Hook Issues

- **Problem:** Hooks fail during commit
- **Solution:**
  - Fix the reported issues
  - Run `pre-commit run --all-files` to check all files before attempting to commit
  - NEVER use `git commit --no-verify` to bypass hooks

## Questions and Support

If you have questions about contributing or need help with code quality standards, please:
- Open an issue with the label "question"
- Ask in the project's communication channels
