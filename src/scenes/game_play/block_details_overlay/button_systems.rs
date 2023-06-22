use crate::{
    scenes::game_play::{blocks_grid::SelectedBlock, events::BuyBlockRequest},
    CommsApiState, DisplayInvoice, GameState, PlayerUsername,
};
use bevy::prelude::*;

use super::{
    components::{BackBdButton, BuyBdBlockButton},
    styles::{
        BG_BUY_BTN_COLOR, BG_BUY_BTN_HOVER_COLOR, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
    },
};

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
                *color = BG_BUY_BTN_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BG_BUY_BTN_COLOR.into();
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
