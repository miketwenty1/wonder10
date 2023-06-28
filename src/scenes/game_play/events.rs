use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Event)]
pub struct BlockButtonSelected {
    pub block: u32,
}

#[derive(Event)]
pub struct PlayerMove {
    pub block: u32,
}

#[derive(Event)]
pub struct BlockDetailClick {
    pub block: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Event)]
pub struct BuyBlockRequest {
    pub height: u32,
    pub owner: String,
    pub refund_address: String,
    pub color: String,
    pub message: String,
}
