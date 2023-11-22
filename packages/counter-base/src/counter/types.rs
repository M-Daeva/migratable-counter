use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ActionType {
    // TODO: 3. Add Mul
    Add,
    Sub,
}
