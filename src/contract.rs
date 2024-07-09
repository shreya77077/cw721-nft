const CONTRACT_NAME: &str = "crates.io:orai-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.unit_price == Uint128::new(0) {
        return Err(ContractError::InvalidUnitPrice {});
    }

    if msg.max_tokens == 0 {
        return Err(ContractError::InvalidMaxTokens {});
    }

    let config = Config {
        cw721_address: None,
        cw20_address: msg.cw20_address,
        unit_price: msg.unit_price,
        max_tokens: msg.max_tokens,
        owner: info.sender,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        token_uri: msg.token_uri.clone(),
        extension: msg.extension.clone(),
        unused_token_id: 0,
    };

    CONFIG.save(deps.storage, &config)?;

    let sub_msg: Vec<SubMsg> = vec![SubMsg {
        msg: WasmMsg::Instantiate {
            code_id: msg.token_code_id,
            msg: to_binary(&Cw721InstantiateMsg {
                name: msg.name.clone(),
                symbol: msg.symbol,
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            admin: None,
            label: String::from("Instantiate fixed price NFT contract"),
        }
        .into(),
        id: INSTANTIATE_TOKEN_REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(Response::new().add_submessages(sub_msg))
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.cw721_address != None {
        return Err(ContractError::Cw721AlreadyLinked {});
    }

    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidTokenReplyId {});
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    config.cw721_address = Addr::unchecked(reply.contract_address).into();
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

let sub_msg: Vec<SubMsg> = vec![SubMsg {
    msg: WasmMsg::Instantiate {
        code_id: msg.token_code_id,
        msg: to_binary(&Cw721InstantiateMsg {
            name: msg.name.clone(),
            symbol: msg.symbol,
            minter: env.contract.address.to_string(),
        })?,
        funds: vec![],
        admin: None,
        label: String::from("Instantiate fixed price NFT contract"),
    }
    .into(),
    id: INSTANTIATE_TOKEN_REPLY_ID,
    gas_limit: None,
    reply_on: ReplyOn::Success,
}];

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.cw721_address != None {
        return Err(ContractError::Cw721AlreadyLinked {});
    }

    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidTokenReplyId {});
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    config.cw721_address = Addr::unchecked(reply.contract_address).into();
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
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