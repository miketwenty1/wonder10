use bevy::prelude::*;

use super::components::EditableText;

#[derive(Resource, Clone)]
pub struct LightningAddressRes(pub String);

#[derive(Resource, Clone)]
pub struct ColorRes(pub String);

#[derive(Resource, Clone)]
pub struct MessageRes(pub String);

#[derive(Resource)]
pub struct SelectedText(pub EditableText);
