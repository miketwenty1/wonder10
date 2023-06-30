use bevy::prelude::*;

use super::{
    components::{EditableButton, EditableText},
    styles::{get_editable_button_style, BG_INPUT_COLOR, INPUT_COLOR, NORMAL_BUTTON},
};

pub fn spawn_ln_editable(builder: &mut ChildBuilder, font: Handle<Font>, default_text: &str) {
    builder
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(1.0)),
                ..Default::default()
            },
            //z_index: ZIndex::Global(3),
            background_color: BG_INPUT_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: get_editable_button_style(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    EditableButton::LN, //LightningAddressButton(false),
                ))
                .with_children(|inner| {
                    inner.spawn((
                        TextBundle::from_section(
                            default_text,
                            TextStyle {
                                font,
                                font_size: 20.0,
                                color: INPUT_COLOR,
                            },
                        ),
                        EditableText::LN, //LightningAddressText,
                    ));
                });
        });
}

pub fn spawn_color_editable(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(1.0)),
                ..Default::default()
            },
            //z_index: ZIndex::Global(3),
            background_color: BG_INPUT_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: get_editable_button_style(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    EditableButton::Color, //ColorButton(false),
                ))
                .with_children(|inner| {
                    inner.spawn((
                        TextBundle::from_section(
                            "  ",
                            TextStyle {
                                font,
                                font_size: 20.0,
                                color: INPUT_COLOR,
                            },
                        ),
                        EditableText::Color, //ColorText,
                    ));
                });
        });
}

pub fn spawn_message_editable(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(1.0)),
                ..Default::default()
            },
            //z_index: ZIndex::Global(3),
            background_color: BG_INPUT_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: get_editable_button_style(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    EditableButton::Msg, //MessageButton(false),
                ))
                .with_children(|inner| {
                    inner.spawn((
                        TextBundle::from_section(
                            "  ",
                            TextStyle {
                                font,
                                font_size: 20.0,
                                color: INPUT_COLOR,
                            },
                        ),
                        EditableText::Msg, //MessageText,
                    ));
                });
        });
}
