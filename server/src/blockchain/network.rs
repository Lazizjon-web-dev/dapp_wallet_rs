use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type Address = String;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NodeRole {
    Authority,
    Validator,
    Reader,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub authority: Node,
    pub validators: Vec<Node>,
    pub readers: Vec<Node>,
    pub allowed_nodes: HashSet<String>,
}

impl Network {
    pub fn new(authority: String) -> Self {
        let authority_node = Node::new(authority, NodeRole::Authority);
        Self {
            authority: authority_node,
            validators: Vec::new(),
            readers: Vec::new(),
            allowed_nodes: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, address: String, role: NodeRole) {
        let node = Node::new(address, role);
        match node.role {
            NodeRole::Authority => {
                // Only one authority allowed
                println!("Authority node already exists.");
            }
            NodeRole::Validator => {
                self.allowed_nodes.insert(node.address.clone());
                self.validators.push(node);
            }
            NodeRole::Reader => {
                self.allowed_nodes.insert(node.address.clone());
                self.readers.push(node);
            }
        }
    }

    pub fn change_authority(&mut self, new_authority: String) {
        let authority_node = Node::new(new_authority, NodeRole::Authority);
        self.authority = authority_node;
    }

    pub fn is_node_allowed(&self, address: &String) -> bool {
        self.allowed_nodes.contains(address)
    }
}