use crate::{scenes::game_play::events::PlayerMove, CommsApiState, ServerURL};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

use super::{
    api_timer::ApiPollingTimer, events::ServerGameBocksIn, resources::GameBlockDataFromServer,
};

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
    for event in player_move_event_reader.read() {
        let pool = IoTaskPool::get();
        let cc = set_player_move_channel.tx.clone();
        let server = api_server.0.to_owned();

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
    mut current_block_server_data: ResMut<GameBlockDataFromServer>,
    mut server_block_in_event: EventWriter<ServerGameBocksIn>,
) {
    if api_timer.timer.finished() && !set_player_move_channel.rx.is_empty() {
        let api_res = set_player_move_channel.rx.try_recv();

        match api_res {
            Ok(r) => {
                //info!("response to move player: {}", r);
                let r_invoice_result = serde_json::from_str::<GameBlockDataFromServer>(&r);
                match r_invoice_result {
                    Ok(server_block_data) => {
                        //info!("{:?}", server_block_data);
                        *current_block_server_data = server_block_data;
                        if set_player_move_channel.rx.is_empty() {
                            api_name_set_state.set(CommsApiState::Off);
                        }

                        server_block_in_event.send(ServerGameBocksIn);
                    }
                    Err(e) => {
                        info!("player move fail: {}", e);
                    }
                };
                r
            }
            Err(e) => {
                info!("response to moveplayer: {}", e);
                if set_player_move_channel.rx.is_empty() {
                    api_name_set_state.set(CommsApiState::Off);
                }
                e.to_string()
            }
        };
    }
}
