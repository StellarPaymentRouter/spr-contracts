use soroban_sdk::{
    testutils::{Address as _, Events as _},
    Address, Env, Symbol, Vec,
};
use spr_contracts::{
    errors::ContractError,
    router::Router,
    types::{Hop, Pool, Route},
    SprRouter, SprRouterClient,
};

fn sym(env: &Env, value: &str) -> Symbol {
    Symbol::new(env, value)
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
        id: sym(env, id),
        asset_in: sym(env, asset_in),
        asset_out: sym(env, asset_out),
        reserve_in,
        reserve_out,
        fee_rate,
    }
}

fn client(env: &Env) -> SprRouterClient<'_> {
    let contract_id = env.register_contract(None, SprRouter);
    SprRouterClient::new(env, &contract_id)
}

#[test]
fn amm_calculates_exact_output() {
    let output = Router::calculate_swap_output(1_000, 1_000_000, 500_000, 25);
    assert_eq!(output, Ok(498));
}

#[test]
fn amm_fee_reduces_output() {
    let without_fee = Router::calculate_swap_output(1_000, 1_000_000, 500_000, 0);
    let with_fee = Router::calculate_swap_output(1_000, 1_000_000, 500_000, 25);

    assert_eq!(without_fee, Ok(499));
    assert_eq!(with_fee, Ok(498));
}

#[test]
fn amm_rejects_zero_input() {
    let output = Router::calculate_swap_output(0, 1_000_000, 500_000, 25);
    assert_eq!(output, Err(ContractError::InvalidParams));
}

#[test]
fn amm_reports_overflow() {
    let output = Router::calculate_swap_output(i128::MAX, i128::MAX, i128::MAX, 1);
    assert_eq!(output, Err(ContractError::ArithmeticOverflow));
}

