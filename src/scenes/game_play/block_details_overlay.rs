use crate::{
    comms::BlockchainBlockDataFromServer, scenes::game_play::events::ServerBlockchainBlocksIn,
};
use bevy::prelude::*;

pub fn display_blockchain_block_details(
    mut server_block_in: EventReader<ServerBlockchainBlocksIn>,
    current_block_server_data: Res<BlockchainBlockDataFromServer>,
) {
    let block_map = &current_block_server_data.blocks;

    for _event in server_block_in.iter() {
        info!("WE WILL NOW WANT TO DISPLAY BLOCK INFO");
        info!("TEST:::: {:#?}", block_map);
    }
}
