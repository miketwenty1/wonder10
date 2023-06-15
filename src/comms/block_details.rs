use crate::{
    comms::BlockchainBlockDataFromServer,
    scenes::game_play::events::{BlockDetailClick, ServerBlockchainBlocksIn},
    CommsApiState, ServerURL,
};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

use super::api_timer::ApiPollingTimer;

#[derive(Resource, Clone)]
pub struct BlockDetailsChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[allow(unused_must_use)]
pub fn api_request_blockchain_block(
    block_details_channel: Res<BlockDetailsChannel>,
    api_server: Res<ServerURL>,
    mut details_button_event_reader: EventReader<BlockDetailClick>,
) {
    for event in details_button_event_reader.iter() {
        info!("event received from blockdetails clicked {}", event.block);

        let pool = IoTaskPool::get();
        let cc = block_details_channel.tx.clone();
        let server = api_server.0.to_owned();

        let block_copy = event.block;
        let _task = pool.spawn(async move {
            let api_response_text =
                reqwest::get(format!("{}/comms/blockdetails/{}", server, block_copy))
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
            cc.try_send(api_response_text);
        });
    }
}

pub fn api_receive_blockchain_block(
    channel: ResMut<BlockDetailsChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    mut current_block_server_data: ResMut<BlockchainBlockDataFromServer>,
    mut server_block_in_event: EventWriter<ServerBlockchainBlocksIn>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        info!("waiting to receive block details");
        match api_res {
            Ok(r) => {
                info!("response to requesting block details: {}", r);
                let r_invoice_result = serde_json::from_str::<BlockchainBlockDataFromServer>(&r);
                match r_invoice_result {
                    Ok(server_block_data) => {
                        info!("{:?}", server_block_data);
                        *current_block_server_data = server_block_data;

                        api_name_set_state.set(CommsApiState::Off);
                        server_block_in_event.send(ServerBlockchainBlocksIn);
                    }
                    Err(e) => {
                        info!("requesting block details fail: {}", e);
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
