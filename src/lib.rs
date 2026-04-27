#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Symbol, Vec, Env};

pub mod router;
pub mod path;
pub mod liquidity;
pub mod errors;
pub mod types;
pub mod events;

use router::Router;
use types::Route;
use errors::ContractError;

#[contract]
pub struct SprRouter;

#[contractimpl]
impl SprRouter {
    /// Find the best route between two assets
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `source_asset` - The source asset symbol
    /// * `dest_asset` - The destination asset symbol
    /// * `amount` - The amount to route
    ///
    /// # Returns
    /// A Route containing the optimal path
    ///
    /// # Errors
    /// Returns ContractError if route not found
    pub fn find_route(
        env: Env,
        source_asset: Symbol,
        dest_asset: Symbol,
        amount: i128,
    ) -> Result<Route, ContractError> {
        Router::find_route(&env, &source_asset, &dest_asset, amount)
    }

    /// Simulate route execution without committing
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `route` - The route to simulate
    ///
    /// # Returns
    /// Simulated output amount
    pub fn simulate_route(env: Env, route: Route) -> Result<i128, ContractError> {
        Router::simulate_route(&env, &route)
    }

    /// Execute a payment route
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `route` - The route to execute
    /// * `from` - Source account
    /// * `to` - Destination account
    ///
    /// # Returns
    /// Transaction hash or ID
    pub fn execute_route(
        env: Env,
        route: Route,
        from: Symbol,
        to: Symbol,
    ) -> Result<Symbol, ContractError> {
        Router::execute_route(&env, &route, &from, &to)
    }

    /// Get routing fee
    pub fn get_fee(env: Env) -> i128 {
        Router::get_fee(&env)
    }

    /// Set routing fee (admin only)
    pub fn set_fee(env: Env, fee: i128) -> Result<(), ContractError> {
        Router::set_fee(&env, fee)
    }
}