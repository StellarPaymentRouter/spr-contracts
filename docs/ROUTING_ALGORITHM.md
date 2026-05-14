# Routing Algorithm

SPR Contracts uses graph-based pathfinding to discover reliable payment routes between Stellar assets. The routing engine is designed to compare direct and multi-hop paths, account for fees and slippage, validate liquidity, and return a route that can be simulated before execution.

## Goals

- Find a usable route between source and destination assets.
- Prefer routes with strong liquidity and predictable output.
- Avoid circular paths and invalid hops.
- Keep contract execution cost bounded.
- Return deterministic route data for simulation and execution.

## Graph Model

The router treats the liquidity network as a graph:

- Assets are vertices.
- Liquidity pools are edges.
- Pool reserves, fees, and slippage risk are edge weights.
- A route is an ordered list of hops from source asset to destination asset.

Example:

```text
USDC ---- XLM
  |
 EURC ---- XLM
```

In this graph, the router may compare:

- Direct path: `USDC -> XLM`
- Multi-hop path: `USDC -> EURC -> XLM`

## Algorithm Phases

### Phase 1: Pool Discovery

The router discovers pools that can contribute to a route.

Steps:

1. Query available pools.
2. Filter pools by source, destination, or intermediate asset relevance.
3. Read reserves and pool fee rates.
4. Reject pools with invalid assets, zero reserves, or unsupported fee configuration.
5. Build graph edges from valid pools.

Pool discovery should avoid unnecessary state reads and should be optimized for Soroban WASM constraints.

### Phase 2: Path Finding

The path finder searches the graph for candidate paths.

Supported strategies:

- BFS for simple hop-limited discovery.
- Dijkstra-style scoring when edge weights include fees and slippage.
- Weighted ranking for route quality.

Constraints:

- Maximum hop count should be bounded.
- Circular routes must be rejected.
- Source and destination assets must differ.
- Paths must preserve hop continuity.

### Phase 3: Route Scoring

Candidate routes are ranked using a score that balances output quality and route risk.

Recommended scoring inputs:

- Expected destination output
- Number of hops
- Total route fee
- Pool liquidity depth
- Slippage risk
- Reserve imbalance
- Execution cost

Example cost model:

```text
cost = hop_penalty + fee_penalty + slippage_penalty + liquidity_penalty
```

Example efficiency model:

```text
efficiency = expected_output / (1 + cost)
```

Higher efficiency means a better route.

### Phase 4: Route Construction

After selecting the best path, the router constructs a `Route`.

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

Each `Hop` records the source asset, destination asset, rate, and fee:

```rust
pub struct Hop {
    pub source: Symbol,
    pub destination: Symbol,
    pub rate: i128,
    pub fee: i128,
}
```

The route should include enough information for `simulate_route()` and `execute_route()` to validate current liquidity before funds move.

### Phase 5: Validation

Validation checks:

- Amount is greater than zero.
- Source and destination are valid and different.
- Each hop connects to the next hop.
- Pools exist for every hop.
- Reserves are sufficient.
- Fee values are in range.
- Minimum output is satisfiable.
- Slippage tolerance is respected.

Failures should return explicit `ContractError` values such as `RouteNotFound`, `InvalidParams`, `InsufficientLiquidity`, `SlippageExceeded`, or `RoutingError`.

## Example Route

Scenario: route `100 USDC` to `XLM`.

Available pools:

```text
Pool 1: USDC/EURC, rate 1.2, strong liquidity
Pool 2: EURC/XLM, rate 0.5, strong liquidity
Pool 3: USDC/XLM, rate 0.6, low liquidity
```

Candidate paths:

```text
Direct:    USDC -> XLM
Multi-hop: USDC -> EURC -> XLM
```

The direct route has fewer hops, but low liquidity increases slippage risk. The multi-hop route uses deeper liquidity and may produce a more reliable final amount.

Route construction:

```text
Hop 1:
  Input:  100 USDC
  Output: 120 EURC
  Fee:    1.2 EURC

Hop 2:
  Input:  118.8 EURC
  Output: 59.4 XLM
  Fee:    0.6 XLM

Final output after route fees:
  58.8 XLM
```

## AMM Calculation

For constant-product pools, swap output can be calculated as:

```text
input_with_fee = input_amount * (10000 - fee_rate) / 10000
output = input_with_fee * reserve_out / (reserve_in + input_with_fee)
```

Validation must reject:

- `input_amount <= 0`
- `input_reserve <= 0`
- `output_reserve <= 0`
- Invalid fee rates
- Division by zero

## Slippage Handling

Slippage should be checked during simulation and again during execution. A route that was valid at discovery time may become invalid if reserves change before execution.

Execution should fail with `ContractError::SlippageExceeded` when the final output is below `min_received`.

## Edge Cases

### No Path Available

Return `ContractError::RouteNotFound`.

### Circular Route

Reject routes that revisit an asset unnecessarily, such as:

```text
USDC -> XLM -> USDC -> EURC
```

### Zero or Negative Amount

Return `ContractError::InvalidParams`.

### Insufficient Liquidity

Either select a better route or return `ContractError::InsufficientLiquidity`.

### Identical Source and Destination

Return `ContractError::InvalidParams`.

### Invalid Pool Data

Return `ContractError::PoolNotFound`, `ContractError::InvalidAsset`, or `ContractError::RoutingError` depending on the failure.

## Performance

Expected complexity for simple graph traversal:

```text
Time:  O(V + E)
Space: O(V + E)
```

Where:

- `V` is the number of assets.
- `E` is the number of pool connections.

Optimization priorities:

- Bound maximum hop count.
- Avoid repeated reserve reads.
- Short-circuit invalid paths early.
- Cache intermediate scoring data within a single invocation.
- Keep route data compact.

## Testing Strategy

Routing tests should cover:

- Direct route selection
- Multi-hop route selection
- Low-liquidity direct route avoidance
- Circular path rejection
- Slippage failure
- Invalid amount failure
- Missing pool failure
- Fee calculation
- Deterministic route scoring

Property tests should verify:

- Output never exceeds available reserves.
- Invalid reserves always fail.
- Higher fees do not increase output.
- Increasing slippage risk lowers route score.

## Future Improvements

- Configurable route scoring weights
- Dynamic fee-aware path ranking
- Parallel off-chain route candidate generation
- Risk-adjusted scoring
- Cross-chain route abstractions
- Formal verification for swap math and route invariants
