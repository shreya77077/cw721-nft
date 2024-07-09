use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw721_base::Extension;


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
}

pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender,
            amount,
            msg,
        }) => execute_receive(deps, info, sender, amount, msg),
    }
}

pub fn execute_receive(
    deps: DepsMut,
    info: MessageInfo,
    sender: String,
    amount: Uint128,
    _msg: Binary,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.cw20_address != info.sender {
        return Err(ContractError::UnauthorizedTokenContract {});
    }

    if config.cw721_address == None {
        return Err(ContractError::Uninitialized {});
    }

    if config.unused_token_id >= config.max_tokens {
        return Err(ContractError::SoldOut {});
    }

    if amount != config.unit_price {
        return Err(ContractError::WrongPaymentAmount {});
    }

    let mint_msg = Cw721ExecuteMsg::<Extension, Empty>::Mint(MintMsg::<Extension> {
        token_id: config.unused_token_id.to_string(),
        owner: sender,
        token_uri: config.token_uri.clone().into(),
        extension: config.extension.clone(),
    });

    match config.cw721_address.clone() {
        Some(cw721) => {
            let callback =
                Cw721Contract::<Empty, Empty>(cw721, PhantomData, PhantomData).call(mint_msg)?;
            config.unused_token_id += 1;
            CONFIG.save(deps.storage, &config)?;

            Ok(Response::new().add_message(callback))
        }
        None => Err(ContractError::Cw721NotLinked {}),
    }
}

//const mint_msg = Cw721ExecuteMsg::<Extension, Empty>::Mint(MintMsg::<Extension> {
const MINT_MSG = Cw721ExecuteMsg::<Extension, Empty>::Mint(MintMsg::<Extension> {

// const MINT_MSG: TypeOfMintMsg = Cw721ExecuteMsg::<Extension, Empty>::Mint(MintMsg::<Extension> { 
//     // ... });

    token_id: config.unused_token_id.to_string(),
    owner: sender,
    token_uri: config.token_uri.clone().into(),
    extension: config.extension.clone(),
});

match config.cw721_address.clone() {
    Some(cw721) => {
        let callback =
            Cw721Contract::<Empty, Empty>(cw721, PhantomData, PhantomData).call(mint_msg)?;
        config.unused_token_id += 1;
        CONFIG.save(deps.storage, &config)?;

        Ok(Response::new().add_message(callback))
    }
    None => Err(ContractError::Cw721NotLinked {}),
}

pub enum QueryMsg {
    GetConfig {},
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
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        cw20_address: config.cw20_address,
        cw721_address: config.cw721_address,
        max_tokens: config.max_tokens,
        unit_price: config.unit_price,
        name: config.name,
        symbol: config.symbol,
        token_uri: config.token_uri,
        extension: config.extension,
        unused_token_id: config.unused_token_id,
    })
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CustomMsg { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CustomMsg { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
