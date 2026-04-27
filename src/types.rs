use soroban_sdk::{Symbol, Vec};

/// Represents a single hop in a route
#[derive(Clone, Debug)]
pub struct Hop {
    pub source: Symbol,
    pub destination: Symbol,
    pub rate: i128,
    pub fee: i128,
}

/// Represents a complete route
#[derive(Clone, Debug)]
pub struct Route {
    pub source_asset: Symbol,
    pub destination_asset: Symbol,
    pub amount: i128,
    pub hops: Vec<Hop>,
    pub total_fee: i128,
    pub min_received: i128,
}

/// Represents pool information
#[derive(Clone, Debug)]
pub struct Pool {
    pub id: Symbol,
    pub asset_a: Symbol,
    pub asset_b: Symbol,
    pub reserve_a: i128,
    pub reserve_b: i128,
    pub fee_rate: i128,
}

/// Represents liquidity information
#[derive(Clone, Debug)]
pub struct Liquidity {
    pub pools: Vec<Pool>,
    pub total_value: i128,
}