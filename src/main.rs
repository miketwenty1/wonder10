use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    audio::setup_music,
    comms::CommsPlugin,
    keyboard::{resources::KeyboardData, KeyboardPlugin},
    scenes::{
        game_play::GamePlayPlugin, instructions::InstructionsPlugin,
        player_select::PlayerSelectPlugin,
    },
};

mod audio;
mod comms;
mod consolelog;
pub mod core_systems;
mod keyboard;
mod scenes;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Instructions,
    PlayerSelect,
    Game,
    BlockDetailsOverlay,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DisplayInvoiceQr {
    #[default]
    Off,
    On,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CommsApiState {
    #[default]
    Off,
    SetName,
    Move,
    BlockDetails,
    Buy,
    CheckInvoice,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MusicState {
    #[default]
    Off,
    Lobby,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum KeyboardState {
    #[default]
    Off,
    On,
}

#[derive(Resource, Clone)]
pub struct PlayerUsername(String);

#[derive(Resource, Clone)]
pub struct PlayerLocation(u32);

#[derive(Resource, Clone)]
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
        .insert_resource(PlayerUsername(username))
        .insert_resource(PlayerLocation(69420))
        .insert_resource(ServerURL(server_url))
        .insert_resource(KeyboardData("".to_string()))
        .add_state::<GameState>()
        .add_state::<DisplayInvoiceQr>()
        .add_state::<CommsApiState>()
        .add_state::<MusicState>()
        .add_state::<KeyboardState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, comms::setup::setup_comm)
        .add_plugins(KeyboardPlugin)
        .add_plugins(InstructionsPlugin)
        .add_plugins(PlayerSelectPlugin)
        .add_plugins(GamePlayPlugin)
        .add_plugins(CommsPlugin)
        .add_systems(OnEnter(MusicState::Lobby), setup_music)
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
