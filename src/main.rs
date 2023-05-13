use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{keyboard::keyboard_system, player_name_menu::button_system};

mod keyboard;
mod player_name_menu;
//mod flexy;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource)]
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
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .add_systems(Update, keyboard_system)
        .add_state::<GameState>()
        .add_systems(Startup, player_name_menu::spawn_layout)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
