use crate::{
    comms::{events::ServerGameBocksIn, resources::GameBlockDataFromServer},
    scenes::game_play::blocks_grid::BlockButton,
};
use bevy::prelude::*;

pub fn update_blocks_from_server_on_move(
    mut server_block_in: EventReader<ServerGameBocksIn>,
    mut block_query: Query<(Entity, &mut BlockButton)>,
    mut color_query: Query<&mut BackgroundColor>,
    current_block_server_data: Res<GameBlockDataFromServer>,
) {
    let block_map = &current_block_server_data.blocks;

    for _event in server_block_in.read() {
        for (id, mut block_button) in block_query.iter_mut() {
            //info!("made it here 3");
            let bheight = block_button.height.to_string();
            let a = block_map.get(&bheight);

            match a {
                Some(s) => {
                    //info!("made it here 4");
                    let c = Color::hex(s.color.clone()).unwrap();
                    color_query.get_mut(id).unwrap().0 = c;
                    block_button.color = c;
                    block_button.paid_color = c;
                }
                None => {
                    info!("no match for height {}", bheight);
                    color_query.get_mut(id).unwrap().0 = Color::DARK_GRAY;
                    block_button.color = Color::DARK_GRAY;
                    block_button.paid_color = Color::DARK_GRAY;
                }
            }
        }
    }
}
