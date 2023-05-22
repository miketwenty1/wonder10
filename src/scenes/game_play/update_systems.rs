use bevy::prelude::*;
use ulam::{calc_coord::calc_xy, value_of_xy};

use crate::scenes::game_play::{PRESSED_BUTTON, SELECTED_BUTTON};

use super::{
    blocks_grid::{BlockButton, SelectedBlock},
    events::{BlockButtonSelected, PlayerMove},
    HOVERED_BUTTON,
};

pub fn update_listen_for_player_move(
    //mut block_selected_event_reader: EventReader<BlockButtonSelected>,
    mut player_move_event_reader: EventReader<PlayerMove>,
    mut block_query: Query<(Entity, &mut BlockButton, &Children), With<BlockButton>>,
    mut button_text_query: Query<&mut Text>,
    mut color_query: Query<&mut BackgroundColor>,
    selected_block: ResMut<SelectedBlock>,
) {
    for event in player_move_event_reader.iter() {
        let (x, y) = calc_xy(event.block);
        for (id, mut block_button, children) in block_query.iter_mut() {
            let mut text = button_text_query.get_mut(children[0]).unwrap();
            let new_block_x = block_button.grid_offset_x + x;
            let new_block_y = block_button.grid_offset_y + y;
            let new_block_val = value_of_xy(new_block_x, new_block_y);
            block_button.height = new_block_val;
            text.sections[0].value = new_block_val.to_string();

            if selected_block.entity != id {
                color_query.get_mut(id).unwrap().0 = block_button.paid_color;
            }
        }
        info!("Move detected")
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

#[allow(clippy::type_complexity)]
pub fn button_interaction_system(
    mut click_event_writer: EventWriter<BlockButtonSelected>,
    mut player_move_event_writer: EventWriter<PlayerMove>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BlockButton),
        (Changed<Interaction>, With<BlockButton>),
    >,
    //mut color_query: Query<&mut BackgroundColor>,
    mut selected_block: ResMut<SelectedBlock>,
) {
    //button_entity,
    for (button_entity, interaction, mut bg_color, block_comp) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                //info!("clicked block {}", block_comp.value);
                if selected_block.entity == button_entity {
                    selected_block.entity = Entity::PLACEHOLDER;
                    selected_block.height = 0;

                    player_move_event_writer.send(PlayerMove {
                        block: block_comp.height,
                    });
                } else {
                    selected_block.entity = button_entity;
                    selected_block.height = block_comp.height;
                    click_event_writer.send(BlockButtonSelected {
                        block: block_comp.height,
                    });
                }
                *bg_color = PRESSED_BUTTON.into();
                // *visibility = Visibility::Hidden;
                // block_comp.color = selected_color;
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
