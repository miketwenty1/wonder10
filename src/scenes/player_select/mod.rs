use bevy::prelude::*;

use crate::{despawn_screen, keyboard::capitalize_system, GameState};

use self::player_start_menu::{
    button_system, player_pkeyboard_system, player_vkeyboard_system, spawn_layout,
    username_text_system, PlayerSelectMenu,
};
pub mod player_start_menu;
pub struct PlayerSelect;

impl Plugin for PlayerSelect {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(GameState::PlayerSetup), spawn_layout)
            .add_systems(
                Update,
                button_system.run_if(in_state(GameState::PlayerSetup)),
            )
            .add_systems(
                Update,
                player_vkeyboard_system.run_if(in_state(GameState::PlayerSetup)),
            )
            .add_systems(
                Update,
                player_pkeyboard_system.run_if(in_state(GameState::PlayerSetup)),
            )
            .add_systems(
                Update,
                capitalize_system.run_if(in_state(GameState::PlayerSetup)),
            )
            .add_systems(
                Update,
                username_text_system.run_if(in_state(GameState::PlayerSetup)),
            )
            .add_systems(
                OnExit(GameState::PlayerSetup),
                despawn_screen::<PlayerSelectMenu>,
            );
    }
}
