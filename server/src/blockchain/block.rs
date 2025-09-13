use crate::{
    blockchain::transaction::{MergeVu8, Transaction},
    utils::error::Result,
};
use bincode::{config, encode_to_vec};
use chrono::Utc;
use log::info;
use merkle_cbt::CBMT;
use sha2::{Digest, Sha256};
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
    }

    pub fn calculate_merkle_root(&self) -> Result<Vec<u8>> {
        if self.transactions.is_empty() {
            info!(
                "No transactions in the block to calculate Merkle root. Returning an empty string."
            );
            return String::new();
        }

        let mut transactions = Vec::new();

        for tx in &mut self.transactions {
            transactions.push(tx.calculate_hash()?.as_bytes().to_owned());
        }

        let tree = CBMT::<Vec<u8>, MergeVu8>::build_merkle_tree(&transactions);

        Ok(tree.root())
    }

    pub fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.previous_hash.clone(),
            self.calculate_merkle_root()?,
            self.timestamp,
        );
        let bytes: Vec<u8> = encode_to_vec(&content, config::standard())?;
        Ok(bytes)
    }

    pub fn calculate_hash(&mut self) -> Result<()> {
        let mut hasher = Sha256::new();
        let data = self.prepare_hash_data()?;
        hasher.update(&data);
        self.hash = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        Ok(())
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_hash_ref(&self) -> &String {
        &self.hash
    }

    pub fn get_previous_hash(&self) -> String {
        self.previous_hash.clone()
    }

    pub fn get_previous_hash_ref(&self) -> &String {
        &self.previous_hash
    }

    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    pub fn get_transactions_ref(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}
