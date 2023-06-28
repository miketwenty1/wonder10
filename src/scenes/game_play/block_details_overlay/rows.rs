use bevy::prelude::*;

use crate::{
    comms::{BlockchainBlock, GameBlock},
    core_systems::spawn_text,
    keyboard::components::KeyboardNode,
};

use super::{
    components::{BackBdButton, BlockchainMenuNode, BuyBdBlockButton},
    editable_text_box::{spawn_color_editable, spawn_ln_editable, spawn_message_editable},
    styles::{
        get_button_style, get_button_text_style, BG_BUY_BTN_COLOR, INPUT_COLOR, NORMAL_BUTTON,
    },
};

const ALIGN_ITEMS_COLOR: Color = Color::rgb(1., 0.066, 0.349);
const JUSTIFY_CONTENT_COLOR: Color = Color::rgb(0.102, 0.522, 1.);
const MARGIN: Val = Val::Px(5.);

pub fn spawn_header_row(builder: &mut ChildBuilder, font: Handle<Font>, height: &str) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                justify_items: JustifyItems::Stretch,
                margin: UiRect::top(MARGIN),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            spawn_text(
                builder,
                font,
                32.0,
                Color::WHITE,
                format!("BLOCK HEIGHT {}", height).as_str(),
            );
        });
}

pub fn spawn_blockchain_data_row(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    blockchain_data: &BlockchainBlock,
) {
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::top(MARGIN),
                    ..Default::default()
                },
                ..Default::default()
            },
            BlockchainMenuNode,
        ))
        .with_children(|builder| {
            spawn_text(
                builder,
                font,
                10.0,
                Color::WHITE,
                format!("{:#?}", blockchain_data).as_str(),
            );
        });
}
pub fn spawn_game_block_data_row(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    game_block: GameBlock,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                justify_items: JustifyItems::Stretch,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            let gameblock_row_keys = [
                ("owner", "Owner"),
                ("last_update", "Last Update"),
                ("last_payment_amount", "Current Value"),
                ("refund_lnaddress", "Lightning Address"),
                ("color", "Color"),
                ("message", "Message"),
            ];

            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|inner| {
                    let game_block_json = serde_json::to_value(&game_block).unwrap();

                    for (key, display_key) in &gameblock_row_keys {
                        let value = game_block_json.get(*key).unwrap().to_string();
                        let val_text = remove_quotes(value.as_str());
                        spawn_text(
                            inner,
                            font.clone(),
                            20.0,
                            Color::ORANGE,
                            format!("{} : {}", display_key, val_text).as_str(),
                        );
                    }
                });
        });
}

pub fn spawn_input_header_row(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                justify_items: JustifyItems::Stretch,
                margin: UiRect::top(MARGIN),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            spawn_text(builder, font, 32.0, Color::WHITE, "Set New Values");
        });
}

pub fn spawn_input_values_area_row(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    owner: String,
    lightning_address: Option<String>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                justify_items: JustifyItems::Stretch,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_items: JustifyItems::Stretch,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|inner| {
                    inner
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                padding: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::SpaceBetween,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|inner_input_row| {
                            spawn_text(inner_input_row, font.clone(), 20.0, Color::GRAY, "Owner: ");
                            spawn_text(inner_input_row, font.clone(), 20.0, INPUT_COLOR, &owner);
                        });
                    inner
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                padding: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|inner_input_row| {
                            spawn_text(
                                inner_input_row,
                                font.clone(),
                                20.0,
                                Color::GRAY,
                                "Lightning Address: ",
                            );
                            match lightning_address {
                                Some(l) => {
                                    spawn_ln_editable(
                                        inner_input_row,
                                        font.clone(),
                                        //20.0,
                                        //INPUT_COLOR,
                                        l.as_str(),
                                    );
                                }
                                None => {
                                    spawn_ln_editable(
                                        inner_input_row,
                                        font.clone(),
                                        //20.0,
                                        //INPUT_COLOR,
                                        "satoshisettlers@zbd.gg",
                                    );
                                }
                            }
                        });
                    inner
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                padding: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|inner_input_row| {
                            spawn_text(inner_input_row, font.clone(), 20.0, Color::GRAY, "Color: ");
                            spawn_color_editable(inner_input_row, font.clone());
                        });
                    inner
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                padding: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|inner_input_row| {
                            spawn_text(
                                inner_input_row,
                                font.clone(),
                                20.0,
                                Color::GRAY,
                                "Message: ",
                            );
                            spawn_message_editable(inner_input_row, font.clone());
                        });
                });
        });
}

pub fn spawn_detail_buttons_row(builder: &mut ChildBuilder, font: Handle<Font>, buy_amount: u32) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::SpaceEvenly,
                justify_items: JustifyItems::Stretch,
                margin: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|inner| {
            inner
                .spawn((
                    ButtonBundle {
                        style: get_button_style(),
                        background_color: BG_BUY_BTN_COLOR.into(),
                        ..default()
                    },
                    BuyBdBlockButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style { ..default() },
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("Buy Block for {} sats", buy_amount),
                                get_button_text_style(font.clone()),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Back Button
            inner
                .spawn((
                    ButtonBundle {
                        style: get_button_style(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    BackBdButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style { ..default() },
                        text: Text {
                            sections: vec![TextSection::new("Back", get_button_text_style(font))],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

pub fn keyboard_row(builder: &mut ChildBuilder) {
    builder.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(MARGIN),
                justify_content: JustifyContent::SpaceEvenly,
                justify_items: JustifyItems::Stretch,
                ..Default::default()
            },
            ..Default::default()
        },
        KeyboardNode,
    ));
}

fn remove_quotes(input: &str) -> &str {
    let input = input.trim(); // Trim leading and trailing whitespace
    if input.starts_with('"') && input.ends_with('"') {
        &input[1..input.len() - 1] // Remove the first and last character
    } else {
        input // Return the input string as is
    }
}
