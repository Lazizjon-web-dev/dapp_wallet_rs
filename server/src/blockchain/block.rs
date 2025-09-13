use crate::{
    blockchain::transaction::{MergeVu8, Transaction},
    utils::error::Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub authority_signature: String,
    pub validator_signatures: Vec<String>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        Self {
            hash: String::new(),
            previous_hash,
            timestamp,
            transactions,
            authority_signature: String::new(),
            validator_signatures: Vec::new(),
        }
    };
}
