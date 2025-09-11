use crate::transaction::Transaction;
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
