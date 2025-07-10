use crate::types::{Message, ClientInfo};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use chrono::Utc;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct Storage {
    messages: Arc<RwLock<HashMap<String, Vec<Message>>>>,
    clients: Arc<RwLock<HashMap<String, ClientInfo>>>,
    data_dir: String,
}

impl Storage {
    pub fn new(data_dir: &str) -> Result<Self> {
        // Create data directory if it doesn't exist
        fs::create_dir_all(data_dir)?;
        
        let storage = Self {
            messages: Arc::new(RwLock::new(HashMap::new())),
            clients: Arc::new(RwLock::new(HashMap::new())),
            data_dir: data_dir.to_string(),
        };
        
        // Load existing data
        storage.load_data()?;
        
        Ok(storage)
    }

    pub async fn add_message(&self, message: Message) -> Result<()> {
        let mut messages = self.messages.write().await;
        let recipient_messages = messages.entry(message.recipient_id.clone()).or_insert_with(Vec::new);
        recipient_messages.push(message);
        
        // Save to disk
        self.save_messages().await?;
        Ok(())
    }

    pub async fn get_messages_for_client(&self, client_id: &str) -> Result<Vec<Message>> {
        let messages = self.messages.read().await;
        Ok(messages.get(client_id).cloned().unwrap_or_default())
    }

    pub async fn register_client(&self, client_id: String, public_key: String) -> Result<()> {
        let mut clients = self.clients.write().await;
        let client_info = ClientInfo {
            id: client_id.clone(),
            public_key,
            registered_at: Utc::now(),
            last_seen: Utc::now(),
        };
        clients.insert(client_id, client_info);
        
        // Save to disk
        self.save_clients().await?;
        Ok(())
    }

    pub async fn update_client_last_seen(&self, client_id: &str) -> Result<()> {
        let mut clients = self.clients.write().await;
        if let Some(client_info) = clients.get_mut(client_id) {
            client_info.last_seen = Utc::now();
        }
        
        // Save to disk
        self.save_clients().await?;
        Ok(())
    }

    pub async fn get_client_info(&self, client_id: &str) -> Option<ClientInfo> {
        let clients = self.clients.read().await;
        clients.get(client_id).cloned()
    }

    pub async fn get_all_clients(&self) -> Vec<String> {
        let clients = self.clients.read().await;
        clients.keys().cloned().collect()
    }

    async fn save_messages(&self) -> Result<()> {
        let messages = self.messages.read().await;
        let messages_path = format!("{}/messages.json", self.data_dir);
        let json = serde_json::to_string_pretty(&*messages)?;
        fs::write(messages_path, json)?;
        Ok(())
    }

    async fn save_clients(&self) -> Result<()> {
        let clients = self.clients.read().await;
        let clients_path = format!("{}/clients.json", self.data_dir);
        let json = serde_json::to_string_pretty(&*clients)?;
        fs::write(clients_path, json)?;
        Ok(())
    }

    fn load_data(&self) -> Result<()> {
        // Load messages
        let messages_path = format!("{}/messages.json", self.data_dir);
        if Path::new(&messages_path).exists() {
            let content = fs::read_to_string(&messages_path)?;
            let messages: HashMap<String, Vec<Message>> = serde_json::from_str(&content)?;
            let mut messages_guard = futures::executor::block_on(self.messages.write());
            *messages_guard = messages;
        }

        // Load clients
        let clients_path = format!("{}/clients.json", self.data_dir);
        if Path::new(&clients_path).exists() {
            let content = fs::read_to_string(&clients_path)?;
            let clients: HashMap<String, ClientInfo> = serde_json::from_str(&content)?;
            let mut clients_guard = futures::executor::block_on(self.clients.write());
            *clients_guard = clients;
        }

        Ok(())
    }
} 