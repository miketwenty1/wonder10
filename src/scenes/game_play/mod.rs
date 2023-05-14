pub mod game_grid;
use bevy::prelude::*;

use crate::GameState;

use self::game_grid::{button_system, spawn_layout};

pub struct GamePlay;
impl Plugin for GamePlay {
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
