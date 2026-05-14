use soroban_sdk::{Env, Map, Symbol, Vec};

use crate::errors::ContractError;
use crate::types::{Hop, Pool, Route};

const BASIS_POINTS: i128 = 10_000;
const RATE_SCALE: i128 = 10_000;
const SCORE_SCALE: i128 = 1_000_000_000;
const MAX_HOPS: u32 = 5;
const MAX_QUEUE_EXPANSIONS: u32 = 50;

pub struct Router;

impl Router {
    pub fn get_mock_pools(env: &Env) -> Vec<Pool> {
        let mut pools = Vec::new(env);

        pools.push_back(Self::pool(
            env,
            "p_usdc_xlm",
            "USDC",
            "XLM",
            1_000_000,
            500_000,
            30,
        ));
        pools.push_back(Self::pool(
            env,
            "p_usdc_eurc",
            "USDC",
            "EURC",
            1_000_000,
            990_000,
            20,
        ));
        pools.push_back(Self::pool(
            env,
            "p_eurc_xlm",
            "EURC",
            "XLM",
            900_000,
            540_000,
            25,
        ));
        pools.push_back(Self::pool(
            env,
            "p_usdc_btc",
            "USDC",
            "BTC",
            2_000_000,
            60_000,
            35,
        ));
        pools.push_back(Self::pool(
            env,
            "p_btc_xlm",
            "BTC",
            "XLM",
            50_000,
            700_000,
            30,
        ));
        pools.push_back(Self::pool(
            env,
            "p_xlm_aqua",
            "XLM",
            "AQUA",
            800_000,
            1_600_000,
            25,
        ));
        pools.push_back(Self::pool(
            env,
            "p_aqua_usdc",
            "AQUA",
            "USDC",
            1_500_000,
            700_000,
            30,
        ));
        pools.push_back(Self::pool(
            env,
            "p_eurc_eth",
            "EURC",
            "ETH",
            600_000,
            300_000,
            40,
        ));
        pools.push_back(Self::pool(
            env,
            "p_eth_xlm",
            "ETH",
            "XLM",
            250_000,
            650_000,
            35,
        ));
        pools.push_back(Self::pool(
            env,
            "p_btc_eth",
            "BTC",
            "ETH",
            80_000,
            120_000,
            30,
        ));
        pools.push_back(Self::pool(
            env,
            "p_xlm_usdc",
            "XLM",
            "USDC",
            450_000,
            900_000,
            30,
        ));
        pools.push_back(Self::pool(
            env,
            "p_eurc_dead",
            "EURC",
            "DEAD",
            300_000,
            10_000,
            50,
        ));
        pools.push_back(Self::pool(
            env,
            "p_dead_sink",
            "DEAD",
            "SINK",
            10_000,
            1_000,
            60,
        ));

        pools
    }

    pub fn build_pool_graph(env: &Env, pools: &Vec<Pool>) -> Map<Symbol, Vec<Pool>> {
        let mut graph = Map::new(env);
        let mut i = 0;

        while i < pools.len() {
            if let Some(pool) = pools.get(i) {
                let key = pool.asset_in.clone();
                let mut outgoing = match graph.get(key.clone()) {
                    Some(existing) => existing,
                    None => Vec::new(env),
                };
                outgoing.push_back(pool);
                graph.set(key, outgoing);
            }
            i += 1;
        }

        graph
    }

    pub fn find_paths(
        env: &Env,
        graph: &Map<Symbol, Vec<Pool>>,
        source_asset: &Symbol,
        dest_asset: &Symbol,
        amount: i128,
    ) -> Result<Vec<Vec<Pool>>, ContractError> {
        Self::validate_assets(source_asset, dest_asset, amount)?;

        let mut queue: Vec<Vec<Pool>> = Vec::new(env);
        let mut results: Vec<Vec<Pool>> = Vec::new(env);

        let first_pools = match graph.get(source_asset.clone()) {
            Some(pools) => pools,
            None => return Err(ContractError::RouteNotFound),
        };

        let mut i = 0;
        while i < first_pools.len() {
            if let Some(pool) = first_pools.get(i) {
                let mut path = Vec::new(env);
                path.push_back(pool);
                queue.push_back(path);
            }
            i += 1;
        }

        let mut cursor = 0;
        let mut expansions = 0;

        while cursor < queue.len() && expansions < MAX_QUEUE_EXPANSIONS {
            let path = match queue.get(cursor) {
                Some(path) => path,
                None => return Err(ContractError::RoutingError),
            };
            cursor += 1;
            expansions += 1;

            let last = Self::last_pool(&path)?;
            if last.asset_out == *dest_asset {
                if Self::path_has_liquidity(&path, amount)? {
                    Self::insert_path_by_score(env, &mut results, path)?;
                }
                continue;
            }

            if path.len() >= MAX_HOPS {
                continue;
            }

            let next_pools = match graph.get(last.asset_out.clone()) {
                Some(pools) => pools,
                None => continue,
            };

            let mut next_index = 0;
            while next_index < next_pools.len() {
                if let Some(next_pool) = next_pools.get(next_index) {
                    if !Self::would_create_cycle(&path, &next_pool.asset_out) {
                        let mut next_path = path.clone();
                        next_path.push_back(next_pool);
                        queue.push_back(next_path);
                    }
                }
                next_index += 1;
            }
        }

        if results.is_empty() {
            return Err(ContractError::RouteNotFound);
        }

        Ok(results)
    }

