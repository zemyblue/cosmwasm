use cosmwasm_std::{
    entry_point, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg,
};

use crate::errors::FilterError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MsgSendNFT};
use prost::Message;

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
        ExecuteMsg::CallSendNft {} => do_call_send_nft(),
    }
}

pub fn try_filter(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msgs: Vec<CosmosMsg>,
) -> Result<Response, FilterError> {
    // TODO:
    // let binary: &[u8] = br#"{"contract_id":"1","from":"foo","to":"bar","amount":[{"denom":"token","amount":"100"}]}"#;

    // let stargate_msg = CosmosMsg::Stargate {
    //     type_url: "/lbm.collection.v1.MsgTransferFT".to_string(),
    //     value: to_binary(binary)?,
    // };

    // Ok(Response::new()
    //     .add_attribute("action", "send_ft")
    //     .add_message(stargate_msg))
    Ok(Response::new().add_attribute("action", "send_ft"))
}

pub fn do_call_send_nft() -> Result<Response, FilterError> {
    let stargate_msg = CosmosMsg::Stargate {
        type_url: "/lbm.collection.v1.MsgSendNFT".to_string(),
        value: MsgSendNFT {
            contract_id: "1".to_string(),
            from: "foo".to_string(),
            to: "bar".to_string(),
            token_ids: vec!["tokens".to_string()],
        }
        .encode_to_vec()
        .into(),
    };

    Ok(Response::new()
        .add_attribute("action", "call_send_nft")
        .add_submessage(SubMsg::new(stargate_msg)))
}
