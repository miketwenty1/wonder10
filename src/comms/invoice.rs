use async_channel::{Receiver, Sender};
use bevy::{prelude::*, tasks::IoTaskPool};
use serde::Deserialize;

use crate::{
    comms::resources::InvoiceDataFromServer, scenes::game_play::events::BuyBlockRequest,
    CommsApiState, DisplayInvoiceQr, ServerURL,
};

use super::{
    api_timer::ApiPollingTimer,
    events::{ServerInvoiceDoneIn, ServerInvoiceIn},
    resources::InvoiceCheckFromServer,
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

#[derive(Debug, Clone, Deserialize, Default)]
pub enum InvoiceStatus {
    #[default]
    Pending,
    Completed,
    Expired,
    Error,
}

// impl InvoiceStatus {
//     fn as_str(&self) -> &'static str {
//         match self {
//             InvoiceStatus::Pending => "pending",
//             InvoiceStatus::Completed => "completed",
//             InvoiceStatus::Expired => "expired",
//             InvoiceStatus::Error => "error",
//         }
//     }
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct InvoiceCheckData {
//     status: InvoiceStatus,
// }

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
    mut qr_state: ResMut<NextState<DisplayInvoiceQr>>,
    mut server_event: EventWriter<ServerInvoiceIn>,
    mut invoice_data: ResMut<InvoiceDataFromServer>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        info!("waiting to receive invoice");
        match api_res {
            Ok(r) => {
                info!("response to requesting invoice: {:#?}", r);
                let r_result = serde_json::from_str::<InvoiceDataFromServer>(&r);
                match r_result {
                    Ok(server_data) => {
                        *invoice_data = server_data;
                        api_name_set_state.set(CommsApiState::CheckInvoice);
                        server_event.send(ServerInvoiceIn);
                        qr_state.set(DisplayInvoiceQr::On);
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
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    //mut current_block_server_data: ResMut<BlockchainBlockDataFromServer>,
    mut event: EventWriter<ServerInvoiceDoneIn>,
    //mut game_state: ResMut<NextState<GameState>>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        info!("waiting to receive invoice check");
        match api_res {
            Ok(r) => {
                // info!("received something from invoice check: {}", r);
                let r_result = serde_json::from_str::<InvoiceCheckFromServer>(&r);
                match r_result {
                    Ok(o) => {
                        match o.status.as_str() {
                            "pending" => {
                                info!("pending invoice");
                            }
                            "completed" => {
                                info!("completed invoice");
                                event.send(ServerInvoiceDoneIn);
                                api_name_set_state.set(CommsApiState::Off);
                            }
                            "expired" => {
                                info!("expired invoice");
                                event.send(ServerInvoiceDoneIn);
                                api_name_set_state.set(CommsApiState::Off);
                            }
                            "error" => {
                                info!("error invoice");
                                event.send(ServerInvoiceDoneIn);
                                api_name_set_state.set(CommsApiState::Off);
                            }
                            _ => {
                                info!("Something very bizaare happened picka2");
                                api_name_set_state.set(CommsApiState::Off);
                            }
                        }
                        *invoice_check_res = o;
                    }
                    Err(e) => {
                        info!("requesting check invoice fail: {}", e);
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
