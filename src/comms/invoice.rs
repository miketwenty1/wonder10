use async_channel::{Receiver, Sender};
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    comms::resources::InvoiceDataFromServer, scenes::game_play::events::BuyBlockRequest,
    CommsApiState, ServerURL,
};

use super::{
    api_timer::ApiPollingTimer, events::ServerInvoiceIn, resources::InvoiceCheckFromServer,
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
    invoice_data: Res<InvoiceDataFromServer>,
) {
    for buy_block_data in button_event_reader.iter() {
        if invoice_data.invoice.is_empty() {
            info!("requested invoice from buy button");

            let pool = IoTaskPool::get();
            let cc = request_invoice_channel.tx.clone();
            let server = api_server.0.to_owned();

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
        } else {
            info!("already has an invoice in memory! pay that one, or let it expire and request a new one.");
            info!("current invoice to be paid: {:#?}", invoice_data.invoice);
        }
    }
}

pub fn api_receive_invoice(
    channel: ResMut<RequestInvoiceChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    mut server_event: EventWriter<ServerInvoiceIn>,
    mut invoice_data: ResMut<InvoiceDataFromServer>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        info!("waiting to receive invoice details");
        match api_res {
            Ok(r) => {
                info!("response to requesting invoice: {:#?}", r);
                let r_result = serde_json::from_str::<InvoiceDataFromServer>(&r);
                match r_result {
                    Ok(server_data) => {
                        *invoice_data = server_data;
                        api_name_set_state.set(CommsApiState::CheckInvoice);
                        server_event.send(ServerInvoiceIn);
                        //game_state.set(GameState::BlockDetailsOverlay);
                    }
                    Err(e) => {
                        info!("response to invoice creation - fail: {}", e);
                    }
                };
                r
            }
            Err(e) => {
                info!("response to invoice creation: {}", e);
                e.to_string()
            }
        };
    }
}

#[allow(unused_must_use)]
pub fn api_check_invoice(
    channel: Res<CheckInvoiceChannel>,
    api_server: Res<ServerURL>,
    api_timer: Res<ApiPollingTimer>,
    invoice_res: Res<InvoiceDataFromServer>,
    //mut details_button_event_reader: EventReader<BlockDetailClick>,
) {
    if api_timer.timer.finished() {
        info!("check for invoice status");

        let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.0.to_owned();
        let code = invoice_res.code.to_owned();
        let _task = pool.spawn(async move {
            let api_response_text = reqwest::get(format!("{}/comms/checkinvoice/{}", server, code))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            cc.try_send(api_response_text);
        });
    }
}

pub fn api_receive_invoice_check(
    channel: ResMut<CheckInvoiceChannel>,
    mut invoice_check_res: ResMut<InvoiceCheckFromServer>,
    api_timer: Res<ApiPollingTimer>,
    //mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    //mut current_block_server_data: ResMut<BlockchainBlockDataFromServer>,
    //mut server_block_in_event: EventWriter<ServerBlockchainBlockIn>,
    //mut game_state: ResMut<NextState<GameState>>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        info!("waiting to receive block details");
        match api_res {
            Ok(r) => {
                info!("response to requesting block details: {}", r);
                let r_result = serde_json::from_str::<InvoiceCheckFromServer>(&r);
                match r_result {
                    Ok(o) => {
                        info!("{:?}", o);
                        *invoice_check_res = o;

                        // api_name_set_state.set(CommsApiState::Off);
                        // server_block_in_event.send(ServerBlockchainBlockIn);
                        // game_state.set(GameState::BlockDetailsOverlay);
                    }
                    Err(e) => {
                        info!("requesting block details fail: {}", e);
                    }
                };
                r
            }
            Err(e) => {
                info!("response to check invoice: {}", e);
                e.to_string()
            }
        };
    }
}
