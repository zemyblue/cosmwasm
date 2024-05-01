use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    FilterMsg { msgs: Vec<CosmosMsg> },
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
