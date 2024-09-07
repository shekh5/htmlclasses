// use cosmwasm_std::{
//     to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, QueryRequest,
//     StdError, StdResult, Storage,
// };
// use serde::{Deserialize, Serialize};

// // Define the state of your contract
// #[derive(Serialize, Deserialize)]
// struct State {
//     value: Option<String>,
// }

// // Define your smart contract's entry points and logic
// pub fn init(_env: Env, _info: InitMsg) -> InitResponse {
//     let state = State { value: None };
//     let res = to_binary(&state);
//     InitResponse::new(res)
// }

// pub fn handle(_env: Env, _info: HandleMsg) -> StdResult<HandleResponse> {
//     // Handle logic for different message types
//     match _info {
//         HandleMsg::Set { value } => try_set(_env.storage, value),
//         HandleMsg::Get {} => try_get(_env),
//     }
// }

// fn try_set(storage: &mut dyn Storage, value: String) -> StdResult<HandleResponse> {
//     let mut state = get_state(storage)?;
//     state.value = Some(value.clone());
//     set_state(storage, &state)?;
//     Ok(HandleResponse::default())
// }

// fn try_get(_env: Env) -> StdResult<HandleResponse> {
//     let state = get_state(&_env.storage)?;
//     let res = state.value.unwrap_or_default();
//     Ok(HandleResponse::new().add_attribute("value", res))
// }

// fn get_state(storage: &dyn Storage) -> StdResult<State> {
//     let state: State = match storage.get(b"state") {
//         Some(data) => data,
//         None => State { value: None },
//     };
//     Ok(state)
// }

// fn set_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
//     storage.set(b"state", &to_binary(state)?);
//     Ok(())
// }

// // Define your smart contract's initialization message
// #[derive(Serialize, Deserialize)]
// pub struct InitMsg {}

