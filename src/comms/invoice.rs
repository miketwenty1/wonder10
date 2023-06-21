use async_channel::{Receiver, Sender};
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    scenes::game_play::events::{BlockDetailClick, BuyBlockRequest},
    ServerURL,
};

#[derive(Resource, Clone)]
pub struct RequestInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct CheckInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[allow(unused_must_use)]
pub fn api_request_invoice(
    request_invoice_channel: Res<RequestInvoiceChannel>,
    api_server: Res<ServerURL>,
    mut button_event_reader: EventReader<BuyBlockRequest>,
) {
    for buy_block_data in button_event_reader.iter() {
        info!("event received from buy block clicked");

        let pool = IoTaskPool::get();
        let cc = request_invoice_channel.tx.clone();
        let server = api_server.0.to_owned();

        //let a = buy_block_data.to_owned();
        //let copy = buy_block_data.to_owned();
        //let copy2 = copy;

        let b = BuyBlockRequest {
            height: buy_block_data.height,
            owner: buy_block_data.owner.clone(),
            refund_address: buy_block_data.refund_address.clone(),
            color: buy_block_data.color.clone(),
            message: buy_block_data.message.clone(),
        };
        let url = format!("{}/comms/invoice/block", server);
        info!("url: {}", url);
        let _task = pool.spawn(async move {
            let api_response_text = reqwest::Client::new()
                .post(url)
                .header("Content-Type", "application/json")
                .json(&b)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            cc.try_send(api_response_text);
        });
    }
}

pub fn api_receive_invoice(// channel: ResMut<RequestInvoiceChannel>,
    // api_timer: Res<ApiPollingTimer>,
    // mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    // mut current_block_server_data: ResMut<BlockchainBlockDataFromServer>,
    // mut server_block_in_event: EventWriter<ServerBlockchainBlockIn>,
    // mut game_state: ResMut<NextState<GameState>>,
) {
    // if api_timer.timer.finished() {
    //     let api_res = channel.rx.try_recv();

    //     info!("waiting to receive block details");
    //     match api_res {
    //         Ok(r) => {
    //             info!("response to requesting block details: {}", r);
    //             let r_result = serde_json::from_str::<BlockchainBlockDataFromServer>(&r);
    //             match r_result {
    //                 Ok(server_block_data) => {
    //                     info!("{:?}", server_block_data);
    //                     *current_block_server_data = server_block_data;

    //                     api_name_set_state.set(CommsApiState::Off);
    //                     server_block_in_event.send(ServerBlockchainBlockIn);
    //                     game_state.set(GameState::BlockDetailsOverlay);
    //                 }
    //                 Err(e) => {
    //                     info!("requesting block details fail: {}", e);
    //                 }
    //             };
    //             r
    //         }
    //         Err(e) => {
    //             info!("response to setname: {}", e);
    //             e.to_string()
    //         }
    //     };
    // }
}

#[allow(unused_must_use)]
pub fn api_check_invoice(// block_details_channel: Res<BlockDetailsChannel>,
   // api_server: Res<ServerURL>,
  //  mut details_button_event_reader: EventReader<BlockDetailClick>,
) {
    // for event in details_button_event_reader.iter() {
    //     info!("event received from blockdetails clicked {}", event.block);

    //     let pool = IoTaskPool::get();
    //     let cc = block_details_channel.tx.clone();
    //     let server = api_server.0.to_owned();

    //     let block_copy = event.block;
    //     let _task = pool.spawn(async move {
    //         let api_response_text =
    //             reqwest::get(format!("{}/comms/blockdetails/{}", server, block_copy))
    //                 .await
    //                 .unwrap()
    //                 .text()
    //                 .await
    //                 .unwrap();
    //         cc.try_send(api_response_text);
    //     });
    // }
}

pub fn api_receive_invoice_check(// channel: ResMut<BlockDetailsChannel>,
    // api_timer: Res<ApiPollingTimer>,
    // mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    // mut current_block_server_data: ResMut<BlockchainBlockDataFromServer>,
    // mut server_block_in_event: EventWriter<ServerBlockchainBlockIn>,
    // mut game_state: ResMut<NextState<GameState>>,
) {
    // if api_timer.timer.finished() {
    //     let api_res = channel.rx.try_recv();

    //     info!("waiting to receive block details");
    //     match api_res {
    //         Ok(r) => {
    //             info!("response to requesting block details: {}", r);
    //             let r_result = serde_json::from_str::<BlockchainBlockDataFromServer>(&r);
    //             match r_result {
    //                 Ok(server_block_data) => {
    //                     info!("{:?}", server_block_data);
    //                     *current_block_server_data = server_block_data;

    //                     api_name_set_state.set(CommsApiState::Off);
    //                     server_block_in_event.send(ServerBlockchainBlockIn);
    //                     game_state.set(GameState::BlockDetailsOverlay);
    //                 }
    //                 Err(e) => {
    //                     info!("requesting block details fail: {}", e);
    //                 }
    //             };
    //             r
    //         }
    //         Err(e) => {
    //             info!("response to setname: {}", e);
    //             e.to_string()
    //         }
    //     };
    // }
}
