# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands
- Build: `cargo build --release`
- Run desktop app: `cargo run --release`
- Test: `cargo test`
- Test specific crate: `cargo test -p zebra_crypto`
- Test specific function: `cargo test -p zebra_crypto -- tests::basic_signatures_work`
- Build WebAssembly: `cd zebra_wasm && wasm-pack build`
- Build & run webapp: `cd zebra_webapp && npm install && npm run build && npm run start`

## Code Style Guidelines
- Use Rust Edition 2021 (minimum Rust 1.65)
- Follow default Rust formatting and idioms
- Security: Use `zeroize` and `ZeroizeOnDrop` for sensitive data
- Error handling: Use `Result<T, E>` for recoverable errors. Define custom error types (e.g., enums or structs implementing `std::error::Error`), potentially using crates like `thiserror` for libraries or `anyhow` for application-level error handling. Propagate errors using the `?` operator
- Naming: Follow Rust standard snake_case for variables, function names, and module names; PascalCase for types (structs, enums, traits); SCREAMING_SNAKE_CASE for constants and statics
- Imports: Organize imports by standard library, then external crates, then local modules
- Documentation: Thoroughly document public interfaces with /// comments
- Memory safety: Minimize time private keys are in memory
- Types: Use borsh serialization with explicit tags for forward compatibility
- Testing: Write comprehensive tests for cryptographic code

## Architecture
- Crypto: Implements ring signatures using Ristretto group (based on Curve25519)
- Storage: Uses age encryption with operating system keychain for DB protection
- UI: Built with Dioxus (Rust web framework)
