mod api_timer;

pub mod player_move;
pub mod set_name;
pub mod setup;

use crate::CommsApiState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use self::{
    api_timer::{tick_api_receive_timer, ApiPollingTimer},
    player_move::{
        api_receive_player_movement, api_send_player_move, update_blocks_from_server_on_move,
    },
    set_name::api_receive_username,
};

#[derive(Resource, Clone, Debug, Default, Validate, Deserialize)]
pub struct BlockDataFromServer {
    pub blocks: HashMap<String, BlockData>,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct BlockData {
    pub height: u32,
    pub hash: String,
    pub x: i32,
    pub y: i32,
    pub owner: String,
    pub last_payment_amount: u64,
    pub color: String,
}

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            .init_resource::<BlockDataFromServer>()
            .add_systems(
                Update,
                tick_api_receive_timer.run_if(
                    in_state(CommsApiState::SetName).or_else(in_state(CommsApiState::Move)),
                ),
            )
            .add_systems(
                Update,
                api_receive_username.run_if(in_state(CommsApiState::SetName)),
            )
            .add_systems(
                Update,
                api_send_player_move.run_if(in_state(CommsApiState::Move)),
            )
            .add_systems(
                Update,
                api_receive_player_movement.run_if(in_state(CommsApiState::Move)),
            )
            .add_systems(Update, update_blocks_from_server_on_move);
    }
}
