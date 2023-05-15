mod api_timer;
pub mod set_name;
pub mod setup;
use crate::CommsApiState;
use bevy::prelude::*;

use self::{
    api_timer::{tick_enemy_spawn_timer, ApiPollingTimer},
    set_name::api_receive_username,
};

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            .add_systems(
                Update,
                tick_enemy_spawn_timer.run_if(in_state(CommsApiState::SetName)),
            )
            .add_systems(
                Update,
                api_receive_username.run_if(in_state(CommsApiState::SetName)),
            );
    }
}
