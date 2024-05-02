use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg;
use prost::Message;
use serde::{Serialize, Deserialize};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    FilterMsg { msgs: Vec<CosmosMsg> },
    CallSendNft {},
}

#[cw_serde]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

#[cw_serde]
pub struct MsgSendFT {
    pub contract_id: String,
    pub from: String,
    pub to: String,
    pub amount: Vec<Coin>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct MsgSendNFT {
    #[prost(string, tag = "1")]
    #[serde(alias = "ContractID")]
    pub contract_id: String,

    #[prost(string, tag = "2")]
    pub from: String,

    #[prost(string, tag = "3")]
    pub to: String,

    #[prost(string, repeated, tag = "4")]
    #[serde(alias = "tokenIDs")]
    pub token_ids: Vec<String>,
}
