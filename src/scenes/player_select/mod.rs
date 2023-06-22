use bevy::prelude::*;

use crate::{despawn_screen, GameState};

use self::player_select_layout::{
    button_system, spawn_layout, username_text_system, PlayerSelectMenu,
};
pub mod player_select_layout;
pub struct PlayerSelectPlugin;

impl Plugin for PlayerSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(GameState::PlayerSelect), spawn_layout)
            .add_systems(
                Update,
                button_system.run_if(in_state(GameState::PlayerSelect)),
            )
            // .add_systems(
            //     Update,
            //     player_vkeyboard_system.run_if(in_state(GameState::PlayerSelect)),
            // )
            // .add_systems(
            //     Update,
            //     player_pkeyboard_system.run_if(in_state(GameState::PlayerSelect)),
            // )
            // .add_systems(
            //     Update,
            //     sync_keyboard_playername.run_if(in_state(GameState::PlayerSelect)),
            // )
            .add_systems(
                Update,
                username_text_system.run_if(in_state(GameState::PlayerSelect)),
            )
            .add_systems(
                OnExit(GameState::PlayerSelect),
                despawn_screen::<PlayerSelectMenu>,
            );
    }
}