    pub fn score_route(path: &Vec<Pool>) -> Result<i128, ContractError> {
        if path.is_empty() {
            return Err(ContractError::InvalidRoute);
        }

        let mut fee_bps = 0_i128;
        let mut i = 0;
        while i < path.len() {
            let pool = match path.get(i) {
                Some(pool) => pool,
                None => return Err(ContractError::RoutingError),
            };
            fee_bps = fee_bps
                .checked_add(pool.fee_rate)
                .ok_or(ContractError::ArithmeticOverflow)?;
            i += 1;
        }

        let hop_count = i128::from(path.len());
        let hop_penalty = hop_count
            .checked_mul(BASIS_POINTS)
            .ok_or(ContractError::ArithmeticOverflow)?;
        let denominator = hop_penalty
            .checked_add(fee_bps)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if denominator <= 0 {
            return Err(ContractError::InvalidRoute);
        }

        SCORE_SCALE
            .checked_div(denominator)
            .ok_or(ContractError::ArithmeticOverflow)
    }

    pub fn calculate_swap_output(
        input_amount: i128,
        reserve_in: i128,
        reserve_out: i128,
        fee_rate: i128,
    ) -> Result<i128, ContractError> {
        if input_amount <= 0 || reserve_in <= 0 || reserve_out <= 0 {
            return Err(ContractError::InvalidParams);
        }
        if !(0..BASIS_POINTS).contains(&fee_rate) {
            return Err(ContractError::InvalidParams);
        }

        let fee_multiplier = BASIS_POINTS
            .checked_sub(fee_rate)
            .ok_or(ContractError::ArithmeticOverflow)?;
        let input_after_fee = input_amount
            .checked_mul(fee_multiplier)
            .ok_or(ContractError::ArithmeticOverflow)?
            .checked_div(BASIS_POINTS)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if input_after_fee <= 0 {
            return Err(ContractError::InvalidParams);
        }

        let numerator = input_after_fee
            .checked_mul(reserve_out)
            .ok_or(ContractError::ArithmeticOverflow)?;
        let denominator = reserve_in
            .checked_add(input_after_fee)
            .ok_or(ContractError::ArithmeticOverflow)?;
        let output = numerator
            .checked_div(denominator)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if output <= 0 {
            return Err(ContractError::InsufficientLiquidity);
        }

        Ok(output)
    }

    pub fn find_route(
        env: &Env,
        source_asset: Symbol,
        dest_asset: Symbol,
        amount: i128,
        router_fee_rate: i128,
    ) -> Result<Route, ContractError> {
        Self::validate_assets(&source_asset, &dest_asset, amount)?;
        Self::validate_fee(router_fee_rate)?;

        let pools = Self::get_mock_pools(env);
        let graph = Self::build_pool_graph(env, &pools);
        let paths = Self::find_paths(env, &graph, &source_asset, &dest_asset, amount)?;
        let best_path = match paths.get(0) {
            Some(path) => path,
            None => return Err(ContractError::RouteNotFound),
        };

        Self::route_from_pools(
            env,
            &source_asset,
            &dest_asset,
            amount,
            &best_path,
            router_fee_rate,
        )
    }