// // Define your smart contract's handle messages
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum HandleMsg {
//     Set { value: String },
//     Get {},
// }

// // Define your smart contract's query messages
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {}

// // Define your smart contract's querier interface
// pub fn query(_env: Env, _info: QueryMsg, _querier: &Querier) -> StdResult<Binary> {
//     Err(StdError::generic_err("Query not supported"))
// }


























// use cosmwasm_std::{
//     to_binary, Api, Binary, CosmosMsg, Env, Extern, HandleResponse, InitResponse, Querier,
//     QueryRequest, StdError, StdResult, Storage,
// };
// use serde::{Deserialize, Serialize};

// // Define the state of your contract
// #[derive(Serialize, Deserialize)]
// struct State {
//     pool_balance: u64,
//     investors: Vec<Investor>,
// }

// #[derive(Serialize, Deserialize)]
// struct Investor {
//     address: String,
//     amount: u64,
// }

// // Define your smart contract's initialization message
// #[derive(Serialize, Deserialize)]
// pub struct InitMsg {
//     initial_balance: u64,
// }

// // Define your smart contract's handle messages
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum HandleMsg {
//     Invest {},
//     Withdraw {},
// }

// // Define your smart contract's query messages
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     GetPoolBalance {},
// }

// // Initialize your smart contract's state
// pub fn init(_env: Env, _info: InitMsg) -> InitResponse {
//     let state = State {
//         pool_balance: _info.initial_balance,
//         investors: vec![],
//     };
//     let res = to_binary(&state);
//     InitResponse::new(res)
// }

// // Handle investment requests
// pub fn handle_invest(
//     _env: Env,
//     _storage: &mut dyn Storage,
//     _investor_address: String,
//     _amount: u64,
// ) -> StdResult<HandleResponse> {
//     // Deduct the investment amount from the investor's account
//     // Transfer the amount to the contract's address
//     // Update the pool balance and investor list
//     Ok(HandleResponse::default())
// }

// // Handle withdrawal requests
// pub fn handle_withdraw(
//     _env: Env,
//     _storage: &mut dyn Storage,
//     _investor_address: String,
//     _amount: u64,
// ) -> StdResult<HandleResponse> {
//     // Check if the investor has sufficient balance
//     // Transfer the requested amount to the investor's address
//     // Update the pool balance and investor list
//     Ok(HandleResponse::default())
// }

// // Define your smart contract's handle function
// pub fn handle(
//     _env: Env,
//     _info: HandleMsg,
//     _storage: &mut dyn Storage,
// ) -> StdResult<HandleResponse> {
//     match _info {
//         HandleMsg::Invest {} => {
//             // Call the handle_invest function
//             // Pass the investor's address and investment amount
//             handle_invest(_env, _storage, _env.message.sender.to_string(), _env.message.sent_funds[0].amount)
//         }
//         HandleMsg::Withdraw {} => {
//             // Call the handle_withdraw function
//             // Pass the investor's address and withdrawal amount
//             handle_withdraw(_env, _storage, _env.message.sender.to_string(), _env.message.sent_funds[0].amount)
//         }
//     }
// }

// // Define your smart contract's query function
// pub fn query(
//     _env: Env,
//     _info: QueryMsg,
//     _querier: &Querier,
// ) -> StdResult<Binary> {
//     match _info {
//         QueryMsg::GetPoolBalance {} => {
//             // Retrieve the pool balance from the contract's state
//             // Return the balance as a response
//             Ok(Binary::default())
//         }
//     }
// }




















use cosmwasm_std::{
    to_binary, Api, Binary, CosmosMsg, Env, Extern, HandleResponse, InitResponse, Querier,
    QueryRequest, StdError, StdResult, Storage,
};
use serde::{Deserialize, Serialize};

// Define the state of your contract
#[derive(Serialize, Deserialize)]
struct State {
    pool_balance: u64,
    investments: Vec<Investment>,
}

#[derive(Serialize, Deserialize)]
struct Investment {
    investor: String,
    amount: u64,
}

// Define your smart contract's initialization message
#[derive(Serialize, Deserialize)]
pub struct InitMsg {
    initial_pool_balance: u64,
}

// Define your smart contract's handle messages
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Invest {},
    Withdraw {},
}

// Initialize your smart contract's state
pub fn init(_env: Env, _info: InitMsg) -> InitResponse {
    let state = State {
        pool_balance: _info.initial_pool_balance,
        investments: vec![],
    };
    let res = to_binary(&state);
    InitResponse::new(res)
}

// Handle investment requests
pub fn handle_invest(
    _env: Env,
    _storage: &mut dyn Storage,
    _investor_address: String,
    _amount: u64,
) -> StdResult<HandleResponse> {
    // Deduct the investment amount from the investor's account
    // Transfer the amount to the contract's address
    // Update the pool balance and investments list
    Ok(HandleResponse::default())
}

// Handle withdrawal requests
pub fn handle_withdraw(
    _env: Env,
    _storage: &mut dyn Storage,
    _investor_address: String,
    _amount: u64,
) -> StdResult<HandleResponse> {
    // Check if the investor has sufficient balance
    // Transfer the requested amount to the investor's address
    // Update the pool balance and investments list
    Ok(HandleResponse::default())
}

// Define your smart contract's handle function
pub fn handle(
    _env: Env,
    _info: HandleMsg,
    _storage: &mut dyn Storage,
) -> StdResult<HandleResponse> {
    match _info {
        HandleMsg::Invest {} => {
            // Call the handle_invest function
            // Pass the investor's address and investment amount
            handle_invest(_env, _storage, _env.message.sender.to_string(), _env.message.sent_funds[0].amount)
        }
        HandleMsg::Withdraw {} => {
            // Call the handle_withdraw function
            // Pass the investor's address and withdrawal amount
            handle_withdraw(_env, _storage, _env.message.sender.to_string(), _env.message.sent_funds[0].amount)
        }
    }
}

// Define your smart contract's query messages
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPoolBalance {},
}

// Define your smart contract's query function
pub fn query(
    _env: Env,
    _info: QueryMsg,
    _querier: &Querier,
) -> StdResult<Binary> {
    match _info {
        QueryMsg::GetPoolBalance {} => {
            // Retrieve the pool balance from the contract's state
            // Return the balance as a response
            Ok(Binary::default())
        }
    }
}
