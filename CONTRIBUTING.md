# Contributing to SPR Contracts

Thank you for your interest in contributing to the Stellar Payment Router smart contracts. We welcome contributions of all kinds — new features, bug fixes, tests, documentation, and optimizations.

## Table of Contents

- [Ways to Contribute](#ways-to-contribute)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Building and Testing](#building-and-testing)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Code Standards](#code-standards)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Getting Help](#getting-help)

## Ways to Contribute

We welcome:

- **New Features** — Additional routing algorithms, optimization techniques
- **Bug Fixes** — Issues with existing contract logic
- **Tests** — Unit tests, integration tests, property-based tests
- **Documentation** — Architecture docs, algorithm explanations, examples
- **Performance Improvements** — Gas optimization, calculation efficiency
- **Security Improvements** — Better validation, error handling
- **Fuzzing & Testing** — Edge case discovery

## Development Setup

### Prerequisites

- Rust 1.70+
- Soroban CLI
- Git
- Code editor (VS Code recommended with Rust Analyzer)

### Install Rust

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
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/spr-contracts.git
cd spr-contracts

# Add upstream remote
git remote add upstream https://github.com/StellarPaymentRouter/spr-contracts.git
```

### Verify Setup

```bash
cargo build --target wasm32-unknown-unknown --release
cargo test
make all
```

## Development Workflow

### Create a Feature Branch

```bash
git checkout main
git pull upstream main
git checkout -b feat/your-feature-name
```

### Make Your Changes

Implement feature in src/
Add tests in tests/
Update documentation if needed
Follow code standards (see below)

### Build and Test Locally

```bash
# Build WASM contracts
make build

# Run all tests
make test

# Run specific test with output
cargo test test_name -- --nocapture --test-threads=1

# Format code
make fmt

# Run linter
make lint

# Full quality check
make all
```

## Building and Testing

### Build

```bash
# Release build (optimized)
make build

# Debug build
cargo build --target wasm32-unknown-unknown
```

WASM files are in target/wasm32-unknown-unknown/release/

### Testing

```bash
# Run all tests
make test

# Run tests with output
make test-verbose

# Run specific test file
cargo test --test integration_test

# Run tests matching pattern
cargo test route_finding

# Generate coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

### Quality Checks

```bash
# Format code
make fmt

# Check formatting
cargo fmt -- --check

# Run clippy linter
make lint

# Full check
make all
```

## Submitting a Pull Request

### Before You Start

Check existing issues and PRs
Fork the repository
Create a feature branch
Make your changes
Ensure all tests pass

### Creating the PR

Push to your fork:

```bash
git push origin feat/your-feature-name
```

Open PR on GitHub with:

Clear title
Detailed description
Related issue links
Fill in PR template

### PR Checklist

Code builds without errors (make build)
All tests pass (make test)
Code formatted (make fmt)
No lint issues (make lint)
Documentation updated
New tests added for features
No breaking changes

### PR Review

Automated Checks — CI runs all checks
Code Review — Maintainers review
Feedback — Address requested changes
Merge — Approved PR is merged

## Code Standards

### Rust Best Practices

Follow Rust naming conventions
Use Result for error handling
Avoid unwrap() in library code
Use ? operator for error propagation
Write idiomatic Rust

### Module Structure

```text
src/
├── lib.rs          # Main contract
├── router.rs       # Core routing
├── path.rs         # Path finding
├── liquidity.rs    # Liquidity management
├── types.rs        # Data structures
├── errors.rs       # Error types
└── events.rs       # Event emissions

tests/
└── integration_test.rs

docs/
├── ARCHITECTURE.md
└── ROUTING_ALGORITHM.md
```

### Functions

Single Responsibility Principle
Clear, descriptive names
Proper error handling
Documentation comments

### Example Function

```rust
/// Calculate swap output using AMM formula
///
/// # Arguments
/// * `input_amount` - Amount to swap in
/// * `input_reserve` - Reserve of input asset
/// * `output_reserve` - Reserve of output asset
/// * `fee_rate` - Trading fee (basis points)
///
/// # Returns
/// Output amount after fee
///
/// # Errors
/// Returns `InvalidParams` if reserves or amounts are invalid
///
/// # Formula
/// `output = (input × (1 - fee) × output_reserve) / (input_reserve + input × (1 - fee))`
pub fn calculate_swap_output(
    input_amount: i128,
    input_reserve: i128,
    output_reserve: i128,
    fee_rate: i128,
) -> Result<i128, ContractError> {
    if input_amount <= 0 || input_reserve <= 0 || output_reserve <= 0 {
        return Err(ContractError::InvalidParams);
    }

    let input_with_fee = input_amount * (10000 - fee_rate) / 10000;
    let numerator = input_with_fee * output_reserve;
    let denominator = input_reserve + input_with_fee;

    Ok(numerator / denominator)
}
```

## Tests

Test file names: \*\_test.rs or in tests/ directory
Describe test purpose in function name
Test happy path, edge cases, errors
Use assertions and expect messages

### Example Test

```rust
#[test]
fn test_calculate_swap_output_valid_input() {
    // Arrange
    let input_amount = 1000;
    let input_reserve = 1000000;
    let output_reserve = 500000;
    let fee_rate = 25; // 0.25%

    // Act
    let result = calculate_swap_output(
        input_amount,
        input_reserve,
        output_reserve,
        fee_rate,
    );

    // Assert
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output > 0);
    assert!(output < input_amount); // Output less than input due to fee
}

#[test]
fn test_calculate_swap_output_invalid_amount() {
    // Should reject zero or negative amounts
    let result = calculate_swap_output(0, 1000, 1000, 25);
    assert!(result.is_err());
}
```

## Documentation

Doc comments for all public functions
Include purpose, parameters, returns, errors
Add examples where helpful

### Comments

```rust
// Use for implementation notes and explanations

/// Use for documentation comments
/// They appear in generated docs
```

## Commit Message Guidelines

### Format

```text
type: subject (max 72 characters)

Optional body (max 100 characters per line)

Optional footer
```

### Types

feat — New feature
fix — Bug fix
test — Test additions
docs — Documentation
refactor — Code refactoring
perf — Performance improvement
style — Code formatting
chore — Build/tooling

### Examples

```text
feat: implement advanced pathfinding algorithm

Add Dijkstra-based pathfinding for multi-hop routes.
Includes pool discovery and efficiency ranking.
Improves route quality by 15% in benchmarks.
```

```text
fix: handle division by zero in swap calculation

The AMM formula could fail with zero reserves.
Added validation to check reserves > 0.

Fixes #45
```

```text
test: add comprehensive test coverage for liquidity

Added 20+ tests for liquidity calculations:
- Valid calculations
- Edge cases (zero amounts, high fees)
- Error cases

Coverage: 95% → 99%
```

## Performance Considerations

### Gas Optimization

Minimize state reads/writes
Use efficient algorithms
Avoid loops when possible
Batch operations

### Memory

Pre-allocate where possible
Clean up temporary data
Use stack over heap

### Benchmarking

```bash
# Add benchmarks with bencher
cargo bench
```

## Security Guidelines

Input Validation — Always validate inputs
Overflow Protection — Use checked\_\* operations
Error Handling — Never panic in contracts
Invariants — Document and test invariants
Audit — Consider security audit for critical code

## Getting Help

GitHub Issues — Bug reports and features
GitHub Discussions — Questions and ideas
PR Comments — Implementation questions

### Asking Questions

Be specific and provide context
Include error messages
Share minimal reproducible example
Describe what you've tried

### Reporting Bugs

Include Rust version (rustc --version)
Describe steps to reproduce
Share error output
Provide minimal test case

## Testing Standards

### Unit Tests

Test individual functions
Cover happy path and errors
Use clear test names

### Integration Tests

Test component interactions
Test end-to-end flows
Verify contract behavior

### Property Tests

Mathematical properties
Invariant testing
Random input generation

## Documentation Standards

Keep docs up-to-date with code
Include examples
Document public APIs thoroughly
Add inline comments for complex logic

## Recognition

Contributors recognized in:

README.md — Contributors section
Release notes — For significant contributions

## Questions?

Open an issue or discussion! We're here to help.

MIT License

Copyright (c) 2026 Stellar Payment Router Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
