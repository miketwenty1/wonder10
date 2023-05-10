use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::PlayerName;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Debug)]
pub struct KeyBoardButton(char);

#[derive(Component)]
pub struct Capitalizable;

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

#[derive(Component)]
pub struct CapitalizeToggle(bool);

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_name: Res<PlayerName>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                size: Size::all(Val::Percent(100.)),
                /// Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                ///   - The first column will size to the size of it's contents
                ///   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::flex(1.0)], // GridTrack::min_content()
                /// Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                ///  - The first row will size to the size of it's contents
                ///  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                ///  - The third row will be exactly 20px high
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        })
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        /// Make this node span two grid columns so that it takes up the entire top tow
                        //grid_column: GridPlacement::span(1),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), "Choose a name:");
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        /// Make this node span two grid columns so that it takes up the entire top tow
                        //grid_column: GridPlacement::span(1),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), &player_name.0.to_string());
                });
            builder.spawn(NodeBundle {
                style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        /// Make this node span two grid columns so that it takes up the entire top tow
                        //grid_column: GridPlacement::span(1),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                ..default()
            });
            // .with_children(|builder| {
            //     keyboard_bundle(builder, font.clone());
            // });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        /// Make this node span two grid columns so that it takes up the entire top tow
                        //grid_column: GridPlacement::span(1),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    start_button(builder, font.clone());
                });
        });
}

fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        },
    ));
}

fn start_button(builder: &mut ChildBuilder, font: Handle<Font>) {
    // let key_chars = ["1234567890<", "qwertyuiop", "^asdfghjkl", "zxcvbnm "];

    // let number_set = "1234567890";
    // let function_set = "<^ ";
    // let letter_set = "abcdefghijklmnopqrstuvwxyz";

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
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Moon!".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Ready?".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Start".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
//commands.spawn(CapitalizeToggle(false));

// builder
//     .spawn(NodeBundle {
//         style: Style {
//             size: Size::width(Val::Percent(100.0)),
//             align_items: AlignItems::Center,
//             justify_content: JustifyContent::Center,
//             ..default()
//         },
//         ..default()
//     })
//     .with_children(|parent| {
//         parent
//             .spawn(ButtonBundle {
//                 style: Style {
//                     size: Size::new(Val::Px(150.0), Val::Px(65.0)),
//                     // horizontally center child text
//                     justify_content: JustifyContent::Center,
//                     // vertically center child text
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 background_color: NORMAL_BUTTON.into(),
//                 ..default()
//             })
//             .with_children(|parent| {
//                 parent.spawn(TextBundle::from_section(
//                     "Button",
//                     TextStyle {
//                         font,
//                         font_size: 40.0,
//                         color: Color::rgb(0.9, 0.9, 0.9),
//                     },
//                 ));
//             });
// });
