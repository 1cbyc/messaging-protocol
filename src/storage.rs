use crate::types::{Message, ClientInfo};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use chrono::Utc;
use tokio::sync::RwLock;
use std::sync::Arc;
use futures;

pub struct Storage {
    messages: Arc<RwLock<HashMap<String, Vec<Message>>>>,
    clients: Arc<RwLock<HashMap<String, ClientInfo>>>,
    data_dir: String,
}

impl Storage {
    pub fn new(data_dir: &str) -> Result<Self> {
        println!("📁 Creating storage in directory: {}", data_dir);
        
        // Create data directory if it doesn't exist
        match fs::create_dir_all(data_dir) {
            Ok(_) => println!("✅ Data directory created/verified"),
            Err(e) => {
                eprintln!("❌ Failed to create data directory: {}", e);
                // Try to continue anyway
            }
        }
        
        let storage = Self {
            messages: Arc::new(RwLock::new(HashMap::new())),
            clients: Arc::new(RwLock::new(HashMap::new())),
            data_dir: data_dir.to_string(),
        };
        println!("✅ Storage struct created");
        
        // Load existing data (ignore errors for now)
        println!("📂 Loading existing data...");
        if let Err(e) = storage.load_data() {
            eprintln!("⚠️ Warning: Failed to load existing data: {}", e);
        } else {
            println!("✅ Data loaded successfully");
        }
        
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
        println!("📝 Storage: Registering client {}", client_id);
        let mut clients = self.clients.write().await;
        println!("📝 Storage: Got write lock");
        let client_info = ClientInfo {
            id: client_id.clone(),
            public_key,
            registered_at: Utc::now(),
            last_seen: Utc::now(),
        };
        clients.insert(client_id, client_info);
        println!("📝 Storage: Client inserted into map");
        
        // Temporarily disable file saving to debug
        println!("📝 Storage: Skipping file save for now");
        // self.save_clients().await?;
        println!("📝 Storage: Registration completed");
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
        println!("🔍 Starting data load...");
        
        // Load messages
        let messages_path = format!("{}/messages.json", self.data_dir);
        println!("📂 Checking messages file: {}", messages_path);
        if Path::new(&messages_path).exists() {
            println!("📖 Loading messages from disk...");
            match fs::read_to_string(&messages_path) {
                Ok(content) => {
                    match serde_json::from_str::<HashMap<String, Vec<Message>>>(&content) {
                        Ok(messages) => {
                            let message_count = messages.len();
                            let mut messages_guard = futures::executor::block_on(self.messages.write());
                            *messages_guard = messages;
                            println!("✅ Messages loaded: {} message groups", message_count);
                        }
                        Err(e) => eprintln!("⚠️ Warning: Failed to parse messages file: {}", e),
                    }
                }
                Err(e) => eprintln!("⚠️ Warning: Failed to read messages file: {}", e),
            }
        } else {
            println!("📝 No existing messages file found");
        }

        // Load clients
        let clients_path = format!("{}/clients.json", self.data_dir);
        println!("📂 Checking clients file: {}", clients_path);
        if Path::new(&clients_path).exists() {
            println!("📖 Loading clients from disk...");
            match fs::read_to_string(&clients_path) {
                Ok(content) => {
                    match serde_json::from_str::<HashMap<String, ClientInfo>>(&content) {
                        Ok(clients) => {
                            let client_count = clients.len();
                            let mut clients_guard = futures::executor::block_on(self.clients.write());
                            *clients_guard = clients;
                            println!("✅ Clients loaded: {} clients", client_count);
                        }
                        Err(e) => eprintln!("⚠️ Warning: Failed to parse clients file: {}", e),
                    }
                }
                Err(e) => eprintln!("⚠️ Warning: Failed to read clients file: {}", e),
            }
        } else {
            println!("📝 No existing clients file found");
        }

        println!("✅ Data load completed");
        Ok(())
    }
} 