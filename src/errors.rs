use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    RouteNotFound = 1,
    InvalidParams = 2,
    InvalidRoute = 3,
    InsufficientLiquidity = 4,
    SlippageExceeded = 5,
    Unauthorized = 6,
    ArithmeticOverflow = 7,
    RoutingError = 8,
}
