//mod components;
pub mod button_systems;
mod components;
mod edit_text_buttons_systems;
mod editable_text_box;
mod layout;
mod resources;
mod rows;
pub mod styles;

use crate::{despawn_screen, GameState};

use bevy::prelude::*;

use self::{
    button_systems::{interact_with_back_button, interact_with_buy_button},
    components::{DetailsMenu, EditableText},
    edit_text_buttons_systems::{
        fill_textboxes_from_keyboard, interact_with_lnaddress_edit_button,
    },
    layout::spawn_block_details_menu,
    resources::{ColorRes, LightningAddressRes, MessageRes, SelectedText},
};

pub struct BlockDetailsMenuPlugin;
impl Plugin for BlockDetailsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedText(EditableText::Msg))
            .insert_resource(LightningAddressRes("".to_string()))
            .insert_resource(ColorRes("".to_string()))
            .insert_resource(MessageRes("".to_string()))
            // OnEnter Systems
            .add_systems(
                OnEnter(GameState::BlockDetailsOverlay),
                spawn_block_details_menu,
            )
            .add_systems(
                Update,
                (
                    interact_with_buy_button,
                    interact_with_back_button,
                    interact_with_lnaddress_edit_button,
                    fill_textboxes_from_keyboard,
                )
                    .run_if(in_state(GameState::BlockDetailsOverlay)),
            )
            .add_systems(
                OnExit(GameState::BlockDetailsOverlay),
                despawn_screen::<MoveMenuNode>,
            );
    }
}
