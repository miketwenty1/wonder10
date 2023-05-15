use bevy::prelude::*;

use super::set_name::SetUsernameChannel;

pub fn setup_comm(mut commands: Commands) {
    let (tx, rx) = async_channel::bounded(1);

    commands.insert_resource(SetUsernameChannel { tx, rx });
}
