use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ActionType {
    // TODO: add Mul
    Add,
    Sub,
}
