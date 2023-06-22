use bevy::prelude::*;

use super::{
    components::{ColorInput, LightningAddressInput, MessageInput},
    styles::{BG_INPUT_COLOR, INPUT_COLOR},
};

pub fn spawn_ln_editable(builder: &mut ChildBuilder, font: Handle<Font>, default_text: &str) {
    builder
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            //z_index: ZIndex::Global(3),
            background_color: BG_INPUT_COLOR.into(),
            ..default()
        },))
        .with_children(|inner| {
            inner.spawn((
                TextBundle::from_section(
                    "satoshisettlers@zbd.gg",
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: INPUT_COLOR,
                    },
                ),
                LightningAddressInput,
            ));
        });
}

pub fn spawn_color_editable(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            //z_index: ZIndex::Global(3),
            background_color: BG_INPUT_COLOR.into(),
            ..default()
        },))
        .with_children(|inner| {
            inner.spawn((
                TextBundle::from_section(
                    "blue",
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: INPUT_COLOR,
                    },
                ),
                ColorInput,
            ));
        });
}

pub fn spawn_message_editable(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            //z_index: ZIndex::Global(3),
            background_color: BG_INPUT_COLOR.into(),
            ..default()
        },))
        .with_children(|inner| {
            inner.spawn((
                TextBundle::from_section(
                    "custom message",
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: INPUT_COLOR,
                    },
                ),
                MessageInput,
            ));
        });
}
