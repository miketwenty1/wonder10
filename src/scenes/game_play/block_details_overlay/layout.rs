use crate::{
    comms::{BlockchainBlockDataFromServer, GameBlock, GameBlockDataFromServer},
    keyboard::resources::KeyboardData,
    scenes::game_play::{
        block_details_overlay::{
            components::{BackBdButton, BuyBdBlockButton},
            styles::{
                get_bd_menu_container_style, get_bd_menu_style, get_button_style,
                get_button_text_style, get_title_text_style, BACKGROUND_COLOR,
            },
        },
        events::{PlayerMove, ServerBlockchainBlockIn},
    },
    CommsApiState, GameState, KeyboardState, PlayerLocation, PlayerUsername,
};
use bevy::{input::keyboard, prelude::*};

use super::{
    components::DetailsMenu,
    rows::{
        keyboard_row, spawn_blockchain_data_row, spawn_detail_buttons_row,
        spawn_game_block_data_row, spawn_header_row, spawn_input_header_row,
        spawn_input_values_area_row,
    },
    styles::NORMAL_BUTTON,
};

#[allow(clippy::too_many_arguments)]
pub fn spawn_block_details_menu(
    mut server_block_in: EventReader<ServerBlockchainBlockIn>,
    current_blockchain_server_data: Res<BlockchainBlockDataFromServer>,
    current_gameblock_server_data: Res<GameBlockDataFromServer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut player_move_event_writer: EventWriter<PlayerMove>,
    player_location: ResMut<PlayerLocation>,
    mut game_state: ResMut<NextState<GameState>>,
    mut api_state: ResMut<NextState<CommsApiState>>,
    player_username: Res<PlayerUsername>,
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
    mut keyboard_text: ResMut<KeyboardData>,
) {
    for _event in server_block_in.iter() {
        keyboard_text.0 = "".to_string();
        // get height
        let mut height = "0";
        for k in current_blockchain_server_data.blocks.keys() {
            info!("should not find more than 1 of these: {}", k);
            height = k;
        }

        // get specific gameblock from height of incoming block
        let game_block = current_gameblock_server_data.blocks.get(height);

        match game_block {
            Some(s) => {
                let buy_amount = if s.last_payment_amount == 0 {
                    128
                } else {
                    s.last_payment_amount * 2
                };
                spawn_menu(
                    &mut commands,
                    height,
                    &asset_server,
                    &current_blockchain_server_data,
                    s.clone(),
                    buy_amount,
                    player_username.0.to_string(),
                );
                keyboard_state.set(KeyboardState::On);
            }

            None => {
                info!("please report this issue problem code: blah1");
                player_move_event_writer.send(PlayerMove {
                    block: player_location.0,
                });
                api_state.set(CommsApiState::Move);
                game_state.set(GameState::Game);
            }
        }
    }
}

fn spawn_menu(
    commands: &mut Commands,
    height: &str,
    asset_server: &Res<AssetServer>,
    current_blockchain_server_data: &Res<BlockchainBlockDataFromServer>,
    game_block: GameBlock,
    buy_amount: u32,
    player_username: String,
) {
    let blockchain_data = current_blockchain_server_data.blocks.get(height).unwrap();
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            NodeBundle {
                style: get_bd_menu_style(),
                z_index: ZIndex::Global(2),
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            },
            DetailsMenu,
        ))
        .with_children(|builder| {
            spawn_header_row(builder, font.clone(), height);
            spawn_blockchain_data_row(builder, font.clone(), blockchain_data);
            spawn_game_block_data_row(builder, font.clone(), game_block);
            spawn_input_header_row(builder, font.clone());
            spawn_input_values_area_row(builder, font.clone(), player_username, None);
            spawn_detail_buttons_row(builder, font.clone(), buy_amount);
            keyboard_row(builder);
        });
}
