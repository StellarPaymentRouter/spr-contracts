#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

pub mod errors;
pub mod events;
pub mod router;
pub mod types;

use errors::ContractError;
use router::Router;
use types::{DataKey, Route};

const DEFAULT_FEE: i128 = 10;
const MAX_FEE: i128 = 10_000;

#[contract]
pub struct SprRouter;

#[contractimpl]
impl SprRouter {
    pub fn initialize(env: Env, admin: Address, fee: i128) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::InvalidParams);
        }
        validate_fee(fee)?;
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Fee, &fee);
        env.storage().instance().set(&DataKey::Paused, &false);

        Ok(())
    }

    pub fn find_route(
        env: Env,
        source_asset: Symbol,
        dest_asset: Symbol,
        amount: i128,
    ) -> Result<Route, ContractError> {
        let fee = get_stored_fee(&env);
        let route =
            Router::find_route(&env, source_asset.clone(), dest_asset.clone(), amount, fee)?;

        events::emit_route_found(
            &env,
            source_asset,
            dest_asset,
            amount,
            route.estimated_output,
            route.path.len(),
        );

        Ok(route)
    }

    pub fn simulate_route(env: Env, route: Route) -> Result<Route, ContractError> {
        let fee = get_stored_fee(&env);
        Router::simulate_route(&env, &route, fee)
    }

    pub fn execute_route(env: Env, route: Route) -> Result<i128, ContractError> {
        let fee = get_stored_fee(&env);
        let simulated = Router::simulate_route(&env, &route, fee)?;

        events::emit_route_executed(
            &env,
            simulated.source_asset.clone(),
            simulated.dest_asset.clone(),
            simulated.amount,
            simulated.estimated_output,
            simulated.total_fee,
        );

        Ok(simulated.estimated_output)
    }

    pub fn get_fee(env: Env) -> i128 {
        get_stored_fee(&env)
    }

    pub fn set_fee(env: Env, fee: i128) -> Result<(), ContractError> {
        validate_fee(fee)?;

        let admin = match env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
        {
            Some(admin) => admin,
            None => return Err(ContractError::Unauthorized),
        };
        admin.require_auth();

        let old_fee = get_stored_fee(&env);
        env.storage().instance().set(&DataKey::Fee, &fee);
        events::emit_fee_updated(&env, old_fee, fee);

        Ok(())
    }
}

fn get_stored_fee(env: &Env) -> i128 {
    match env.storage().instance().get::<DataKey, i128>(&DataKey::Fee) {
        Some(fee) => fee,
        None => DEFAULT_FEE,
    }
}

fn validate_fee(fee: i128) -> Result<(), ContractError> {
    if !(0..=MAX_FEE).contains(&fee) {
        return Err(ContractError::InvalidParams);
    }
    Ok(())
}
