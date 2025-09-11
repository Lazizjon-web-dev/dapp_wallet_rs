use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type Address = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub address: Address,
    pub public_key: String,
    pub role: NodeRole,
}

impl Node {
    pub fn new(address: String, role: NodeRole) -> Self {
        // In a real implementation, you would generate a public/private key pair here.
        // For simplicity, we use a placeholder for the public key.
        let public_key = format!("public_key_of_{}", address);
        Self {
            address,
            public_key,
            role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NodeRole {
    Authority,
    Validator,
    Reader,
}
