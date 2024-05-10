use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MapEntriesResponse)]
    MapValues {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct MapEntriesResponse {
    pub values: Vec<bool>,
}
