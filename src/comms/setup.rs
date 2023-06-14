use bevy::prelude::*;

use super::block_details::BlockDetailsChannel;
use super::player_move::PlayerMovementChannel;
use super::set_name::SetUsernameChannel;

pub fn setup_comm(mut commands: Commands) {
    let (tx_username, rx_username) = async_channel::bounded(1);
    commands.insert_resource(SetUsernameChannel {
        tx: tx_username,
        rx: rx_username,
    });
    let (tx_playermovement, rx_playermovement) = async_channel::bounded(1);
    commands.insert_resource(PlayerMovementChannel {
        tx: tx_playermovement,
        rx: rx_playermovement,
    });
    let (tx_blockdetails, rx_blockdetails) = async_channel::bounded(1);
    commands.insert_resource(BlockDetailsChannel {
        tx: tx_blockdetails,
        rx: rx_blockdetails,
    });
}
