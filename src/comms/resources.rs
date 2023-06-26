use std::collections::HashMap;

use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

use super::{BlockchainBlock, GameBlock};

#[derive(Resource, Clone, Debug, Default, Validate, Deserialize)]
pub struct BlockchainBlockDataFromServer {
    pub blocks: HashMap<String, BlockchainBlock>,
}

#[derive(Resource, Clone, Debug, Default, Validate, Deserialize)]
pub struct GameBlockDataFromServer {
    pub blocks: HashMap<String, GameBlock>,
}

#[derive(Resource, Clone, Debug, Default, Validate, Deserialize)]
pub struct InvoiceDataFromServer {
    pub invoice: String,
    pub expires: DateTime<Utc>,
    pub code: String,
}

#[derive(Resource, Clone, Debug, Default, Validate, Deserialize)]
pub struct InvoiceCheckFromServer {
    pub status: String,
}
