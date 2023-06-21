use crate::{
    scenes::game_play::events::PlayerMove, CommsApiState, GameState, PlayerUsername, ServerURL,
};
use async_channel::{Receiver, Sender};
use bevy::{prelude::*, tasks::IoTaskPool};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::api_timer::ApiPollingTimer;

#[derive(Resource, Clone)]
pub struct SetUsernameChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct RespUsernameSet {
    pub name: String,
}

#[allow(unused_must_use)]
pub fn api_send_username(
    set_username_channel: &Res<SetUsernameChannel>,
    api_server: &Res<ServerURL>,
    player_username: &Res<PlayerUsername>,
) {
    let pool = IoTaskPool::get();
    let cc = set_username_channel.tx.clone();
    let server = api_server.0.to_owned();
    let player = player_username.0.to_owned();
    info!("{}/playername/{}", server, player);
    let _task = pool.spawn(async move {
        //info!("{}/playername/{}", api_sever.0, player_username.0);
        let api_response_text = reqwest::get(format!("{}/comms/playername/{}", server, player))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        cc.try_send(api_response_text);
    });
}

pub fn api_receive_username(
    set_username_channel: ResMut<SetUsernameChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    mut player_username: ResMut<PlayerUsername>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_move_event_writer: EventWriter<PlayerMove>,
) {
    if api_timer.timer.finished() {
        info!("timer finished");
        let api_res = set_username_channel.rx.try_recv();

        match api_res {
            Ok(r) => {
                info!("response to setname: {}", r);
                let r_invoice_result = serde_json::from_str::<RespUsernameSet>(&r);
                match r_invoice_result {
                    Ok(o) => {
                        info!("{:?}", o);
                        player_username.0 = o.name;
                        game_state.set(GameState::Game);
                        player_move_event_writer.send(PlayerMove { block: 0 }); //IF YOU NEED TO SET A CUSTOM START
                        api_name_set_state.set(CommsApiState::Move);
                    }
                    Err(e) => {
                        info!("no new invoice data to get: {}", e);
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
