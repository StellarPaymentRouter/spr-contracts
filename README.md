# SPR Contracts

[![Soroban SDK](https://img.shields.io/badge/Soroban-SDK%2020-blue.svg)](https://github.com/stellar/rs-soroban-sdk)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

SPR Contracts is the Soroban smart contract layer for Stellar Payment Router, providing on-chain payment routing, liquidity-aware path selection, multi-hop execution, and configurable fee collection for Stellar assets.

The project is designed as open-source routing infrastructure for applications that need reliable asset-to-asset payments across Stellar liquidity sources.

## Features

- On-chain route discovery between Stellar assets
- Liquidity-aware multi-hop path construction
- Route simulation before execution
- Atomic route execution through Soroban contracts
- Configurable routing fees in basis points
- Structured contract errors for predictable integrations
- On-chain events for route discovery, execution, and fees
- WASM-optimized Rust implementation for Soroban

## Quick Start

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli
git clone https://github.com/StellarPaymentRouter/spr-contracts.git
cd spr-contracts
make build
make test
```

The optimized contract artifact is generated at:

```text
target/wasm32-unknown-unknown/release/spr_contracts.wasm
```

For environment setup, deployment commands, and production readiness steps, see [Deployment](./docs/DEPLOYMENT.md).

## Documentation

- [Architecture](./docs/ARCHITECTURE.md) - Contract design, modules, data flow, and trade-offs
- [Routing Algorithm](./docs/ROUTING_ALGORITHM.md) - Pathfinding phases, scoring, validation, and edge cases
- [Contributing](./docs/CONTRIBUTING.md) - Development setup, standards, testing, and PR workflow
- [Deployment](./docs/DEPLOYMENT.md) - Build, testnet deployment, mainnet checklist, and operations
- [Security](./docs/SECURITY.md) - Reporting process, contract security model, and user best practices

The original root [CONTRIBUTING.md](./CONTRIBUTING.md) is preserved for compatibility.

## Support

Use GitHub Issues for bugs and feature requests. Use GitHub Discussions for design questions, implementation ideas, and community support.

## License

SPR Contracts is released under the MIT License. See [Contributing](./docs/CONTRIBUTING.md#license) for the current license text.
