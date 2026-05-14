use soroban_sdk::{Env, Symbol};

pub fn emit_route_found(
    env: &Env,
    source_asset: Symbol,
    dest_asset: Symbol,
    amount: i128,
    estimated_output: i128,
    hop_count: u32,
) {
    env.events().publish(
        (Symbol::new(env, "route_found"), source_asset, dest_asset),
        (amount, estimated_output, hop_count),
    );
}

pub fn emit_route_executed(
    env: &Env,
    source_asset: Symbol,
    dest_asset: Symbol,
    input_amount: i128,
    output_amount: i128,
    total_fee: i128,
) {
    env.events().publish(
        (Symbol::new(env, "route_exec"), source_asset, dest_asset),
        (input_amount, output_amount, total_fee),
    );
}

pub fn emit_fee_updated(env: &Env, old_fee: i128, new_fee: i128) {
    env.events()
        .publish((Symbol::new(env, "fee_updated"),), (old_fee, new_fee));
}
