use crate::{
    comms::BlockchainBlockDataFromServer, scenes::game_play::events::ServerBlockchainBlocksIn,
};
use bevy::prelude::*;

pub fn display_blockchain_block_details(
    mut server_block_in: EventReader<ServerBlockchainBlocksIn>,
    current_block_server_data: Res<BlockchainBlockDataFromServer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let block_map = &current_block_server_data.blocks;

    for _event in server_block_in.iter() {
        info!("WE WILL NOW WANT TO DISPLAY BLOCK INFO");
        info!("TEST:::: {:#?}", block_map);

        //let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        // Top-level grid (app frame)
        let ent = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute, // Needed to display separately from HUD.
                        display: Display::Flex,                // Hidden by Default
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Style::DEFAULT
                    },
                    z_index: ZIndex::Local(1), // See Ref. 1
                    ..default()
                },
                //PauseMenu {},
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(400.0), Val::Px(400.0)),
                            gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                            ..Style::DEFAULT
                        },
                        background_color: Color::rgba(0.25, 0.25, 0.25, 0.8).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Title
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "BLOCK HEIGHT",
                                    get_title_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                        // Resume Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Style::DEFAULT
                                    },
                                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..default()
                                },
                                //ResumeButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Buy Block for XXX sats",
                                            get_button_text_style(&asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                        // Main Menu Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Style::DEFAULT
                                    },
                                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..default()
                                },
                                //MainMenuButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Back",
                                            get_button_text_style(&asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                        // Quit Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Style::DEFAULT
                                    },
                                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..default()
                                },
                                //QuitButton {},
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            "Back",
                                            get_button_text_style(&asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    });
            })
            .id();
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}
