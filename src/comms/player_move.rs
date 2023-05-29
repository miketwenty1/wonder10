use crate::{
    comms::BlockDataFromServer,
    scenes::game_play::{
        blocks_grid::BlockButton,
        events::{PlayerMove, ServerBocksIn},
    },
    CommsApiState, ServerURL,
};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::api_timer::ApiPollingTimer;

#[derive(Resource, Clone)]
pub struct PlayerMovementChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[allow(unused_must_use)]
pub fn api_send_player_move(
    set_player_move_channel: Res<PlayerMovementChannel>,
    api_server: Res<ServerURL>,
    mut player_move_event_reader: EventReader<PlayerMove>,
) {
    for event in player_move_event_reader.iter() {
        let pool = IoTaskPool::get();
        let cc = set_player_move_channel.tx.clone();
        let server = api_server.0.to_owned();
        info!(
            "hey this working here buddy? -> {}/comms/move/{}",
            server, event.block
        );
        let block_copy = event.block;
        let _task = pool.spawn(async move {
            let api_response_text = reqwest::get(format!("{}/comms/move/{}", server, block_copy))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            cc.try_send(api_response_text);
        });
    }
}

pub fn api_receive_player_movement(
    set_player_move_channel: ResMut<PlayerMovementChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    mut current_block_server_data: ResMut<BlockDataFromServer>,
    mut server_block_in_event: EventWriter<ServerBocksIn>,
) {
    if api_timer.timer.finished() {
        let api_res = set_player_move_channel.rx.try_recv();

        match api_res {
            Ok(r) => {
                info!("response to move player: {}", r);
                let r_invoice_result = serde_json::from_str::<BlockDataFromServer>(&r);
                match r_invoice_result {
                    Ok(server_block_data) => {
                        info!("{:?}", server_block_data);
                        *current_block_server_data = server_block_data;

                        api_name_set_state.set(CommsApiState::Off);
                        server_block_in_event.send(ServerBocksIn);
                    }
                    Err(e) => {
                        info!("player move fail: {}", e);
                    }
                };
                r
            }
            Err(e) => {
                info!("response to setname: {}", e);
                e.to_string()
            }
        };
    }
}

pub fn update_blocks_from_server_on_move(
    mut server_block_in: EventReader<ServerBocksIn>,
    mut block_query: Query<(Entity, &mut BlockButton)>,
    mut color_query: Query<&mut BackgroundColor>,
    current_block_server_data: Res<BlockDataFromServer>,
) {
    //info!("made it here 1");
    let block_map = &current_block_server_data.blocks;

    for _event in server_block_in.iter() {
        //info!("made it here 2");
        for (id, mut block_button) in block_query.iter_mut() {
            //info!("made it here 3");
            let bheight = block_button.height.to_string();
            let a = block_map.get(&bheight);

            match a {
                Some(s) => {
                    //info!("made it here 4");
                    let c = Color::hex(s.color.clone()).unwrap();
                    color_query.get_mut(id).unwrap().0 = c;
                    block_button.color = c;
                    block_button.paid_color = c;
                }
                None => {
                    info!("no match for height {}", bheight);
                    color_query.get_mut(id).unwrap().0 = Color::DARK_GRAY;
                    block_button.color = Color::DARK_GRAY;
                    block_button.paid_color = Color::DARK_GRAY;
                }
            }
        }
    }
}
