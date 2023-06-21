use crate::{
    comms::{BlockchainBlockDataFromServer, GameBlock, GameBlockDataFromServer},
    scenes::game_play::{
        block_details_overlay::{
            styles::{
                get_bd_menu_container_style, get_bd_menu_style, get_button_style,
                get_button_text_style, get_title_text_style, BACKGROUND_COLOR,
            },
            BlockDetailsMenu,
        },
        blocks_grid::SelectedBlock,
        events::{BuyBlockRequest, PlayerMove, ServerBlockchainBlockIn},
    },
    CommsApiState, DisplayInvoice, GameState, PlayerLocation, PlayerUsername,
};
use bevy::prelude::*;

use super::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

#[derive(Component)]
pub struct BuyBdBlockButton;

#[derive(Component)]
pub struct BackBdButton;

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
) {
    for _event in server_block_in.iter() {
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
                );
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
) {
    let ent = commands
        .spawn((
            NodeBundle {
                style: get_bd_menu_style(),
                z_index: ZIndex::Global(2),
                ..default()
            },
            BlockDetailsMenu,
        ))
        .with_children(|parent| {
            //let innerblock_map = block_map;
            parent
                .spawn(NodeBundle {
                    style: get_bd_menu_container_style(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("BLOCK HEIGHT {}", height),
                                get_title_text_style(asset_server, 64.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("{:#?}", current_blockchain_server_data.blocks),
                                get_title_text_style(asset_server, 16.0),
                            )],

                            alignment: TextAlignment::Left,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("{:#?}", game_block),
                                get_title_text_style(asset_server, 16.0),
                            )],

                            alignment: TextAlignment::Left,
                            ..default()
                        },
                        ..default()
                    });
                    // Buy Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            BuyBdBlockButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        format!("Buy Block for {} sats", buy_amount),
                                        get_button_text_style(asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Back Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            BackBdButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Back",
                                        get_button_text_style(asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();
    info!("entity {:#?}", ent);
}
