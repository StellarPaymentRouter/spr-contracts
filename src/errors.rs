use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    /// Route not found between assets
    RouteNotFound = 1,

    /// Invalid input parameters
    InvalidParams = 2,

    /// Insufficient liquidity
    InsufficientLiquidity = 3,

    /// Transaction failed
    TransactionFailed = 4,

    /// Unauthorized operation
    Unauthorized = 5,

    /// Amount exceeds slippage tolerance
    SlippageExceeded = 6,

    /// Pool not found
    PoolNotFound = 7,

    /// Invalid asset
    InvalidAsset = 8,

    /// Routing error
    RoutingError = 9,
}