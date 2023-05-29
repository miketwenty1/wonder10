use bevy::prelude::*;

use super::ulam::get_25_blocks;
#[derive(Component, Debug)]
pub struct GridLayout;

#[derive(Component, Debug)]
pub struct BlockButton {
    pub color: Color,
    pub paid_color: Color,
    pub grid_offset_x: i32,
    pub grid_offset_y: i32,
    pub height: u32,
}

#[derive(Resource, Clone, Debug)]
pub struct SelectedBlock {
    pub entity: Entity,
    pub height: u32,
}

impl Default for SelectedBlock {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            height: 0,
        }
    }
}

pub fn spawn_blocks(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    size: Size::all(Val::Percent(100.)),
                    //padding: UiRect::all(Val::Px(0.0)),
                    grid_template_columns: vec![
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                    ],
                    grid_template_rows: vec![
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                        GridTrack::min_content(),
                    ],
                    //padding: UiRect::horizontal(Val::Percent(5.0)),
                    ..default()
                },

                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            GridLayout,
        ))
        .with_children(|builder| {
            let origin_vec = get_25_blocks(0);

            for grid_block in origin_vec.iter() {
                builder
                    .spawn(NodeBundle {
                        //background_color: BackgroundColor(*block_color),
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            //grid_column: GridPlacement::span(3),
                            //padding: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_block_button_bundle(
                            builder,
                            font.clone(),
                            grid_block.value,
                            Color::DARK_GRAY,
                            grid_block.grid_x,
                            grid_block.grid_y,
                        );
                    });
            }
        });
}

fn spawn_block_button_bundle(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    block_num: u32,
    block_color: Color,
    grid_offset_x: i32,
    grid_offset_y: i32,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                            size: Size::new(Val::Percent(94.0), Val::Percent(94.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(12.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(block_color),
                        ..default()
                    },
                    BlockButton {
                        color: block_color,
                        paid_color: block_color,
                        grid_offset_x,
                        grid_offset_y,
                        height: block_num,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            block_num.to_string(),
                            TextStyle {
                                font,
                                font_size: 60.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                            },
                        ),
                        //BlockHeightText(block_num.to_string()),
                    ));
                });
        });
}
