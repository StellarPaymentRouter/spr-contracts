use soroban_sdk::{contracttype, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Route {
    pub source_asset: Symbol,
    pub dest_asset: Symbol,
    pub amount: i128,
    pub path: Vec<Hop>,
    pub total_fee: i128,
    pub estimated_output: i128,
    pub min_received: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hop {
    pub source_asset: Symbol,
    pub dest_asset: Symbol,
    pub input_amount: i128,
    pub output_amount: i128,
    pub fee: i128,
    pub rate: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pool {
    pub id: Symbol,
    pub asset_in: Symbol,
    pub asset_out: Symbol,
    pub reserve_in: i128,
    pub reserve_out: i128,
    pub fee_rate: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Fee,
    Paused,
}
