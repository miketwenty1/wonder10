use bevy::prelude::*;

use super::ulam::get_25_blocks;
#[derive(Component, Debug)]
pub struct GameLayout;

#[derive(Component, Debug)]
pub struct BlockButton(Color);

#[derive(Component, Debug)]
pub struct BlockHeightText(usize);

// const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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
            GameLayout,
        ))
        .with_children(|builder| {
            // let test_vec = [
            //     "16", "15", "14", "13", "12", "17", "4", "3", "2", "11", "18", "5", "0", "1", "10",
            //     "19", "6", "7", "8", "9", "20", "21", "22", "23", "24",
            // ];
            let origin_vec = get_25_blocks(0);

            let test_color_vec = [
                Color::AZURE,
                Color::BEIGE,
                Color::BISQUE,
                Color::FUCHSIA,
                Color::BLUE,
                Color::CRIMSON,
                Color::CYAN,
                Color::DARK_GRAY,
                Color::DARK_GREEN,
                Color::GOLD,
                Color::GREEN,
                Color::INDIGO,
                Color::LIME_GREEN,
                Color::MAROON,
                Color::MIDNIGHT_BLUE,
                Color::NAVY,
                Color::OLIVE,
                Color::ORANGE,
                Color::ORANGE_RED,
                Color::PINK,
                Color::PURPLE,
                Color::RED,
                Color::SALMON,
                Color::TEAL,
                Color::TOMATO,
            ];

            for (block_num, block_color) in origin_vec.iter().zip(test_color_vec.iter()) {
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
                        spawn_block_button_bundle(builder, font.clone(), block_num, *block_color);
                    });
            }
        });
}

fn spawn_block_button_bundle(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    block_num: &str,
    block_color: Color,
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
                    BlockButton(block_color),
                ))
                .with_children(|parent| {
                    // crash if this fails
                    let number = block_num.parse::<usize>().unwrap();
                    parent.spawn((
                        TextBundle::from_section(
                            block_num,
                            TextStyle {
                                font,
                                font_size: 60.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                            },
                        ),
                        BlockHeightText(number),
                    ));
                });
        });
}

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &BlockButton, &Children),
        (Changed<Interaction>, With<BlockButton>),
    >,
    block_text_query: Query<&BlockHeightText>,
) {
    for (interaction, mut color, block_comp, children) in &mut interaction_query {
        let block_number = block_text_query.get(children[0]).unwrap().0;
        match *interaction {
            Interaction::Clicked => {
                info!("clicked block {}", block_number);
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Buy?".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = BackgroundColor(block_comp.0);
            }
        }
    }
}
