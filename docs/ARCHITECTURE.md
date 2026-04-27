# SPR Contracts Architecture

## Overview

The Stellar Payment Router contracts provide on-chain routing logic for optimal payment paths across the Stellar network.

## Components

### Router (`src/router.rs`)

- Core routing engine
- Entry point for route operations
- Manages fee collection

**Key Methods:**

- `find_route()` — Discover optimal routes
- `simulate_route()` — Preview execution
- `execute_route()` — Execute transaction
- `get_fee()` / `set_fee()` — Fee management

### Path Finder (`src/path.rs`)

- Path construction algorithms
- Path validation
- Efficiency calculations

**Responsibilities:**

- Build multi-hop paths
- Validate path integrity
- Calculate path efficiency scores

### Liquidity Manager (`src/liquidity.rs`)

- Pool discovery
- Liquidity calculations
- Swap output calculations

**Key Calculations:**

- AMM formula: `output = (input × (1 - fee) × reserve_out) / (reserve_in + input × (1 - fee))`
- Slippage detection
- Reserve validation

### Types (`src/types.rs`)

- Core data structures
- Route, Hop, Pool definitions

### Errors (`src/errors.rs`)

- Contract error types
- Error codes

### Events (`src/events.rs`)

- Event emissions
- On-chain logging

## Data Flow

User Input ↓ find_route() ↓ PathFinder::build_path() ↓ LiquidityManager::calculate_swap_output() ↓ Return optimized Route

Code

## Execution Flow

execute_route(route) ↓ Validate route ↓ For each hop:

Execute swap on pool
Collect fee
Emit event ↓ Return transaction ID

Code

## Fee Model

- Base fee: 10 basis points (0.1%)
- Configurable by admin
- Collected per hop
- Emitted as event

## Security Considerations

1. **Input Validation** — All parameters validated
2. **Liquidity Checks** — Ensure sufficient reserves
3. **Slippage Protection** — Min output enforcement
4. **Reentrancy** — N/A (WASM contracts don't have this issue)
5. **Authorization** — Admin-only functions protected

## Error Handling

All errors return `ContractError` with specific codes:

- `RouteNotFound` — No path available
- `InvalidParams` — Invalid input
- `InsufficientLiquidity` — Not enough reserves
- `SlippageExceeded` — Output below minimum
- `Unauthorized` — Operation not allowed

## Testing Strategy

1. **Unit Tests** — Individual functions
2. **Integration Tests** — Component interactions
3. **Property Tests** — Mathematical properties
4. **Fuzzing** — Random input testing

## Future Enhancements

- [ ] Advanced pathfinding algorithms
- [ ] Cross-chain routing
- [ ] Dynamic fee adjustment
- [ ] Risk scoring
- [ ] Performance optimization
