use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::PlayerName;

const NORMAL_BUTTON: Color = Color::BLACK;
const HOVERED_BUTTON: Color = Color::DARK_GREEN;
const PRESSED_BUTTON: Color = Color::WHITE;

#[derive(Component, Debug)]
//pub struct KeyBoardButton(char);
pub struct KeyBoardButton(char);

#[derive(Component)]
enum KeyType {
    Letter,
    Function,
    Number,
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn setup_keyboard(builder: &mut ChildBuilder, font: Handle<Font>) {
    let key_chars = ["1234567890<", "qwertyuiop", "^asdfghjkl", "zxcvbnm "];

    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                size: Size::width(Val::Percent(100.0)),
                grid_template_columns: vec![GridTrack::flex(1.0)],
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                gap: Size {
                    width: Val::Px(0.0),
                    height: Val::Px(0.0),
                },

                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::auto(),
                    GridTrack::auto(),
                    GridTrack::auto(),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..default()
        })
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[0]);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[1]);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[2]);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[3]);
                });
        });
}

fn spawn_keyboard_row(builder: &mut ChildBuilder, font: Handle<Font>, row_keys: &str) {
    builder
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                size: Size::all(Val::Percent(50.)),
                flex_direction: FlexDirection::Row,

                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Percent(5.),
                    right: Val::Percent(5.),
                    top: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                },
                justify_content: JustifyContent::Center,
                // gap: Size {
                //     width: Val::Px(0.0),
                //     height: Val::Px(0.0),
                // },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            let key_type: KeyType;
            for key in row_keys.chars() {
                keyboard_button(builder, font.clone(), key);
            }
        });
}

fn keyboard_button(builder: &mut ChildBuilder, font: Handle<Font>, key: char) {
    let number_set = "1234567890";
    let function_set = "<^ ";
    let letter_set = "abcdefghijklmnopqrstuvwxyz";

    let key_type: KeyType;
    if letter_set.contains(key) {
        key_type = KeyType::Letter;
    } else if number_set.contains(key) {
        key_type = KeyType::Number;
    } else if function_set.contains(key) {
        key_type = KeyType::Function;
    } else {
        key_type = KeyType::Letter; //console_log!("a key is not defined as a type")
    }

    builder
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    KeyBoardButton(key),
                    key_type,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        key.to_string(),
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn keyboard_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &KeyBoardButton,
        ),
        (Changed<Interaction>, With<Button>, With<KeyBoardButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut player_text: ResMut<PlayerName>,
) {
    for (interaction, mut color, children, keyboard_button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Moon!".to_string();
                player_text.0.pop();
                console_log!("new name {:?}", player_text.0);

                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Ready?".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Start".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