    pub fn simulate_route(
        env: &Env,
        route: &Route,
        router_fee_rate: i128,
    ) -> Result<Route, ContractError> {
        Self::validate_route(route)?;
        Self::validate_fee(router_fee_rate)?;

        let pools = Self::get_mock_pools(env);
        let mut matched_pools = Vec::new(env);
        let mut i = 0;

        while i < route.path.len() {
            let hop = match route.path.get(i) {
                Some(hop) => hop,
                None => return Err(ContractError::InvalidRoute),
            };
            let pool = Self::find_pool(&pools, &hop.source_asset, &hop.dest_asset)?;
            matched_pools.push_back(pool);
            i += 1;
        }

        Self::route_from_pools(
            env,
            &route.source_asset,
            &route.dest_asset,
            route.amount,
            &matched_pools,
            router_fee_rate,
        )
    }

    pub fn validate_route(route: &Route) -> Result<(), ContractError> {
        Self::validate_assets(&route.source_asset, &route.dest_asset, route.amount)?;
        if route.path.is_empty() || route.estimated_output <= 0 || route.min_received <= 0 {
            return Err(ContractError::InvalidRoute);
        }
        if route.estimated_output < route.min_received {
            return Err(ContractError::SlippageExceeded);
        }

        let mut i = 0;
        let mut previous_dest: Option<Symbol> = None;
        while i < route.path.len() {
            let hop = match route.path.get(i) {
                Some(hop) => hop,
                None => return Err(ContractError::InvalidRoute),
            };

            if hop.input_amount <= 0 || hop.output_amount <= 0 || hop.fee < 0 || hop.rate <= 0 {
                return Err(ContractError::InvalidRoute);
            }

            match previous_dest {
                Some(ref dest) => {
                    if *dest != hop.source_asset {
                        return Err(ContractError::InvalidRoute);
                    }
                }
                None => {
                    if hop.source_asset != route.source_asset {
                        return Err(ContractError::InvalidRoute);
                    }
                }
            }

            if Self::route_revisits_asset(route, i, &hop.dest_asset)
                && hop.dest_asset != route.dest_asset
            {
                return Err(ContractError::InvalidRoute);
            }

            previous_dest = Some(hop.dest_asset.clone());
            i += 1;
        }

        match previous_dest {
            Some(dest) if dest == route.dest_asset => Ok(()),
            _ => Err(ContractError::InvalidRoute),
        }
    }

    fn route_from_pools(
        env: &Env,
        source_asset: &Symbol,
        dest_asset: &Symbol,
        amount: i128,
        pools: &Vec<Pool>,
        router_fee_rate: i128,
    ) -> Result<Route, ContractError> {
        if pools.is_empty() {
            return Err(ContractError::RouteNotFound);
        }

        let mut path = Vec::new(env);
        let mut input_amount = amount;
        let mut total_fee = 0_i128;
        let mut previous_asset = source_asset.clone();
        let mut i = 0;

        while i < pools.len() {
            let pool = match pools.get(i) {
                Some(pool) => pool,
                None => return Err(ContractError::RoutingError),
            };

            if pool.asset_in != previous_asset {
                return Err(ContractError::InvalidRoute);
            }
            if input_amount > pool.reserve_in {
                return Err(ContractError::InsufficientLiquidity);
            }

            let fee = Self::calculate_fee(input_amount, pool.fee_rate)?;
            let output_amount = Self::calculate_swap_output(
                input_amount,
                pool.reserve_in,
                pool.reserve_out,
                pool.fee_rate,
            )?;
            if output_amount > pool.reserve_out {
                return Err(ContractError::InsufficientLiquidity);
            }

            total_fee = total_fee
                .checked_add(fee)
                .ok_or(ContractError::ArithmeticOverflow)?;

            let rate = output_amount
                .checked_mul(RATE_SCALE)
                .ok_or(ContractError::ArithmeticOverflow)?
                .checked_div(input_amount)
                .ok_or(ContractError::ArithmeticOverflow)?;

            path.push_back(Hop {
                source_asset: pool.asset_in.clone(),
                dest_asset: pool.asset_out.clone(),
                input_amount,
                output_amount,
                fee,
                rate,
            });

            previous_asset = pool.asset_out.clone();
            input_amount = output_amount;
            i += 1;
        }

        if previous_asset != *dest_asset {
            return Err(ContractError::InvalidRoute);
        }

        let router_fee = Self::calculate_fee(input_amount, router_fee_rate)?;
        total_fee = total_fee
            .checked_add(router_fee)
            .ok_or(ContractError::ArithmeticOverflow)?;
        let estimated_output = input_amount
            .checked_sub(router_fee)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if estimated_output <= 0 {
            return Err(ContractError::InsufficientLiquidity);
        }

        let route = Route {
            source_asset: source_asset.clone(),
            dest_asset: dest_asset.clone(),
            amount,
            path,
            total_fee,
            estimated_output,
            min_received: estimated_output,
            timestamp: env.ledger().timestamp(),
        };

        Self::validate_route(&route)?;
        Ok(route)
    }

