mod instructions_layout;
use bevy::prelude::*;

use crate::{despawn_screen, GameState};

use self::instructions_layout::{button_system, spawn_layout, InstructionMenu};

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(GameState::Instructions), spawn_layout)
            .add_systems(
                Update,
                button_system.run_if(in_state(GameState::Instructions)),
            )
            .add_systems(
                OnExit(GameState::Instructions),
                despawn_screen::<InstructionMenu>,
            );
    }
}
