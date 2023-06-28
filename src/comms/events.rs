use bevy::prelude::Event;

#[derive(Event)]
pub struct ServerBlockchainBlockIn;

#[derive(Event)]
pub struct ServerGameBocksIn;
#[derive(Event)]
pub struct ServerInvoiceIn;
#[derive(Event)]
pub struct ServerInvoiceDoneIn;
