use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub encrypted: bool,
    pub signature: Option<String>, // Store as hex string
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub id: String,
    pub public_key: String,
    pub registered_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerCommand {
    Register { client_id: String, public_key: String },
    Send { 
        sender_id: String, 
        recipient_id: String, 
        encrypted_content: String,
        signature: String,
        message_id: String,
    },
    GetMessages { client_id: String },
    GetClients,
    Heartbeat { client_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerResponse {
    Registered { server_public_key: String },
    MessageSent { message_id: String },
    MessageReceived { message: Message },
    ClientList { clients: Vec<String> },
    Error { message: String },
    Ok,
}

impl Message {
    pub fn new(sender_id: String, recipient_id: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sender_id,
            recipient_id,
            content,
            timestamp: Utc::now(),
            encrypted: true,
            signature: None,
        }
    }
} 