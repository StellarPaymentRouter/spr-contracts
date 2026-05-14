# Deployment Guide

This guide covers local build verification, Soroban testnet deployment, production preparation, monitoring, and operational practices for SPR Contracts.

## Prerequisites

Install the required tooling:

- Rust 1.70 or newer
- `wasm32-unknown-unknown` Rust target
- Soroban CLI
- Git
- A funded Stellar account for the target network

Install Rust and the WASM target:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
```

Install Soroban CLI:

```bash
cargo install --locked soroban-cli
soroban --version
```

Clone the repository:

```bash
git clone https://github.com/StellarPaymentRouter/spr-contracts.git
cd spr-contracts
```

## Environment Configuration

The repository includes `.env.example` with the expected configuration shape:

```text
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_NETWORK=testnet
NETWORK_PASSPHRASE=Test SDF Network ; September 2015
ADMIN_KEY=<your-admin-public-key>
BASE_FEE=10
```

Create a local `.env` from the example when needed:

```bash
cp .env.example .env
```

Do not commit `.env`, private keys, seed phrases, or production credentials.

## Build

Build the optimized WASM contract:

```bash
make build
```

Equivalent Cargo command:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Expected artifact:

```text
target/wasm32-unknown-unknown/release/spr_contracts.wasm
```

## Test and Quality Checks

Run tests:

```bash
make test
```

Run tests with output:

```bash
make test-verbose
```

Run formatting, linting, tests, and build:

```bash
make all
```

Before any production deployment, `make all` should pass from a clean checkout.

## Deploy to Testnet

Deploy the compiled WASM contract to Stellar testnet:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/spr_contracts.wasm \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  --source <your-account>
```

Record the resulting contract ID. Integrations should reference this contract ID for route discovery, simulation, and execution.

## Configure Contract Settings

SPR Contracts uses basis points for routing fees. A value of `10` represents `0.1%`.

Fee configuration should be performed only by the configured admin account:

```text
set_fee(10)
```

Production deployments should document:

- Contract ID
- WASM hash
- Network
- Admin public key
- Base fee
- Deployment commit
- Deployment date

## Mainnet Deployment Checklist

- [ ] Code reviewed.
- [ ] `make all` passes.
- [ ] WASM artifact built from a trusted commit.
- [ ] WASM hash recorded.
- [ ] Integration tests pass.
- [ ] Testnet deployment completed.
- [ ] Route simulation tested with representative assets.
- [ ] Route execution tested with small amounts.
- [ ] Security review completed.
- [ ] Admin key secured.
- [ ] Fee configuration reviewed.
- [ ] Monitoring plan prepared.
- [ ] Rollback plan prepared.
- [ ] Deployment records stored.

## Production Setup

### Admin Key Management

The admin key controls configuration such as routing fees. Treat it as production infrastructure:

- Store it in secure key management.
- Restrict access to trusted operators.
- Avoid using development accounts.
- Rotate credentials if exposure is suspected.
- Document recovery and transfer procedures.

### RPC Configuration

Use reliable Soroban RPC infrastructure for production integrations. Monitor RPC availability, latency, and error rates.

Recommended deployment records:

```text
SOROBAN_RPC_URL=<production-rpc-url>
STELLAR_NETWORK=mainnet
NETWORK_PASSPHRASE=Public Global Stellar Network ; September 2015
CONTRACT_ID=<deployed-contract-id>
ADMIN_KEY=<admin-public-key>
BASE_FEE=<fee-in-basis-points>
```

### Contract Invocation

Integrations should follow this sequence:

1. Call `find_route()` to discover the best path.
2. Call `simulate_route()` to verify expected output.
3. Submit `execute_route()` with acceptable slippage controls.
4. Monitor emitted events for execution and fees.

## Monitoring and Logging

Monitor:

- Route discovery volume
- Route execution volume
- Failed route simulations
- Failed executions
- Slippage failures
- Fee changes
- Contract invocation errors
- RPC latency and failures

Events should be indexed by:

- Source asset
- Destination asset
- Amount
- Route hops
- Fee amount
- Sender and receiver when applicable
- Contract ID
- Ledger timestamp

## Scaling Considerations

Routing contracts must balance route quality and execution cost.

Operational recommendations:

- Limit maximum hop count.
- Avoid unnecessary state reads.
- Cache off-chain liquidity metadata when appropriate.
- Keep route simulation cheap enough for common integrations.
- Use efficient data structures for path search.
- Benchmark route discovery as liquidity sources grow.

## Backup and Recovery

Maintain records for:

- Deployment commit
- WASM artifact hash
- Contract ID
- Admin public key
- Fee configuration
- Deployment commands
- Network passphrase
- Operational runbooks

Recovery planning should include:

- Re-deploying the last known good WASM artifact.
- Updating integrations to a replacement contract ID.
- Restoring configuration values.
- Communicating changes to integrators.

## Troubleshooting

### WASM target is missing

```text
error: can't find crate for `core`
```

Fix:

```bash
rustup target add wasm32-unknown-unknown
```

### Soroban CLI is missing

```text
soroban: command not found
```

Fix:

```bash
cargo install --locked soroban-cli
```

### Contract build fails

Run:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --target wasm32-unknown-unknown --release
```

Fix compiler, lint, or test failures before deploying.

### Deployment fails

Check:

- The WASM artifact exists.
- The source account is funded.
- The RPC URL is reachable.
- The network passphrase matches the target network.
- The Soroban CLI version is compatible with the SDK.

### Route execution fails

Possible causes:

- Invalid source or destination asset.
- Amount is zero or negative.
- No route exists.
- Liquidity changed after simulation.
- Slippage exceeded the allowed minimum.
- Fee configuration is invalid.
- Admin-only function was called by a non-admin account.

See [Security](./SECURITY.md) for production safety guidance and [Architecture](./ARCHITECTURE.md) for the contract flow.
