pub mod blocks_grid;
pub mod game_layout;

use bevy::prelude::*;

use crate::GameState;

use self::blocks_grid::button_system;
use self::game_layout::spawn_layout;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(GameState::Game), spawn_layout)
            .add_systems(Update, button_system.run_if(in_state(GameState::Game)));
        // .add_systems(
        //     OnExit(GameState::PlayerSetup),
        //     despawn_screen::<PlayerSelectMenu>,
        // );
    }
}
