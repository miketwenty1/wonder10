pub mod game_play;
pub mod instructions;
pub mod player_select;

use bevy::prelude::*;

fn spawn_nested_text_bundle(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    color: Color,
) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size,
            color,
        },
    ));
}
