use bevy::prelude::*;

fn splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("splash/loading.png"),
        ..default()
    });
}
