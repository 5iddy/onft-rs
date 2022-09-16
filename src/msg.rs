use cosmwasm_std::{CosmosMsg, CustomMsg, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ONFTExecuteMsg {
    CreateDenom {
        // option (gogoproto.equal) = true;
        // string id                = 1;
        id: String,
        //   string symbol            = 2;
        symbol: String,
        //   string name              = 3;
        name: String,
        //   string description       = 4;
        description: String,
        //   string preview_uri       = 5 [
        preview_uri: String,
        //     (gogoproto.moretags)   = "yaml:\"preview_uri\"",
        //     (gogoproto.customname) = "PreviewURI"
        //   ];
        schema: String,
        //   string schema            = 6;
        sender: String, //   string sender            = 7;
    },

    UpdateDenom {
        //   string id                = 1;
        id: String,
        //   string name              = 2;
        name: String,
        //   string description       = 3;
        description: String,
        //   string preview_uri       = 4 [
        //     (gogoproto.moretags)   = "yaml:\"preview_uri\"",
        //     (gogoproto.customname) = "PreviewURI"
        //   ];
        preview_uri: String,
        //   string sender            = 5;
        sender: String,
    },

    TransferDenom {
        //   option (gogoproto.equal) = true; ?
        id: String,
        //   string id                = 1;
        sender: String,
        //   string sender            = 2;
        recipient: String, //   string recipient         = 3;
    },

    MsgMintONFT {
        //   option (gogoproto.equal) = true;

        //   string   id              = 1;
        id: String,
        //   string   denom_id        = 2 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String,
        //   Metadata metadata        = 3 [(gogoproto.nullable) = false];
        // TODO
        //   string   data            = 4;
        data: String,
        //   bool     transferable    = 5;
        transferable: bool,
        //   bool     extensible      = 6;
        extensible: bool,
        //   bool     nsfw            = 7;
        nsfw: bool,
        //   string   royalty_share   = 8 [
        //     (gogoproto.nullable)   = false,
        //     (gogoproto.moretags)   = "yaml:\"royalty_share\"",
        //     (gogoproto.customtype) = "github.com/cosmos/cosmos-sdk/types.Dec"
        //   ];
        royalty_share: Decimal,
        //   string   sender          = 9;
        sender: String,
        //   string   recipient       = 10;
        recipient: String, // }
    },

    TransferONFT {
        //   option (gogoproto.equal) = true;

        //   string id                = 1;
        id: String,
        //   string denom_id          = 2 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String,
        //   string sender            = 3;
        sender: String,
        //   string recipient         = 4;
        recipient: String,
    },

    MsgBurnONFT {
        //   option (gogoproto.equal) = true;

        //   string id                = 1;
        id: String,
        //   string denom_id          = 2 [(gogoproto.moretags) = "yaml:\"denom_id\""];
        denom_id: String,
        //   string sender            = 3;
        sender: String,
    },
}

impl CustomMsg for ONFTExecuteMsg {}

impl From<ONFTExecuteMsg> for CosmosMsg<ONFTExecuteMsg> {
    fn from(msg: ONFTExecuteMsg) -> Self {
        CosmosMsg::Custom(msg)
    }
}