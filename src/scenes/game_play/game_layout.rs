// we need a button to view details of blocks
use crate::PlayerUsername;

use super::blocks_grid::spawn_blocks;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::DARK_GRAY;

#[derive(Component, Debug)]
pub struct GameLayout;

#[derive(Component, Debug)]
pub struct LocationText;

#[derive(Component, Debug)]
pub struct UsernameText;

#[derive(Component, Debug)]
pub struct BalanceText;

#[derive(Component, Debug)]
pub struct BlockDetailsButton;

#[derive(Component, Debug)]
pub struct MoveButton;

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_username: Res<PlayerUsername>,
    mut has_spawned: Local<bool>,
) {
    if !(*has_spawned) {
        *has_spawned = true;

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        // Top-level grid (app frame)
        commands
            .spawn((
                NodeBundle {
                    z_index: ZIndex::Global(1),
                    style: Style {
                        display: Display::Grid,
                        size: Size::all(Val::Percent(100.)),
                        grid_template_columns: vec![
                            GridTrack::auto(),
                            GridTrack::min_content(),
                            GridTrack::auto(),
                        ],
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::min_content(),
                            GridTrack::auto(),
                        ],
                        //padding: UiRect::horizontal(Val::Percent(5.0)),
                        ..default()
                    },

                    background_color: BackgroundColor(Color::GRAY),
                    ..default()
                },
                GameLayout,
            ))
            .with_children(|builder| {
                // Choose a name
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            grid_column: GridPlacement::start(2),
                            //padding: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_location_text(builder, font.clone(), "0");
                        spawn_username_text(builder, font.clone(), &player_username.0);
                        spawn_balance_text(builder, font.clone(), "0");
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            grid_column: GridPlacement::start(2),
                            grid_row: GridPlacement::start(2),
                            //padding: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| spawn_blocks(builder, font.clone()));
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            grid_column: GridPlacement::start(2),
                            grid_row: GridPlacement::start(3),
                            //padding: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| spawn_select_block_buttons(builder, font.clone()));
            });
    }
}

fn spawn_location_text(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Block Location: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::BLACK,
                },
            ),
            TextSection::new(
                text,
                TextStyle {
                    font,
                    font_size: 32.0,
                    color: Color::BLACK,
                },
            ),
        ]),
        LocationText,
    ));
}

fn spawn_username_text(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn((
        TextBundle::from_section(
            format!("Username: {}", text),
            TextStyle {
                font,
                font_size: 32.0,
                color: Color::BLACK,
            },
        ),
        UsernameText,
    ));
}

fn spawn_balance_text(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn((
        TextBundle::from_section(
            format!("Balance: {}", text),
            TextStyle {
                font,
                font_size: 32.0,
                color: Color::BLACK,
            },
        ),
        BalanceText,
    ));
}

fn spawn_select_block_buttons(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                //padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            //size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            //padding: UiRect::all(Val::Px(10.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),

                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MoveButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Move",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            //size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            //padding: UiRect::all(Val::Px(10.0)),
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    BlockDetailsButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Details",
                        TextStyle {
                            font,
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
