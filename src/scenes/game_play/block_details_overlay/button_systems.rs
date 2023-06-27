use crate::{
    scenes::game_play::{blocks_grid::SelectedBlock, events::BuyBlockRequest},
    CommsApiState, GameState, KeyboardState, PlayerUsername,
};
use bevy::prelude::*;

use super::{
    components::{BackBdButton, BuyBdBlockButton},
    resources::{ColorRes, LightningAddressRes, MessageRes},
    styles::{
        BG_BUY_BTN_COLOR, BG_BUY_BTN_HOVER_COLOR, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
    },
};

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn interact_with_buy_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuyBdBlockButton>),
    >,
    mut comms_api_next_state: ResMut<NextState<CommsApiState>>,
    mut button_event_reader: EventWriter<BuyBlockRequest>,
    selected_block: ResMut<SelectedBlock>,
    player_username: Res<PlayerUsername>,
    ln_res: Res<LightningAddressRes>,
    color_res: Res<ColorRes>,
    msg_res: Res<MessageRes>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let ln_addr_r = if ln_res.0.is_empty() {
                    "satoshisettlers@zbd.gg".to_string()
                } else {
                    ln_res.0.clone()
                };
                let color_r = if color_res.0.is_empty() {
                    "blue".to_string()
                } else {
                    color_res.0.clone()
                };
                *color = PRESSED_BUTTON.into();
                comms_api_next_state.set(CommsApiState::Buy);
                button_event_reader.send(BuyBlockRequest {
                    height: selected_block.height,
                    owner: player_username.0.to_string(),
                    refund_address: ln_addr_r,
                    color: color_r,
                    message: msg_res.0.to_string(),
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
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                game_state.set(GameState::Game);
                keyboard_state.set(KeyboardState::Off);
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
