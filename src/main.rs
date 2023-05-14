use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    keyboard::CapitalizeToggle,
    scenes::{game_play::GamePlay, player_select::PlayerSelect},
};

mod consolelog;
mod keyboard;
mod scenes;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    PlayerSetup,
    Game,
}

#[derive(Resource, Clone)]
pub struct PlayerName(String);

#[derive(Resource)]
pub struct ServerURL(String);

pub fn main() {
    //game("localusertesting".to_string(), "localhost:8081".to_string());
}

#[wasm_bindgen]
pub fn game(username: String, server_url: String) {
    info!("user: {}\nserver: {}", username, server_url);
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //resolution: [width as f32, height as f32].into(),
                fit_canvas_to_parent: true,
                title: "SatoshiSettlers".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(PlayerName(username))
        .insert_resource(ServerURL(server_url))
        .insert_resource(CapitalizeToggle(false))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugin(PlayerSelect)
        .add_plugin(GamePlay)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
