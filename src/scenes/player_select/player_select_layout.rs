use bevy::prelude::*;

use crate::{
    comms::set_name::{api_send_username, SetUsernameChannel},
    keyboard::{
        components::{KeyBoardButton, KeyboardNode},
        resources::KeyboardData,
    },
    CommsApiState, KeyboardState, PlayerUsername, ServerURL,
};

use super::super::spawn_nested_text_bundle;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
// pub const PLAYERNAME_MAX_LENGTH: usize = 21;

// const ACCEPTABLE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 ";

#[derive(Component, Debug)]
pub struct PlayerSelectText;

#[derive(Component, Debug)]
pub struct PlayerSelectMenu;

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_name: Res<PlayerUsername>,
    mut keyboard_data: ResMut<KeyboardData>,
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    grid_template_columns: vec![GridTrack::flex(1.0)],
                    grid_template_rows: vec![
                        GridTrack::auto(),
                        GridTrack::flex(1.0),
                        GridTrack::auto(),
                        GridTrack::flex(1.0),
                    ],
                    padding: UiRect::horizontal(Val::Percent(0.0)),
                    ..default()
                },

                background_color: BackgroundColor(Color::GRAY),
                ..default()
            },
            PlayerSelectMenu,
        ))
        .with_children(|builder| {
            // Choose a name
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::top(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(
                        builder,
                        font.clone(),
                        "Choose a name:",
                        32.0,
                        Color::BLACK,
                    );
                });
            // name input
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    keyboard_data.0 = player_name.0.to_string();
                    spawn_player_text_bundle(builder, font.clone(), &player_name.0.to_string());
                });
            // keyboard row
            builder.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::top(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                },
                KeyboardNode,
            ));
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::top(Val::Px(50.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    start_button(builder, font.clone());
                });
        });
    keyboard_state.set(KeyboardState::On);
}

fn spawn_player_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: 32.0,
                color: Color::BLACK,
            },
        ),
        PlayerSelectText,
    ));
}

fn start_button(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
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
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
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

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, Without<KeyBoardButton>),
    >,
    mut text_query: Query<&mut Text>,

    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    set_username_channel: Res<SetUsernameChannel>,
    api_server: Res<ServerURL>,
    player_username: Res<PlayerUsername>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Moon!".to_string();
                *color = PRESSED_BUTTON.into();

                api_send_username(&set_username_channel, &api_server, &player_username);
                api_name_set_state.set(CommsApiState::SetName);
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

pub fn username_text_system(
    mut username_text: Query<&mut Text, With<PlayerSelectText>>,
    mut username_res: ResMut<PlayerUsername>,
    keyboard_res: Res<KeyboardData>,
) {
    for mut user_string in username_text.iter_mut() {
        user_string.sections[0].value = keyboard_res.0.to_string();
        username_res.0 = user_string.sections[0].value.to_string();
    }
}
