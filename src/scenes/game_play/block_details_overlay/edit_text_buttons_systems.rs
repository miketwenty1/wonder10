use crate::{
    keyboard::resources::KeyboardData,
    scenes::game_play::{blocks_grid::SelectedBlock, events::BuyBlockRequest},
    CommsApiState, DisplayInvoice, GameState, KeyboardState, PlayerUsername,
};
use bevy::prelude::*;

use super::{
    components::{
        BackBdButton, BuyBdBlockButton, EditableButton, EditableText, LightningAddressButton,
        LightningAddressText,
    },
    resources::{ColorRes, LightningAddressRes, MessageRes, SelectedText},
    //resources::LightningAddressRes,
    styles::{
        BG_BUY_BTN_COLOR, BG_BUY_BTN_HOVER_COLOR, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
    },
};

#[allow(clippy::type_complexity)]
pub fn interact_with_lnaddress_edit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &EditableButton),
        (Changed<Interaction>, With<EditableButton>),
    >,
    mut keyboard_data: ResMut<KeyboardData>,
    mut selected_text: ResMut<SelectedText>,
    mut ln_res: ResMut<LightningAddressRes>,
    mut color_res: ResMut<ColorRes>,
    mut msg_res: ResMut<MessageRes>,
) {
    for (interaction, mut color, btn_comp) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // first cache select text to a resource from the keyboard
                match selected_text.0 {
                    EditableText::LN => {
                        ln_res.0 = keyboard_data.0.to_string();
                    }
                    EditableText::Color => {
                        color_res.0 = keyboard_data.0.to_string();
                    }
                    EditableText::Msg => {
                        msg_res.0 = keyboard_data.0.to_string();
                    }
                }

                // Now, new text selected for keyboard input and populate/resumed with current data for that resource
                match btn_comp {
                    EditableButton::LN => {
                        selected_text.0 = EditableText::LN;
                        keyboard_data.0 = ln_res.0.to_string();
                    }
                    EditableButton::Color => {
                        selected_text.0 = EditableText::Color;
                        keyboard_data.0 = color_res.0.to_string();
                    }
                    EditableButton::Msg => {
                        selected_text.0 = EditableText::Msg;
                        keyboard_data.0 = msg_res.0.to_string();
                    }
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Go".to_string();
                *color = BG_BUY_BTN_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BG_BUY_BTN_COLOR.into();
            }
        }
    }
    //match selected_tex
}

//

pub fn fill_textboxes_from_keyboard(
    mut text_query: Query<(&mut Text, &EditableText), With<EditableText>>,
    mut keyboard_data: ResMut<KeyboardData>,
    mut selected_text: ResMut<SelectedText>,
    mut ln_res: ResMut<LightningAddressRes>,
    mut color_res: ResMut<ColorRes>,
    mut msg_res: ResMut<MessageRes>,
) {
    for (mut textbox, editable_type) in text_query.iter_mut() {
        match selected_text.0 {
            EditableText::LN => {
                if let EditableText::LN = editable_type {
                    if keyboard_data.0 == *"" {
                        textbox.sections[0].value = "  ".to_string();
                    } else {
                        textbox.sections[0].value = keyboard_data.0.to_string();
                        ln_res.0 = keyboard_data.0.to_string();
                    }
                }
            }
            EditableText::Color => {
                if let EditableText::Color = editable_type {
                    if keyboard_data.0 == *"" {
                        textbox.sections[0].value = "  ".to_string();
                    } else {
                        textbox.sections[0].value = keyboard_data.0.to_string();
                        color_res.0 = keyboard_data.0.to_string();
                    }
                }
            }
            EditableText::Msg => {
                if let EditableText::Msg = editable_type {
                    if keyboard_data.0 == *"" {
                        textbox.sections[0].value = "  ".to_string();
                    } else {
                        textbox.sections[0].value = keyboard_data.0.to_string();
                        msg_res.0 = keyboard_data.0.to_string();
                    }
                }
            }
        }
    }
}

// #[allow(clippy::type_complexity)]
// pub fn interact_with_back_button(
//     mut button_query: Query<
//         (&Interaction, &mut BackgroundColor),
//         (Changed<Interaction>, With<BackBdButton>),
//     >,
//     mut game_state: ResMut<NextState<GameState>>,
//     mut keyboard_state: ResMut<NextState<KeyboardState>>,
// ) {
//     for (interaction, mut color) in button_query.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => {
//                 *color = PRESSED_BUTTON.into();
//                 game_state.set(GameState::Game);
//                 keyboard_state.set(KeyboardState::Off);
//             }
//             Interaction::Hovered => {
//                 *color = HOVERED_BUTTON.into();
//             }
//             Interaction::None => {
//                 *color = NORMAL_BUTTON.into();
//             }
//         }
//     }
// }
