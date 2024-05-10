use std::env::current_dir;

use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use iterator::msg::{InstantiateMsg, QueryMsg};

fn main() {
    // Clear & write standard API
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        migrate: Empty,
        execute: Empty,
    }

    // Schemas for inter-contract communication
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
}
