# SPR Contracts Architecture

SPR Contracts provides the Soroban smart contract layer for Stellar Payment Router. Its purpose is to discover efficient payment paths, validate liquidity, simulate expected output, execute multi-hop routes atomically, collect routing fees, and emit events that make payment routing observable.

## System Overview

```text
+----------------------------------------------+
| Stellar Applications                         |
| Wallets, dApps, payment tools, backends      |
+----------------------+-----------------------+
                       |
+----------------------v-----------------------+
| Soroban RPC / Stellar Network                |
+----------------------+-----------------------+
                       |
+----------------------v-----------------------+
| SPR Router Contract                          |
| find_route / simulate_route / execute_route  |
| get_fee / set_fee                            |
+----------------------+-----------------------+
                       |
+----------------------v-----------------------+
| Core Modules                                  |
| router.rs / path.rs / liquidity.rs           |
| types.rs / errors.rs / events.rs             |
+----------------------+-----------------------+
                       |
+----------------------v-----------------------+
| Stellar Liquidity Sources                     |
| Pools, reserves, AMM pricing, asset graph     |
+----------------------------------------------+
```

## Technology Stack

- **Rust 2021** - Memory-safe systems language with strong compile-time guarantees.
- **Soroban SDK 20** - Smart contract framework for Stellar.
- **WASM** - Optimized contract compilation target.
- **Makefile** - Consistent build, test, format, and lint commands.
- **Stellar Testnet/Mainnet** - Target networks for validation and production usage.

## Public Contract API

### `find_route`

Discovers the best route between a source asset and destination asset for a given amount.

```rust
pub fn find_route(
    env: Env,
    source_asset: Symbol,
    dest_asset: Symbol,
    amount: i128,
) -> Result<Route, ContractError>
```

Responsibilities:

- Validate amount and asset inputs.
- Discover candidate paths.
- Score routes by liquidity, fees, hops, and slippage.
- Return the highest-quality route.
- Emit route discovery events.

### `simulate_route`

Calculates expected output for a route without committing execution.

```rust
pub fn simulate_route(env: Env, route: Route) -> Result<i128, ContractError>
```

Responsibilities:

- Validate route structure.
- Recalculate hop outputs.
- Estimate fees.
- Return expected destination amount.

### `execute_route`

Executes a validated payment route.

```rust
pub fn execute_route(
    env: Env,
    route: Route,
    from: Symbol,
    to: Symbol,
) -> Result<Symbol, ContractError>
```

Responsibilities:

- Validate route, sender, receiver, liquidity, and slippage.
- Execute each hop in order.
- Collect configured fees.
- Emit execution and fee events.
- Return a transaction identifier or execution symbol.

### `get_fee` and `set_fee`

Expose fee configuration in basis points.

```rust
pub fn get_fee(env: Env) -> i128

pub fn set_fee(env: Env, fee: i128) -> Result<(), ContractError>
```

`set_fee` is an admin-only operation and must reject invalid fee values.

## Core Modules

### Router: `src/router.rs`

The router is the orchestration layer. It coordinates route discovery, simulation, execution, fee lookup, fee updates, event emission, and error handling.

Key responsibilities:

- Validate top-level API inputs.
- Call pathfinding and liquidity components.
- Ensure execution respects route invariants.
- Centralize fee behavior.
- Expose stable contract behavior to integrations.

### Path Finder: `src/path.rs`

The path finder models available liquidity as an asset graph. Assets are vertices and pools are edges. Its job is to build candidate paths and select the best route.

Key responsibilities:

- Build paths between assets.
- Reject circular or malformed paths.
- Score paths by efficiency.
- Limit hop count to control execution cost.
- Support route ranking strategies such as BFS, Dijkstra, or weighted scoring.

See [Routing Algorithm](./ROUTING_ALGORITHM.md) for the detailed routing model.

### Liquidity Manager: `src/liquidity.rs`

The liquidity manager discovers pools and performs reserve-aware calculations.

Key responsibilities:

- Discover pools for relevant asset pairs.
- Query reserves.
- Calculate swap output.
- Detect insufficient liquidity.
- Apply pool-level fees.

AMM output formula:

```text
output = (input * (1 - fee) * reserve_out) / (reserve_in + input * (1 - fee))
```

In basis points:

