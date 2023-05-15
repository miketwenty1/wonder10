use crate::PlayerUsername;

use super::blocks_grid::spawn_blocks;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GameLayout;

#[derive(Component, Debug)]
pub struct BlockButton(Color);

#[derive(Component, Debug)]
pub struct LocationText;

#[derive(Component, Debug)]
pub struct UsernameText;

#[derive(Component, Debug)]
pub struct BalanceText;

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_username: Res<PlayerUsername>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn((
            NodeBundle {
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
        });
}

fn spawn_location_text(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn((
        TextBundle::from_section(
            format!("Block Location: {}", text),
            TextStyle {
                font,
                font_size: 32.0,
                color: Color::BLACK,
            },
        ),
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
