pub mod game_play;
pub mod instructions;
pub mod player_select;

use bevy::prelude::*;

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 32.0,
            color: Color::BLACK,
        },
    ));
}