```text
input_with_fee = input_amount * (10000 - fee_rate) / 10000
output = input_with_fee * reserve_out / (reserve_in + input_with_fee)
```

### Types: `src/types.rs`

Shared contract data structures:

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

```rust
pub struct Hop {
    pub source: Symbol,
    pub destination: Symbol,
    pub rate: i128,
    pub fee: i128,
}
```

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

```rust
pub struct Liquidity {
    pub pools: Vec<Pool>,
    pub total_value: i128,
}
```

### Errors: `src/errors.rs`

Contract errors are explicit and integration-friendly:

```rust
pub enum ContractError {
    RouteNotFound = 1,
    InvalidParams = 2,
    InsufficientLiquidity = 3,
    TransactionFailed = 4,
    Unauthorized = 5,
    SlippageExceeded = 6,
    PoolNotFound = 7,
    InvalidAsset = 8,
    RoutingError = 9,
}
```

### Events: `src/events.rs`

Events provide an audit trail for route discovery, route execution, fee collection, and configuration changes.

## Data Flow

### Route Discovery

```text
User input
  |
  v
SprRouter::find_route
  |
  v
Router validates amount and assets
  |
  v
LiquidityManager discovers candidate pools
  |
  v
PathFinder builds and scores paths
  |
  v
LiquidityManager calculates hop outputs and fees
  |
  v
Router returns best Route and emits event
```

### Simulation

```text
Route
  |
  v
Router validates route structure
  |
  v
Each hop is recalculated against current liquidity
  |
  v
Fees and minimum output are checked
  |
  v
Expected output amount is returned
```

### Execution

```text
Validated route
  |
  v
Authorization and account validation
  |
  v
Hop 1 swap
  |
  v
Hop 2 swap
  |
  v
Additional hops
  |
  v
Fee collection
  |
  v
Slippage check
  |
  v
Event emission
  |
  v
Transaction result
```

## Fee Model

Fees are represented in basis points. A fee of `10` means `0.1%`.

Design goals:

- Keep fee math predictable.
- Allow admin-configured rates.
- Emit fee events for observability.
- Reject fees below `0` or above `10000`.
- Apply fees consistently during simulation and execution.

## Design Patterns

- **Facade contract API**: `SprRouter` exposes simple public functions while module internals handle specialized behavior.
- **Module separation**: routing, pathfinding, liquidity, types, errors, and events are isolated.
- **Explicit error codes**: integration clients can handle failures predictably.
- **Simulation before execution**: users can preview a route before committing.
- **Event-sourced observability**: emitted events support monitoring and analytics.

## Key Decisions and Trade-offs

### On-chain routing

Routing on-chain improves transparency and composability, but contract execution must remain efficient. The router should limit graph traversal, hop count, and state reads.

### Basis-point fees

Basis points avoid floating-point math and keep fee calculations deterministic.

### Multi-hop support

Multi-hop routes can find better liquidity than direct routes, but each hop increases execution cost and slippage risk. Route scoring must balance output quality against complexity.

### Explicit errors

Typed contract errors make integrations easier to build and test, but require maintainers to keep error behavior stable.

## Testing Strategy

- Unit tests for pathfinding, swap math, and validation.
- Integration tests for route discovery, simulation, and execution.
- Edge-case tests for zero amounts, identical assets, insufficient liquidity, high fees, and slippage.
- Property tests for AMM invariants and fee math.
- Fuzzing for route and liquidity inputs.

## Project Organization

```text
spr-contracts/
|-- src/
|   |-- lib.rs
|   |-- router.rs
|   |-- path.rs
|   |-- liquidity.rs
|   |-- types.rs
|   |-- errors.rs
|   `-- events.rs
|-- tests/
|   `-- integration_test.rs
|-- docs/
|   |-- ARCHITECTURE.md
|   |-- ROUTING_ALGORITHM.md
|   |-- CONTRIBUTING.md
|   |-- DEPLOYMENT.md
|   `-- SECURITY.md
|-- Cargo.toml
|-- Makefile
`-- README.md
```

## Future Architecture Considerations

- Advanced weighted pathfinding.
- Dynamic fee adjustment.
- Configurable max-hop limits.
- Better liquidity source abstraction.
- Risk scoring for routes.
- Contract upgrade and migration strategy.
- Richer event schema for analytics.
- Formal verification of swap and route invariants.
