use cosmwasm_std::Decimal;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
// use protobuf::well_known_types::timestamp::Timestamp;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all="snake_case")]
pub struct Denom {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub schema: String,
    pub creator: String,
    pub description: String,
    pub preview_uri: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all="snake_case")]
pub struct ONFT {
    pub id: String,
    pub metadata: Metadata,
    pub data: String,
    pub owner: String,
    pub transferable: bool,
    pub extensible: bool,
    // created_at protobuf.Timestamp
    // pub created_at: Timestamp,
    pub nsfw: bool,
    pub royalty_share: Decimal
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all="snake_case")]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub media_uri: String,
    pub preview_uri: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all="snake_case")]
pub struct Owner {
    address: String,
    id_collections: Vec<IDCollection>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all="snake_case")]
pub struct IDCollection {
    pub denom_id: String,
    pub onft_ids: Vec<String>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all="snake_case")]
pub struct Collection {
    pub denom: Denom,
    pub onfts: Vec<ONFT>
}