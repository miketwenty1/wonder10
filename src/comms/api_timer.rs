use bevy::prelude::*;

pub const API_POLLING_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct ApiPollingTimer {
    pub timer: Timer,
}

impl Default for ApiPollingTimer {
    fn default() -> ApiPollingTimer {
        ApiPollingTimer {
            timer: Timer::from_seconds(API_POLLING_TIME, TimerMode::Repeating),
        }
    }
}

pub fn tick_enemy_spawn_timer(mut api_timer: ResMut<ApiPollingTimer>, time: Res<Time>) {
    api_timer.timer.tick(time.delta());
}
