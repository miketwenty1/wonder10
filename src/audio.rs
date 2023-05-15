use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};

pub fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_with_settings(
        asset_server.load("sounds/Windless Slopes.ogg"),
        PlaybackSettings {
            repeat: true,
            volume: Volume::Relative(VolumeLevel::new(0.15)),
            speed: 1.0,
        },
    );
}