    fn path_has_liquidity(path: &Vec<Pool>, amount: i128) -> Result<bool, ContractError> {
        let mut current_amount = amount;
        let mut i = 0;

        while i < path.len() {
            let pool = match path.get(i) {
                Some(pool) => pool,
                None => return Err(ContractError::RoutingError),
            };
            if current_amount > pool.reserve_in {
                return Ok(false);
            }
            current_amount = Self::calculate_swap_output(
                current_amount,
                pool.reserve_in,
                pool.reserve_out,
                pool.fee_rate,
            )?;
            i += 1;
        }

        Ok(true)
    }

    fn insert_path_by_score(
        env: &Env,
        results: &mut Vec<Vec<Pool>>,
        path: Vec<Pool>,
    ) -> Result<(), ContractError> {
        let path_score = Self::score_route(&path)?;
        let mut sorted = Vec::new(env);
        let mut inserted = false;
        let mut i = 0;

        while i < results.len() {
            let current = match results.get(i) {
                Some(current) => current,
                None => return Err(ContractError::RoutingError),
            };
            let current_score = Self::score_route(&current)?;

            if !inserted && path_score > current_score {
                sorted.push_back(path.clone());
                inserted = true;
            }
            sorted.push_back(current);
            i += 1;
        }

        if !inserted {
            sorted.push_back(path);
        }

        *results = sorted;
        Ok(())
    }

    fn would_create_cycle(path: &Vec<Pool>, next_asset: &Symbol) -> bool {
        let mut i = 0;
        while i < path.len() {
            if let Some(pool) = path.get(i) {
                if pool.asset_in == *next_asset || pool.asset_out == *next_asset {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    fn find_pool(
        pools: &Vec<Pool>,
        source_asset: &Symbol,
        dest_asset: &Symbol,
    ) -> Result<Pool, ContractError> {
        let mut i = 0;
        while i < pools.len() {
            let pool = match pools.get(i) {
                Some(pool) => pool,
                None => return Err(ContractError::RoutingError),
            };
            if pool.asset_in == *source_asset && pool.asset_out == *dest_asset {
                return Ok(pool);
            }
            i += 1;
        }
        Err(ContractError::RouteNotFound)
    }

    fn last_pool(path: &Vec<Pool>) -> Result<Pool, ContractError> {
        if path.is_empty() {
            return Err(ContractError::InvalidRoute);
        }
        path.get(path.len() - 1).ok_or(ContractError::RoutingError)
    }

    fn route_revisits_asset(route: &Route, current_index: u32, symbol: &Symbol) -> bool {
        if route.source_asset == *symbol {
            return true;
        }

        let mut i = 0;
        while i < current_index {
            if let Some(hop) = route.path.get(i) {
                if hop.dest_asset == *symbol {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    fn calculate_fee(amount: i128, fee_rate: i128) -> Result<i128, ContractError> {
        Self::validate_fee(fee_rate)?;
        amount
            .checked_mul(fee_rate)
            .ok_or(ContractError::ArithmeticOverflow)?
            .checked_div(BASIS_POINTS)
            .ok_or(ContractError::ArithmeticOverflow)
    }

    fn validate_assets(
        source_asset: &Symbol,
        dest_asset: &Symbol,
        amount: i128,
    ) -> Result<(), ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidParams);
        }
        if source_asset == dest_asset {
            return Err(ContractError::InvalidParams);
        }
        Ok(())
    }

    fn validate_fee(fee_rate: i128) -> Result<(), ContractError> {
        if !(0..=BASIS_POINTS).contains(&fee_rate) {
            return Err(ContractError::InvalidParams);
        }
        Ok(())
    }

    fn pool(
        env: &Env,
        id: &str,
        asset_in: &str,
        asset_out: &str,
        reserve_in: i128,
        reserve_out: i128,
        fee_rate: i128,
    ) -> Pool {
        Pool {
            id: Symbol::new(env, id),
            asset_in: Symbol::new(env, asset_in),
            asset_out: Symbol::new(env, asset_out),
            reserve_in,
            reserve_out,
            fee_rate,
        }
    }
}
