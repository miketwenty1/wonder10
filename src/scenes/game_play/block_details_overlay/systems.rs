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

#[allow(clippy::type_complexity)]
pub fn interact_with_buy_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuyBdBlockButton>),
    >,
    mut app_next_state: ResMut<NextState<DisplayInvoice>>,
    mut comms_api_next_state: ResMut<NextState<CommsApiState>>,
    mut button_event_reader: EventWriter<BuyBlockRequest>,
    selected_block: ResMut<SelectedBlock>,
    mut player_username: ResMut<PlayerUsername>,
) {
    info!("is this triggering?");
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                comms_api_next_state.set(CommsApiState::Buy);
                button_event_reader.send(BuyBlockRequest {
                    height: selected_block.height,
                    owner: player_username.0.to_string(),
                    refund_address: "tidwell@zbd.gg".to_string(),
                    color: "ff0000".to_lowercase(),
                    message: "hope you see this".to_string(),
                });
                //app_next_state.set(DisplayInvoice::On);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn interact_with_back_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackBdButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                game_state.set(GameState::Game);
                info!("clicked back");
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
