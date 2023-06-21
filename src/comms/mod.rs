mod api_timer;

pub mod block_details;
pub mod invoice;
pub mod player_move;
pub mod set_name;
pub mod setup;

use crate::{CommsApiState, DisplayInvoice};
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use self::{
    api_timer::{tick_api_receive_timer, ApiPollingTimer},
    block_details::{api_receive_blockchain_block, api_request_blockchain_block},
    invoice::{api_check_invoice, api_request_invoice},
    player_move::{api_receive_player_movement, api_send_player_move},
    set_name::api_receive_username,
};

#[derive(Resource, Clone, Debug, Default, Validate, Deserialize)]
pub struct GameBlockDataFromServer {
    pub blocks: HashMap<String, GameBlock>,
}

#[derive(Resource, Clone, Debug, Default, Validate, Deserialize)]
pub struct BlockchainBlockDataFromServer {
    pub blocks: HashMap<String, BlockchainBlock>,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct BlockchainBlock {
    #[validate(length(equal = 64))]
    pub hash: String,
    pub ver: u64,
    #[validate(length(equal = 64))]
    pub prev_block: String,
    #[validate(length(equal = 64))]
    pub mrkl_root: String,
    pub time: u64,
    pub bits: u64,
    pub fee: u64,
    pub nonce: u64,
    pub n_tx: u32,
    pub size: u32,
    pub main_chain: bool,
    #[validate(range(min = 0, max = 1_000_000))]
    pub height: u32,
    #[validate(range(min = 0, max = 4_000_000))]
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct GameBlock {
    #[validate(range(min = 0, max = 1_000_000))]
    pub height: u32,
    pub color: String,
    pub owner: String,
    pub x: i32,
    pub y: i32,
    pub last_payment_amount: u32,
    pub invoice: Option<String>,
    pub invoice_expiry: Option<DateTime<Utc>>,
    #[validate(length(max = 140))]
    pub message: String,
    pub last_update: DateTime<Utc>,
    pub refund_lnaddress: String,
}

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            .init_resource::<GameBlockDataFromServer>()
            .init_resource::<BlockchainBlockDataFromServer>()
            // EVERY STATE FOR RECEIVING API CALLS NEEDS TO TICK
            .add_systems(
                Update,
                tick_api_receive_timer.run_if(in_state(CommsApiState::SetName).or_else(
                    in_state(CommsApiState::Move).or_else(
                        in_state(CommsApiState::BlockDetails).or_else(in_state(CommsApiState::Buy)), // .or_else(in_state(CommsApiState::InvoiceCheck)),
                    ),
                )),
            )
            .add_systems(
                Update,
                api_receive_username.run_if(in_state(CommsApiState::SetName)),
            )
            .add_systems(Update, (api_send_player_move, api_request_blockchain_block))
            //.add_systems(Update, )
            .add_systems(
                Update,
                // api_receive_player_movement.run_if(in_state(CommsApiState::Move)),
                api_receive_player_movement,
            )
            .add_systems(
                Update,
                api_receive_blockchain_block.run_if(in_state(CommsApiState::BlockDetails)),
            )
            .add_systems(
                Update,
                (
                    api_request_invoice.run_if(in_state(CommsApiState::Buy)),
                    api_check_invoice.run_if(in_state(DisplayInvoice::On)),
                ),
            );
    }
}
