use cosmwasm_std::{
    entry_point, to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::errors::FilterError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, FilterError> {
    match msg {
        ExecuteMsg::FilterMsg { msgs } => try_filter(deps, env, info, msgs),
    }
}

pub fn try_filter(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msgs: Vec<CosmosMsg>,
) -> Result<Response, FilterError> {
    // TODO:
    let binary: &[u8] = br#"{"contract_id":"1","from":"foo","to":"bar","amount":[{"denom":"token","amount":"100"}]}"#;

    let stargate_msg = CosmosMsg::Stargate {
        type_url: "/lbm.collection.v1.MsgTransferFT".to_string(),
        value: to_binary(binary)?,
    };

    Ok(Response::new()
        .add_attribute("action", "send_ft")
        .add_message(stargate_msg))
}
