use serde::{Deserialize, Serialize};

pub struct BlockButtonSelected {
    pub block: u32,
}

pub struct PlayerMove {
    pub block: u32,
}

pub struct BlockDetailClick {
    pub block: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuyBlockRequest {
    pub height: u32,
    pub owner: String,
    pub refund_address: String,
    pub color: String,
    pub message: String,
}
