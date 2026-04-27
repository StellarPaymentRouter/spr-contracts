# SPR Contracts

[![Soroban SDK](https://img.shields.io/badge/Soroban-SDK%2020-blue.svg)](https://github.com/stellar/rs-soroban-sdk)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Soroban Smart Contracts for Stellar Payment Router**

SPR Contracts provides on-chain routing logic, liquidity management, and multi-hop transaction execution for optimal payments on the Stellar network.

---

## The Problem

Stellar developers need:

- **On-Chain Routing** — Smart contract-based route optimization
- **Liquidity Discovery** — Automated pool discovery and aggregation
- **Multi-Hop Execution** — Atomic multi-step transactions
- **Fee Management** — Configurable routing fees
- **Security** — Audited contract logic with proper validation
- **Efficiency** — Optimized for Soroban WASM constraints

## The Solution

**SPR Contracts** provides:

- **Router Contract** — Core routing engine
- **Liquidity Manager** — Pool discovery and aggregation
- **Path Finder** — Multi-hop pathfinding algorithms
- **Event System** — On-chain event logging
- **Error Handling** — Specific error types with recovery paths
- **Fee Collection** — Built-in fee mechanisms

---

## Architecture

```
┌──────────────────────────────────────┐
│  Stellar Network                     │
├──────────────────────────────────────┤
│  Soroban Runtime (WASM)              │
├──────────────────────────────────────┤
│  SPR Router Contract                 │
│  - find_route()                      │
│  - simulate_route()                  │
│  - execute_route()                   │
│  - get_fee() / set_fee()             │
├──────────────────────────────────────┤
│  Core Modules                        │
│  ├─ router.rs      (Route engine)    │
│  ├─ path.rs        (Pathfinding)     │
│  ├─ liquidity.rs   (Pool management) │
│  ├─ types.rs       (Data structures) │
│  ├─ errors.rs      (Error handling)  │
│  └─ events.rs      (Event logging)   │
├──────────────────────────────────────┤
│  Stellar Layer                       │
│  - Horizon API                       │
│  - Soroban RPC                       │
└──────────────────────────────────────┘
```

### Technology Stack

- **Language**: Rust 1.70+
- **Framework**: Soroban SDK 20
- **Compilation**: WASM (WebAssembly)
- **Network**: Stellar Testnet / Mainnet

---

## Features

### Route Finding

- Discover optimal paths between assets
- Multi-hop pathfinding
- Efficiency scoring and ranking
- Liquidity availability checking
- Slippage tolerance enforcement

### Liquidity Management

- Automated pool discovery
- Real-time reserve tracking
- Fee-adjusted calculations
- Multi-DEX aggregation
- Liquidity sufficiency validation

### Multi-Hop Execution

- Atomic transaction execution
- Intermediate amount calculation
- Automatic fee collection
- Event emission for tracking
- Transaction rollback on failure

### Fee System

- Configurable fee rates
- Per-hop fee collection
- Admin-only fee management
- Fee event tracking
- Basis points configuration

### Security

- Input validation
- Overflow protection
- Reserve sufficiency checks
- Reentrancy prevention
- Admin authorization

### Event Tracking

- Route discovery events
- Transaction execution events
- Fee collection events
- On-chain logging

---

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Soroban CLI
- Git

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli

# Clone repository
git clone https://github.com/StellarPaymentRouter/spr-contracts.git
cd spr-contracts
```

### Building

```bash
# Build WASM contract
make build

# Verify build
ls target/wasm32-unknown-unknown/release/spr_contracts.wasm
```

### Testing

```bash
# Run tests
make test

# Run tests with output
make test-verbose

# Run specific test
cargo test test_route_finding -- --nocapture
```

---

## Project Structure

```
spr-contracts/
├── src/
│   ├── lib.rs                  # Main contract entry
│   ├── router.rs               # Core routing logic
│   ├── path.rs                 # Pathfinding algorithms
│   ├── liquidity.rs            # Liquidity management
│   ├── types.rs                # Data structures
│   ├── errors.rs               # Error definitions
│   └── events.rs               # Event emissions
├── tests/
│   └── integration_test.rs     # Integration tests
├── docs/
│   ├── ARCHITECTURE.md
│   └── ROUTING_ALGORITHM.md
├── Cargo.toml
├── Makefile
└── README.md
```

---

## Contract API

### find_route

Discover the best route between two assets.

```rust
pub fn find_route(
    env: Env,
    source_asset: Symbol,
    dest_asset: Symbol,
    amount: i128,
) -> Result<Route, ContractError>
```

**Parameters:**

- `env` — Soroban environment
- `source_asset` — Source asset symbol
- `dest_asset` — Destination asset symbol
- `amount` — Amount to route

**Returns:**

```rust
Route {
    source_asset: Symbol,
    destination_asset: Symbol,
    amount: i128,
    hops: Vec<Hop>,
    total_fee: i128,
    min_received: i128,
}
```

**Example:**

```rust
let route = SprRouter::find_route(
    &env,
    symbol_short!("USDC"),
    symbol_short!("XLM"),
    1_000_0000000, // 1000 USDC (7 decimals)
)?;

println!("Hops: {}", route.hops.len());
println!("Fee: {}", route.total_fee);
```

### simulate_route

Simulate route execution without commitment.

```rust
pub fn simulate_route(
    env: Env,
    route: Route,
) -> Result<i128, ContractError>
```

**Returns:** Simulated output amount

### execute_route

Execute a route transaction.

```rust
pub fn execute_route(
    env: Env,
    route: Route,
    from: Symbol,
    to: Symbol,
) -> Result<Symbol, ContractError>
```

**Returns:** Transaction ID/Hash

### get_fee

Get current routing fee.

```rust
pub fn get_fee(env: Env) -> i128
```

**Returns:** Fee in basis points (e.g., 10 = 0.1%)

### set_fee

Set routing fee (admin only).

```rust
pub fn set_fee(
    env: Env,
    fee: i128,
) -> Result<(), ContractError>
```

---

## Core Modules

### Router (`src/router.rs`)

The main routing engine that orchestrates the route-finding and execution process.

**Key Methods:**

- `find_route()` — Entry point for route discovery
- `simulate_route()` — Dry-run execution
- `execute_route()` — Execute transaction
- `get_fee()` / `set_fee()` — Fee management

### Path Finder (`src/path.rs`)

Implements multi-hop pathfinding algorithms.

**Key Methods:**

- `build_path()` — Construct path between assets
- `validate_path()` — Check path validity
- `calculate_efficiency()` — Score path quality

**Algorithm:**

1. Discover pools with source and destination assets
2. Build directional graph of available swaps
3. Use BFS/Dijkstra to find optimal path
4. Rank paths by efficiency score
5. Return best path

### Liquidity Manager (`src/liquidity.rs`)

Manages pool discovery and liquidity calculations.

**Key Methods:**

- `discover_pools()` — Find available pools
- `get_liquidity()` — Query pool reserves
- `calculate_swap_output()` — AMM calculation
- `has_sufficient_liquidity()` — Validate reserves

**AMM Formula:**

```
output = (input × (1 - fee) × reserve_out) / (reserve_in + input × (1 - fee))
```

---

## Data Types

### Route

```rust
pub struct Route {
    pub source_asset: Symbol,
    pub destination_asset: Symbol,
    pub amount: i128,
    pub hops: Vec<Hop>,
    pub total_fee: i128,
    pub min_received: i128,
}
```

### Hop

```rust
pub struct Hop {
    pub source: Symbol,
    pub destination: Symbol,
    pub rate: i128,
    pub fee: i128,
}
```

### Pool

```rust
pub struct Pool {
    pub id: Symbol,
    pub asset_a: Symbol,
    pub asset_b: Symbol,
    pub reserve_a: i128,
    pub reserve_b: i128,
    pub fee_rate: i128,
}
```

---

## Error Handling

The contract defines specific error types:

```rust
pub enum ContractError {
    RouteNotFound = 1,           // No path available
    InvalidParams = 2,           // Invalid input
    InsufficientLiquidity = 3,   // Not enough reserves
    TransactionFailed = 4,       // Execution error
    Unauthorized = 5,            // Access denied
    SlippageExceeded = 6,        // Output too low
    PoolNotFound = 7,            // Pool not found
    InvalidAsset = 8,            // Asset format error
    RoutingError = 9,            // General routing error
}
```

**Example Error Handling:**

```rust
match SprRouter::find_route(&env, source, dest, amount) {
    Ok(route) => {
        println!("Route found: {:?}", route);
    }
    Err(ContractError::RouteNotFound) => {
        println!("No route available");
    }
    Err(ContractError::InvalidParams) => {
        println!("Invalid parameters");
    }
    Err(e) => {
        println!("Error: {:?}", e);
    }
}
```

---

## Testing

### Running Tests

```bash
# Run all tests
make test

# Run specific test
cargo test test_route_finding -- --nocapture

# Run with output
make test-verbose

# Generate coverage
cargo tarpaulin --out Html
```

### Test Categories

1. **Unit Tests** — Individual function testing
2. **Integration Tests** — Component interactions
3. **Property Tests** — Mathematical properties
4. **Fuzzing** — Random input generation

### Example Test

```rust
#[test]
fn test_calculate_swap_output() {
    let output = calculate_swap_output(
        1000,        // input
        1000000,     // input reserve
        500000,      // output reserve
        25,          // fee rate (0.25%)
    ).unwrap();

    assert!(output > 0);
    assert!(output < 500000);
}
```

---

## Deployment

### Compile to WASM

```bash
make build
# Output: target/wasm32-unknown-unknown/release/spr_contracts.wasm
```

### Deploy to Testnet

```bash
# Set up Soroban CLI
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/spr_contracts.wasm \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  --source <your-account>
```

### Mainnet Deployment Checklist

- [ ] All tests pass
- [ ] Code reviewed
- [ ] Security audit completed
- [ ] Gas optimization verified
- [ ] Testnet deployment successful
- [ ] Production fees configured
- [ ] Admin key secured
- [ ] Rollback plan prepared

---

## Performance Optimization

### Gas Optimization

```rust
// Inefficient - Multiple state reads
for pool in pools {
    let reserve = get_reserve(&env, &pool);
    process(&env, reserve);
}

// Efficient - Batch operations
let reserves = get_all_reserves(&env, &pools);
for (pool, reserve) in pools.iter().zip(reserves) {
    process(&env, reserve);
}
```

### Memory Management

```rust
// Pre-allocate vectors with capacity
let mut hops = Vec::with_capacity(10);
for hop_data in route_data {
    hops.push(hop_data);
}
```

---

## Security Considerations

### Input Validation

```rust
// Always validate inputs
if amount <= 0 || amount > MAX_AMOUNT {
    return Err(ContractError::InvalidParams);
}

if source_asset == dest_asset {
    return Err(ContractError::InvalidParams);
}
```

### Overflow Protection

```rust
// Use checked arithmetic
let result = input
    .checked_mul(rate)
    .ok_or(ContractError::RoutingError)?
    .checked_div(10000)
    .ok_or(ContractError::RoutingError)?;
```

### Authorization Checks

```rust
// Admin-only operations
fn set_fee(env: &Env, fee: i128) -> Result<(), ContractError> {
    let admin = env.storage().instance().get::<_, Address>(&symbol_short!("admin"))?;
    let invoker = env.invoker();

    if invoker != admin {
        return Err(ContractError::Unauthorized);
    }

    // Update fee...
    Ok(())
}
```

---

## Development Workflow

### Code Quality

```bash
# Format code
make fmt

# Check formatting
cargo fmt -- --check

# Run linter
make lint

# Full checks
make all
```

### Debugging

```bash
# Build debug version
cargo build --target wasm32-unknown-unknown

# Run with logging
RUST_LOG=debug cargo test test_name -- --nocapture
```

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Start

```bash
git clone https://github.com/YOUR_USERNAME/spr-contracts.git
cd spr-contracts
rustup target add wasm32-unknown-unknown
make all
```

---

## Resources

### Documentation

- [Soroban Documentation](https://developers.stellar.org/docs/learn/smart-contracts)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WASM Spec](https://webassembly.org/)

### Learning

- [Stellar Docs](https://developers.stellar.org/)
- [Soroban Examples](https://github.com/stellar/rs-soroban-sdk/tree/main/examples)

---

## License

MIT License © 2026 Stellar Payment Router Contributors

See [LICENSE](LICENSE) for details.

---

## Support

- [Architecture Guide](./docs/ARCHITECTURE.md)
- [Routing Algorithm](./docs/ROUTING_ALGORITHM.md)
- [GitHub Discussions](https://github.com/StellarPaymentRouter/spr-contracts/discussions)
- [Report Issues](https://github.com/StellarPaymentRouter/spr-contracts/issues)
- Email: [support@stellarpaymentrouter.dev](mailto:support@stellarpaymentrouter.dev)

---

## Roadmap

- [ ] Advanced pathfinding algorithms
- [ ] Cross-chain routing
- [ ] Dynamic fee adjustment
- [ ] Risk scoring system
- [ ] Performance benchmarks
- [ ] Formal verification
- [ ] Audit by professional firm

---