#[test]
fn bfs_finds_single_hop_first() {
    let env = Env::default();
    let pools = Router::get_mock_pools(&env);
    let graph = Router::build_pool_graph(&env, &pools);
    let paths = Router::find_paths(&env, &graph, &sym(&env, "USDC"), &sym(&env, "XLM"), 1_000);

    match paths {
        Ok(paths) => {
            assert!(paths.len() > 0);
            match paths.get(0) {
                Some(path) => {
                    assert_eq!(path.len(), 1);
                    match path.get(0) {
                        Some(first) => assert_eq!(first.asset_out, sym(&env, "XLM")),
                        None => assert!(false),
                    }
                }
                None => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn bfs_finds_multi_hop_path() {
    let env = Env::default();
    let pools = Router::get_mock_pools(&env);
    let graph = Router::build_pool_graph(&env, &pools);
    let paths = Router::find_paths(&env, &graph, &sym(&env, "USDC"), &sym(&env, "ETH"), 1_000);

    match paths {
        Ok(paths) => {
            assert!(paths.len() > 0);
            let mut found_multi_hop = false;
            let mut i = 0;
            while i < paths.len() {
                if let Some(path) = paths.get(i) {
                    if path.len() > 1 {
                        found_multi_hop = true;
                    }
                }
                i += 1;
            }
            assert!(found_multi_hop);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn bfs_returns_no_route() {
    let env = Env::default();
    let pools = Router::get_mock_pools(&env);
    let graph = Router::build_pool_graph(&env, &pools);
    let paths = Router::find_paths(&env, &graph, &sym(&env, "SINK"), &sym(&env, "USDC"), 1_000);

    assert_eq!(paths, Err(ContractError::RouteNotFound));
}

#[test]
fn bfs_enforces_max_hops() {
    let env = Env::default();
    let mut pools = Vec::new(&env);
    pools.push_back(pool(&env, "p1", "A", "B", 1_000, 1_000, 1));
    pools.push_back(pool(&env, "p2", "B", "C", 1_000, 1_000, 1));
    pools.push_back(pool(&env, "p3", "C", "D", 1_000, 1_000, 1));
    pools.push_back(pool(&env, "p4", "D", "E", 1_000, 1_000, 1));
    pools.push_back(pool(&env, "p5", "E", "F", 1_000, 1_000, 1));
    pools.push_back(pool(&env, "p6", "F", "G", 1_000, 1_000, 1));

    let graph = Router::build_pool_graph(&env, &pools);
    let paths = Router::find_paths(&env, &graph, &sym(&env, "A"), &sym(&env, "G"), 10);

    assert_eq!(paths, Err(ContractError::RouteNotFound));
}

#[test]
fn bfs_prevents_cycles() {
    let env = Env::default();
    let mut pools = Vec::new(&env);
    pools.push_back(pool(&env, "p1", "A", "B", 1_000, 1_000, 1));
    pools.push_back(pool(&env, "p2", "B", "A", 1_000, 1_000, 1));
    pools.push_back(pool(&env, "p3", "B", "C", 1_000, 1_000, 1));

    let graph = Router::build_pool_graph(&env, &pools);
    let paths = Router::find_paths(&env, &graph, &sym(&env, "A"), &sym(&env, "C"), 10);

    match paths {
        Ok(paths) => match paths.get(0) {
            Some(path) => assert_eq!(path.len(), 2),
            None => assert!(false),
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn validation_rejects_empty_path() {
    let env = Env::default();
    let route = Route {
        source_asset: sym(&env, "USDC"),
        dest_asset: sym(&env, "XLM"),
        amount: 1_000,
        path: Vec::new(&env),
        total_fee: 0,
        estimated_output: 0,
        min_received: 0,
        timestamp: 0,
    };

    assert_eq!(
        Router::validate_route(&route),
        Err(ContractError::InvalidRoute)
    );
}

#[test]
fn validation_rejects_malformed_path() {
    let env = Env::default();
    let mut path = Vec::new(&env);
    path.push_back(Hop {
        source_asset: sym(&env, "EURC"),
        dest_asset: sym(&env, "XLM"),
        input_amount: 1_000,
        output_amount: 900,
        fee: 1,
        rate: 9_000,
    });
    let route = Route {
        source_asset: sym(&env, "USDC"),
        dest_asset: sym(&env, "XLM"),
        amount: 1_000,
        path,
        total_fee: 1,
        estimated_output: 900,
        min_received: 900,
        timestamp: 0,
    };

    assert_eq!(
        Router::validate_route(&route),
        Err(ContractError::InvalidRoute)
    );
}

#[test]
fn simulation_rejects_insufficient_liquidity() {
    let env = Env::default();
    let mut path = Vec::new(&env);
    path.push_back(Hop {
        source_asset: sym(&env, "USDC"),
        dest_asset: sym(&env, "XLM"),
        input_amount: 2_000_000,
        output_amount: 1,
        fee: 0,
        rate: 1,
    });
    let route = Route {
        source_asset: sym(&env, "USDC"),
        dest_asset: sym(&env, "XLM"),
        amount: 2_000_000,
        path,
        total_fee: 0,
        estimated_output: 1,
        min_received: 1,
        timestamp: 0,
    };

    assert_eq!(
        Router::simulate_route(&env, &route, 10),
        Err(ContractError::InsufficientLiquidity)
    );
}

#[test]
fn contract_find_route_success() {
    let env = Env::default();
    env.mock_all_auths();
    let client = client(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin, &10);
    let route = client.find_route(&sym(&env, "USDC"), &sym(&env, "XLM"), &1_000);

    assert_eq!(route.source_asset, sym(&env, "USDC"));
    assert_eq!(route.dest_asset, sym(&env, "XLM"));
    assert_eq!(route.path.len(), 1);
    assert!(route.estimated_output > 0);
}

#[test]
fn contract_find_route_failure() {
    let env = Env::default();
    let client = client(&env);
    let route = client.try_find_route(&sym(&env, "USDC"), &sym(&env, "USDC"), &1_000);

    assert!(route.is_err());
}

#[test]
fn contract_simulate_route_is_deterministic() {
    let env = Env::default();
    env.mock_all_auths();
    let client = client(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin, &10);

    let route = client.find_route(&sym(&env, "USDC"), &sym(&env, "XLM"), &1_000);
    let first = client.simulate_route(&route);
    let second = client.simulate_route(&route);

    assert_eq!(first, second);
}

#[test]
fn contract_execute_route_is_mocked_success() {
    let env = Env::default();
    env.mock_all_auths();
    let client = client(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin, &10);

    let route = client.find_route(&sym(&env, "USDC"), &sym(&env, "XLM"), &1_000);
    let output = client.execute_route(&route);

    assert_eq!(output, route.estimated_output);
    assert!(env.events().all().len() >= 2);
}

#[test]
fn contract_admin_fee_updates() {
    let env = Env::default();
    env.mock_all_auths();
    let client = client(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin, &10);
    assert_eq!(client.get_fee(), 10);
    client.set_fee(&25);
    assert_eq!(client.get_fee(), 25);
}

#[test]
fn contract_unauthorized_fee_update_rejected() {
    let env = Env::default();
    let client = client(&env);
    let update = client.try_set_fee(&25);

    assert!(update.is_err());
}
