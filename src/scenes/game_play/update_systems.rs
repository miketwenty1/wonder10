use bevy::prelude::*;
use ulam::{calc_coord::calc_xy, value_of_xy};

use crate::{
    comms::player_move::PlayerMovementChannel,
    scenes::game_play::{NORMAL_BUTTON, PRESSED_BUTTON, SELECTED_BUTTON},
    CommsApiState, PlayerLocation,
};

use super::{
    blocks_grid::{BlockButton, SelectedBlock},
    events::{BlockButtonSelected, BlockDetailClick, PlayerMove},
    game_layout::{BlockDetailsButton, LocationText},
    HOVERED_BUTTON,
};

pub fn update_listen_for_player_move(
    //mut block_selected_event_reader: EventReader<BlockButtonSelected>,
    mut player_move_event_reader: EventReader<PlayerMove>,
    mut block_query: Query<
        (Entity, &mut BlockButton, &Children, &mut Visibility),
        With<BlockButton>,
    >,
    mut button_text_query: Query<&mut Text>,
    mut color_query: Query<&mut BackgroundColor>,
    selected_block: ResMut<SelectedBlock>,
) {
    for event in player_move_event_reader.iter() {
        let (x, y) = calc_xy(event.block);
        for (id, mut block_button, children, mut visibility) in block_query.iter_mut() {
            let mut text = button_text_query.get_mut(children[0]).unwrap();
            let new_block_x = block_button.grid_offset_x + x;
            let new_block_y = block_button.grid_offset_y + y;
            let new_block_val = value_of_xy(new_block_x, new_block_y);
            block_button.height = new_block_val;

            let fontsize: f32 = match new_block_val.to_string().len() {
                1 => 50.0,
                2 => 40.0,
                3 => 30.0,
                4 => 25.0,
                5 => 23.0,
                6 => 20.0,
                _ => 17.0,
            };

            text.sections[0].style.font_size = fontsize;
            text.sections[0].value = new_block_val.to_string();

            if new_block_val > 800_000 {
                //TODO
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
            }

            if selected_block.entity != id {
                color_query.get_mut(id).unwrap().0 = block_button.paid_color;
            }
        }
        //info!("Move detected")
    }
}

pub fn update_listen_for_player_select(
    mut block_selected_event_reader: EventReader<BlockButtonSelected>,
    mut block_query: Query<(Entity, &mut BlockButton), With<BlockButton>>,
    mut color_query: Query<&mut BackgroundColor>,
    selected_block: ResMut<SelectedBlock>,
) {
    for _event in block_selected_event_reader.iter() {
        for (id, block_button) in block_query.iter_mut() {
            if selected_block.entity != id {
                color_query.get_mut(id).unwrap().0 = block_button.paid_color;
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn button_interaction_system(
    mut click_event_writer: EventWriter<BlockButtonSelected>,
    mut player_move_event_writer: EventWriter<PlayerMove>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BlockButton),
        (Changed<Interaction>, With<BlockButton>),
    >,
    //mut color_query: Query<&mut BackgroundColor>,
    mut selected_block: ResMut<SelectedBlock>,
    mut player_location: ResMut<PlayerLocation>,
    mut location_text_query: Query<&mut Text, With<LocationText>>,
    mut api_state: ResMut<NextState<CommsApiState>>,
    set_player_move_channel: Res<PlayerMovementChannel>,
) {
    //button_entity,
    for (button_entity, interaction, mut bg_color, block_comp) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //info!("clicked block {}", block_comp.value);
                if selected_block.entity == button_entity {
                    selected_block.entity = Entity::PLACEHOLDER; // used to make it where no block is highlighted after a doubleclick
                                                                 //THis is messing stuff up ... selected_block.height = 999_999_999; // this done so no block is highlighted if you hover your mouse over it.

                    if set_player_move_channel.tx.len() < 2 {
                        info!(
                            "rx len {}, tx len {}",
                            set_player_move_channel.rx.len(),
                            set_player_move_channel.tx.len()
                        );
                        player_move_event_writer.send(PlayerMove {
                            block: block_comp.height,
                        });

                        location_text_query.get_single_mut().unwrap().sections[1].value =
                            block_comp.height.to_string();
                        player_location.0 = block_comp.height;
                        api_state.set(CommsApiState::Move);
                    }
                } else {
                    selected_block.entity = button_entity;
                    selected_block.height = block_comp.height;
                    click_event_writer.send(BlockButtonSelected {
                        block: block_comp.height,
                    });
                }
                *bg_color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Buy?".to_string();
                *bg_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                if selected_block.height == block_comp.height {
                    *bg_color = BackgroundColor(SELECTED_BUTTON);
                } else {
                    *bg_color = BackgroundColor(block_comp.paid_color);
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn button_block_details(
    mut event: EventWriter<BlockDetailClick>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BlockDetailsButton>),
    >,
    //mut color_query: Query<&mut BackgroundColor>,
    selected_block: ResMut<SelectedBlock>,
    mut api_state: ResMut<NextState<CommsApiState>>,
) {
    //button_entity,
    for (interaction, mut bg_color) in &mut interaction_query {
        let details_block_height = if selected_block.height > 1_000_000 {
            0
        } else {
            selected_block.height
        };
        match *interaction {
            Interaction::Pressed => {
                info!("blockdetails clicked");

                info!(
                    "block height {}, selected: {}",
                    details_block_height, selected_block.height
                );
                event.send(BlockDetailClick {
                    block: details_block_height,
                });

                api_state.set(CommsApiState::BlockDetails);
                *bg_color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Buy?".to_string();
                *bg_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *bg_color = BackgroundColor(NORMAL_BUTTON);
            }
        }
    }
}
