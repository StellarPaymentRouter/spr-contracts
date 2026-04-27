use soroban_sdk::{symbol_short, Env, Symbol};

/// Emit a route found event
pub fn emit_route_found(env: &Env, source: Symbol, destination: Symbol, amount: i128) {
    env.events().publish(
        (symbol_short!("route_found"),),
        (source, destination, amount),
    );
}

/// Emit a route executed event
pub fn emit_route_executed(env: &Env, source: Symbol, destination: Symbol, amount: i128) {
    env.events().publish(
        (symbol_short!("executed"),),
        (source, destination, amount),
    );
}

/// Emit a fee collected event
pub fn emit_fee_collected(env: &Env, amount: i128) {
    env.events().publish(
        (symbol_short!("fee_collected"),),
        (amount,),
    );
}