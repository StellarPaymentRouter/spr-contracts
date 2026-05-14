# Security Policy

SPR Contracts is designed to route Stellar payments through Soroban smart contracts. Because routing contracts can affect user funds, security reports are treated with priority and should be disclosed responsibly.

## Supported Versions

The project follows the active `main` branch during early open-source development. Security fixes are applied to `main` first. When tagged releases are introduced, this section should list the currently supported release lines.

| Version | Supported |
| --- | --- |
| `main` | Yes |
| Tagged releases | To be defined |

## Reporting a Vulnerability

Please do not disclose suspected vulnerabilities publicly until maintainers have reviewed and remediated the issue.

Recommended reporting process:

1. Open a private security advisory on GitHub if repository permissions allow it.
2. If private advisories are unavailable, contact the maintainers through the repository owner organization and request a private disclosure channel.
3. Include a clear description, affected code paths, reproduction steps, impact, and any suggested remediation.
4. Avoid publishing exploit details, proof-of-concept code, or transaction examples before a fix is available.

Expected maintainer response:

- Acknowledge the report as soon as possible.
- Reproduce and assess severity.
- Prepare a fix and tests.
- Coordinate disclosure timing with the reporter.
- Publish remediation notes after affected users have a reasonable update path.

## Security Model

SPR Contracts focuses on secure payment route discovery, simulation, execution, and fee collection.

Primary security goals:

- Reject invalid routes before execution.
- Prevent execution when liquidity is insufficient.
- Enforce slippage and minimum-output constraints.
- Protect admin-controlled configuration.
- Emit useful events for monitoring and audit trails.
- Return explicit `ContractError` values instead of panicking.

Primary assets to protect:

- User payment amounts
- Route outputs and minimum received values
- Pool reserve assumptions
- Fee configuration
- Admin authority
- Contract deployment identity and WASM artifact integrity

## Contract Security Considerations

### Input Validation

All public contract functions should validate:

- Asset identifiers
- Amounts greater than zero
- Source and destination asset differences
- Route hop ordering
- Fee values in valid basis-point ranges
- Pool existence and reserves
- Slippage limits

Invalid inputs should return `ContractError::InvalidParams`, `ContractError::InvalidAsset`, `ContractError::RouteNotFound`, or another specific error.

### Arithmetic Safety

Routing and liquidity math should avoid overflow, underflow, and division by zero. AMM calculations should validate reserves and fees before multiplication or division.

Recommended practices:

- Use checked arithmetic for high-risk operations.
- Validate denominator values before division.
- Keep fee calculations in basis points.
- Test large values, low liquidity, maximum fees, and precision-sensitive routes.

### Authorization

Admin operations such as `set_fee()` must require explicit authorization from the configured admin account before mutating contract configuration.

The deployment process should protect:

- Admin secret keys
- Deployment source account
- Network passphrase
- Contract ID records
- Upgrade and rollback procedures

### Slippage and Liquidity

Route execution must enforce the route's minimum output. A route that simulated successfully should still fail if reserves change before execution and the final output falls below the expected minimum.

### Atomicity

Multi-hop execution should be atomic: if any hop fails, the full payment route should fail. Intermediate asset movement should not leave users with partial route state.

### Events and Monitoring

Events should be emitted for:

- Route discovery
- Route execution
- Fee collection
- Configuration changes
- Relevant failures when practical

Monitoring systems can use these events to detect unexpected execution patterns, fee changes, repeated failures, and liquidity problems.

## Dependency Security

SPR Contracts currently depends on:

- `soroban-sdk`
- Rust standard tooling for build and test
- Soroban CLI for deployment

Recommended dependency practices:

- Keep `Cargo.lock` under review when dependencies are added.
- Run `cargo audit` when available.
- Review Soroban SDK release notes before upgrades.
- Avoid unnecessary dependencies in contract code.
- Prefer deterministic builds for release artifacts.

## Data Handling

The contract should not store private user secrets. Users and operators must never commit private keys, seed phrases, or production credentials.

Configuration examples such as `.env.example` may include public RPC URLs and placeholder values only.

## User Best Practices

Before deploying or integrating:

- Build from a trusted commit.
- Verify the WASM artifact path and hash.
- Deploy to testnet before mainnet.
- Test route simulation and execution with representative assets.
- Keep admin keys in secure key management.
- Configure conservative fees and slippage limits.
- Monitor route execution events after deployment.

## Production Security Checklist

- [ ] `make all` passes.
- [ ] Contract WASM hash is recorded.
- [ ] Admin account is secured.
- [ ] Fee configuration is reviewed.
- [ ] Slippage behavior is tested.
- [ ] Failure modes are tested.
- [ ] Deployment command and network passphrase are verified.
- [ ] Testnet deployment succeeds.
- [ ] Monitoring is configured.
- [ ] Rollback plan is documented.
