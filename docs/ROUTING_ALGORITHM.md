# Routing Algorithm

## Overview

The Stellar Payment Router uses a graph-based pathfinding algorithm to discover optimal payment routes.

## Algorithm Overview

### Phase 1: Pool Discovery

1. Scan available liquidity pools
2. Identify pools with source and destination assets
3. Build bidirectional graph

### Phase 2: Path Finding

1. Use BFS/Dijkstra to find paths
2. Rank paths by efficiency
3. Apply slippage filtering

### Phase 3: Route Construction

1. Build hop sequence
2. Calculate intermediate amounts
3. Determine fees

### Phase 4: Validation

1. Check liquidity sufficiency
2. Verify rates
3. Validate output

## Example

**Scenario:** Route 100 USDC → XLM

Available Pools:

Pool 1: USDC/EURC (rate: 1.2)
Pool 2: EURC/XLM (rate: 0.5)
Pool 3: USDC/XLM (rate: 0.6, low liquidity)

Path Finding:

Direct: USDC → XLM (rate: 0.6) ❌ Low liquidity
Multi-hop: USDC → EURC → XLM (rate: 1.2 × 0.5 = 0.6) ✅

Route:

Hop 1: USDC → 100 USDC
Output: 100 × 1.2 = 120 EURC
Fee: 1.2 EURC

Hop 2: 120 EURC → 118.8 EURC
Output: 118.8 × 0.5 = 59.4 XLM
Fee: 0.6 XLM

Final Output: 58.8 XLM (after fee)

Code

## Cost Function

efficiency = 1 / (hops_count + fees + slippage_risk)

Code

Higher efficiency = better route

## Edge Cases

1. **No Path Available** → Error
2. **Circular Routes** → Detected and rejected
3. **Zero Amount** → Error
4. **Insufficient Liquidity** → Error or alternative path

## Performance

- Time: O(V + E) where V = pools, E = connections
- Space: O(V + E) for graph storage
- Optimized for < 1 second resolution

## Future Improvements

- Parallel pathfinding
- ML-based optimization
- Cross-chain routes
- Risk-adjusted scoring
