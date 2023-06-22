//mod components;
mod components;
mod editable_text_box;
mod layout;
mod rows;
pub mod styles;
pub mod systems;

use crate::{despawn_screen, GameState};

use bevy::prelude::*;

use self::{
    components::DetailsMenu,
    layout::spawn_block_details_menu,
    systems::{interact_with_back_button, interact_with_buy_button},
};

pub struct BlockDetailsMenuPlugin;
impl Plugin for BlockDetailsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(
                OnEnter(GameState::BlockDetailsOverlay),
                spawn_block_details_menu,
            )
            .add_systems(
                Update,
                (interact_with_buy_button, interact_with_back_button)
                    .run_if(in_state(GameState::BlockDetailsOverlay)),
            )
            // .add_systems(
            //     Update,
            //     interact_with_back_button.run_if(in_state(GameState::BlockDetailsOverlay)),
            // )
            .add_systems(
                OnExit(GameState::BlockDetailsOverlay),
                despawn_screen::<DetailsMenu>,
            );
    }
}