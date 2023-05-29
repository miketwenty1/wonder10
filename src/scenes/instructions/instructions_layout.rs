use bevy::prelude::*;

use crate::{GameState, MusicState};

use super::super::spawn_nested_text_bundle;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const INTRO_TEXT: &str = "This game is in alpha, be prepaed to lose all funds. (seriously!)\nJoin the discord\nhttps://discord.gg/wnXv4mSk\n(link expires June 4th).\nThe current goal of the game is to own land. If someone steals your block you get paid back + 10%! This game uses LN Address for refunds. If you don't have a LN Address download the ZEBEDEE app and login with ZEBEDEE otherwise use any valid LN Address for refunds.\nPick a Player Name and then have fun.\n\nInstructions:\nDouble click a block to move or use the move button and type in a block height. Click \"Details\" to look into a block.";
#[derive(Component, Debug)]
pub struct InstructionMenu;

pub fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    size: Size::all(Val::Percent(100.)),
                    grid_template_columns: vec![GridTrack::flex(1.0)],
                    grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                    ..default()
                },

                background_color: BackgroundColor(Color::GRAY),
                ..default()
            },
            InstructionMenu,
        ))
        .with_children(|builder| {
            // Choose a name
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Percent(15.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), INTRO_TEXT, 20.0, Color::BLACK);
                });
            // name input
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    setup_go_button(builder, font.clone());
                });
            // button
        });
}

fn setup_go_button(builder: &mut ChildBuilder, font: Handle<Font>) {
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
                        "Go",
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
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    mut music_state: ResMut<NextState<MusicState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Moon!".to_string();
                *color = PRESSED_BUTTON.into();
                music_state.set(MusicState::Lobby);
                game_state.set(GameState::PlayerSelect);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Moon!".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Go".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
