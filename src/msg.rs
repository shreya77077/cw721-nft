use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw721_base::Extension;
use cosmwasm_std::{Uint128, Binary};
// use std::marker::PhantomData;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub max_tokens: u32,
    pub unit_price: Uint128,
    pub name: String,
    pub symbol: String,
    pub token_code_id: u64,
    pub cw20_address: Addr,
    pub token_uri: String,
    pub extension: Extension,
     pub val: String,
}

pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    CustomMsg { val: String },
}

pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}


pub enum QueryMsg {
    GetConfig {},
    CustomMsg { val: String },
}

pub struct ConfigResponse {
    pub owner: Addr,
    pub cw20_address: Addr,
    pub cw721_address: Option<Addr>,
    pub max_tokens: u32,
    pub unit_price: Uint128,
    pub name: String,
    pub symbol: String,
    pub token_uri: String,
    pub extension: Extension,
    pub unused_token_id: u32,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Config {
//     pub unused_token_id: u64,
//     pub token_uri: String,
//     pub extension: Extension,
//     pub cw721_address: Option<Addr>,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     CustomMsg { val: String },
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     CustomMsg { val: String },
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
