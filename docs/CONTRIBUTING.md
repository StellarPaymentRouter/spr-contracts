# Contributing to SPR Contracts

Thank you for contributing to SPR Contracts, the Soroban smart contract layer for Stellar Payment Router. The project welcomes feature work, bug fixes, tests, documentation, security improvements, and performance optimizations that help make payment routing on Stellar reliable and open.

## Table of Contents

- [Ways to Contribute](#ways-to-contribute)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Building and Testing](#building-and-testing)
- [Code Standards](#code-standards)
- [Security Guidelines](#security-guidelines)
- [Pull Request Process](#pull-request-process)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Documentation Standards](#documentation-standards)
- [Getting Help](#getting-help)
- [License](#license)

## Ways to Contribute

SPR Contracts needs contributions across several areas:

- Routing algorithms, path scoring, and optimization techniques
- Liquidity discovery, reserve validation, and AMM calculations
- Smart contract execution safety and authorization rules
- Unit tests, integration tests, property tests, and fuzzing
- Documentation, examples, diagrams, and developer guides
- Gas and WASM size optimization
- Security review, invariant testing, and vulnerability fixes

## Development Setup

### Prerequisites

- Rust 1.70 or newer
- Soroban CLI
- Git
- A code editor with Rust Analyzer support

### Install Rust and WASM Target

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
```

### Install Soroban CLI

```bash
cargo install --locked soroban-cli
soroban --version
```

### Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/spr-contracts.git
cd spr-contracts
git remote add upstream https://github.com/StellarPaymentRouter/spr-contracts.git
```

### Verify Setup

```bash
cargo build --target wasm32-unknown-unknown --release
cargo test
make all
```

## Development Workflow

Create a focused feature branch from `main`:

```bash
git checkout main
git pull upstream main
git checkout -b feat/your-feature-name
```

Keep each change scoped to a clear goal. For contract behavior changes, update tests and documentation in the same pull request.

Recommended workflow:

1. Read the relevant module and docs before editing.
2. Add or update tests that describe the desired behavior.
3. Implement the contract logic.
4. Run formatting, linting, tests, and release build.
5. Update documentation when public behavior, configuration, or architecture changes.

## Building and Testing

### Build

```bash
make build
```

This creates the optimized WASM contract at:

```text
target/wasm32-unknown-unknown/release/spr_contracts.wasm
```

For a debug build:

```bash
cargo build --target wasm32-unknown-unknown
```

### Test

```bash
make test
make test-verbose
cargo test --test integration_test
cargo test route_finding
```

Coverage can be generated with Tarpaulin when installed:

```bash
cargo tarpaulin --out Html
```

### Quality Checks

```bash
make fmt
cargo fmt -- --check
make lint
make all
```

## Code Standards

### Rust Practices

- Follow idiomatic Rust naming and module conventions.
- Return `Result<T, ContractError>` for recoverable contract failures.
- Avoid `unwrap()` and `expect()` in contract logic.
- Use explicit validation before arithmetic and state changes.
- Prefer small functions with clear responsibility.
- Document public APIs with Rust doc comments.

### Module Organization

```text
src/
|-- lib.rs          # Soroban contract entry point
|-- router.rs       # Route discovery, simulation, execution, and fees
|-- path.rs         # Path construction, validation, and scoring
|-- liquidity.rs    # Pool discovery, reserve checks, and swap math
|-- types.rs        # Route, Hop, Pool, and Liquidity structures
|-- errors.rs       # ContractError codes
`-- events.rs       # Contract event emission helpers
```

### Function Documentation

Use documentation comments for public functions:

```rust
/// Calculate swap output using an AMM formula.
///
/// Returns `ContractError::InvalidParams` when amounts, reserves, or fees are invalid.
pub fn calculate_swap_output(
    input_amount: i128,
    input_reserve: i128,
    output_reserve: i128,
    fee_rate: i128,
) -> Result<i128, ContractError> {
    // implementation
}
```

## Security Guidelines

- Validate every external input.
- Use checked arithmetic where overflow is possible.
- Never panic in contract execution paths.
- Protect admin-only operations with explicit authorization.
- Preserve route invariants through tests.
- Add tests for zero values, negative values, high fees, low liquidity, circular routes, and slippage failures.
- Consider independent review for changes touching execution, accounting, authorization, or fee logic.

See [Security](./SECURITY.md) for the reporting process and project security model.

## Pull Request Process

Before opening a pull request:

- Confirm the issue or feature request is understood.
- Keep the branch focused.
- Run `make all`.
- Add tests for new or changed behavior.
- Update docs for public APIs, deployment steps, security notes, or architecture changes.
- Ensure no secrets, private keys, or generated artifacts are committed.

Open the PR with:

- A clear title
- A concise description of what changed
- Linked issues
- Notes about tests run
- Any migration, deployment, or security considerations

Review flow:

1. CI runs formatting, linting, tests, and build checks.
2. Maintainers review behavior, security, and maintainability.
3. Contributors address requested changes.
4. Approved changes are merged.

## Commit Message Guidelines

Use this format:

```text
type: subject

Optional body explaining why the change was made.
```

Common types:

- `feat` - New feature
- `fix` - Bug fix
- `test` - Test additions or corrections
- `docs` - Documentation changes
- `refactor` - Internal restructuring
- `perf` - Performance improvement
- `style` - Formatting-only change
- `chore` - Tooling or maintenance

Examples:

```text
feat: add Dijkstra path scoring
```

```text
fix: reject zero reserves in swap calculation
```

```text
test: cover insufficient liquidity route failures
```

## Documentation Standards

- Keep docs synchronized with contract behavior.
- Use relative links between markdown files.
- Include examples for public APIs and deployment steps.
- Document assumptions, invariants, and trade-offs.
- Update [Architecture](./ARCHITECTURE.md) when module responsibilities change.
- Update [Routing Algorithm](./ROUTING_ALGORITHM.md) when pathfinding or scoring changes.
- Update [Deployment](./DEPLOYMENT.md) when build, network, or environment steps change.

## Getting Help

Use GitHub Issues for bugs and feature requests. Use GitHub Discussions for design questions, implementation proposals, and support.

When asking for help, include:

- Rust and Soroban CLI versions
- Steps to reproduce
- Error output
- Expected behavior
- Minimal test case or example command

## Recognition

Significant contributions should be recognized in release notes and project documentation.

## License

MIT License

Copyright (c) 2026 Stellar Payment Router Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
