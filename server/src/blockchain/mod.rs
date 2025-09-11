use crate::network::{Network, Node, NodeRole};
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

pub mod block;
pub mod network;
pub mod transaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub current_block: Option<Block>,
    pub network: Network,
}

impl Blockchain {
    pub fn new(authority: String) -> Self {
        Self {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            current_block: None,
            network: Network::new(authority),
        }
    }
}
