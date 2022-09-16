use cosmwasm_std::{CosmosMsg, CustomMsg, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::types::Denom;

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ONFTQuery{
    Collection {
        //     string                                denom_id   = 1 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String,
        //     cosmos.base.query.v1beta1.PageRequest pagination = 2;
    },
    Denom {
        //     string denom_id = 1 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String
    },  
    QueryDenomsRequest {
        //     // pagination defines an optional pagination for the request.
        //     cosmos.base.query.v1beta1.PageRequest pagination = 1;
        //     string                                owner      = 2;
        owner: String,
    },
    ONFT {
        //     string denom_id = 1 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String,
        //     string id       = 2;
        id: String
    },          
    OwnerONFTs {
        //     string                                denom_id   = 1 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String,
        //     string                                owner      = 2;
        owner: String,
        //     cosmos.base.query.v1beta1.PageRequest pagination = 3;
    },  
    Supply {
        //     string denom_id = 1 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String,
        //     string owner    = 2;
        owner: String
    },  
    ONFTCollection {
        //     Denom         denom = 1  [(gogoproto.nullable) = false];
        denom: Denom,
        //     repeated ONFT onfts = 2 [(gogoproto.nullable) = false];
    }
}