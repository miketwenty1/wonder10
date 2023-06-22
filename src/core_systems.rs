use bevy::prelude::*;

pub fn spawn_text(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text_size: f32,
    text_color: Color,
    text: &str,
) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: text_size,
            color: text_color,
        },
    ));
}
