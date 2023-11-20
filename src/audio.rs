use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};

pub fn setup_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Windless Slopes.ogg"),
        settings: PlaybackSettings {
            volume: Volume::Relative(VolumeLevel::new(0.15)),
            speed: 1.0,
            ..Default::default()
        },
    });
}
