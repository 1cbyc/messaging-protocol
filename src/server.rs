mod types;
mod crypto;
mod storage;

use crate::types::{ServerCommand, ServerResponse, Message};
use crate::crypto::CryptoManager;
use crate::storage::Storage;
use ed25519_dalek::{PublicKey, Signature};
use hex;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use anyhow::{Result, anyhow};
use colored::*;
use log::{info, error};

struct Server {
    crypto: CryptoManager,
    storage: Storage,
    active_connections: Arc<Mutex<HashMap<String, tokio::net::TcpStream>>>,
}

impl Server {
    fn new() -> Result<Self> {
        let crypto = CryptoManager::new();
        let storage = Storage::new("./data")?;
        
        Ok(Server {
            crypto,
            storage,
            active_connections: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    async fn run(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("🚀 Secure messaging server listening on {}", addr);
        println!("📊 Server public key: {}", hex::encode(self.crypto.get_ed25519_public_key().as_bytes()));

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("📱 New connection from {}", addr);
            
            let server = Arc::new(self.clone());
            tokio::spawn(async move {
                if let Err(e) = server.handle_connection(socket).await {
                    eprintln!("❌ Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(&self, mut socket: tokio::net::TcpStream) -> Result<()> {
        let mut buf = [0; 4096];
        
        loop {
            let n = match socket.read(&mut buf).await {
                Ok(n) if n == 0 => {
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    error!("❌ Read error: {}", e);
                    break;
                }
            };

            let request = String::from_utf8_lossy(&buf[..n]);
            
            let response = match self.process_request(&request).await {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("❌ Error processing request: {}", e);
                    ServerResponse::Error { message: e.to_string() }
                }
            };
            
            let response_json = serde_json::to_string(&response)?;
            socket.write_all(response_json.as_bytes()).await?;
        }
        
        Ok(())
    }

    async fn process_request(&self, request: &str) -> Result<ServerResponse> {
        let command: ServerCommand = serde_json::from_str(request)
            .map_err(|e| anyhow!("Invalid JSON: {}", e))?;

        match command {
            ServerCommand::Register { client_id, public_key } => {
                match self.storage.register_client(client_id.clone(), public_key).await {
                    Ok(_) => {
                        let response = ServerResponse::Registered {
                            server_public_key: hex::encode(self.crypto.get_ed25519_public_key().as_bytes()),
                        };
                        Ok(response)
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to register client: {}", e);
                        Err(e)
                    }
                }
            }

            ServerCommand::Send { sender_id, recipient_id, encrypted_content, signature, message_id } => {
                info!("📤 Message from {} to {}", sender_id, recipient_id);
                
                // Verify sender exists
                let sender_info = self.storage.get_client_info(&sender_id).await
                    .ok_or_else(|| anyhow!("Unknown sender: {}", sender_id))?;
                
                // Verify signature
                let sender_pubkey = PublicKey::from_bytes(&hex::decode(&sender_info.public_key)?)?;
                let signature_bytes = hex::decode(&signature)?;
                let signature = Signature::from_bytes(&signature_bytes)?;
                
                self.crypto.verify(encrypted_content.as_bytes(), &signature, &sender_pubkey)?;
                
                // Create message
                let message = Message {
                    id: message_id.clone(),
                    sender_id: sender_id.clone(),
                    recipient_id: recipient_id.clone(),
                    content: encrypted_content,
                    timestamp: chrono::Utc::now(),
                    encrypted: true,
                    signature: Some(hex::encode(signature.to_bytes())), // Store as hex string
                };
                
                // Store message
                self.storage.add_message(message.clone()).await?;
                
                // Update sender's last seen
                self.storage.update_client_last_seen(&sender_id).await?;
                
                info!("✅ Message stored successfully");
                Ok(ServerResponse::MessageSent { message_id })
            }

            ServerCommand::GetMessages { client_id } => {
                info!("📥 Retrieving messages for: {}", client_id);
                let messages = self.storage.get_messages_for_client(&client_id).await?;
                
                if let Some(message) = messages.last() {
                    Ok(ServerResponse::MessageReceived { message: message.clone() })
                } else {
                    Ok(ServerResponse::Error { message: "No messages found".to_string() })
                }
            }

            ServerCommand::GetClients => {
                let clients = self.storage.get_all_clients().await;
                Ok(ServerResponse::ClientList { clients })
            }

            ServerCommand::Heartbeat { client_id } => {
                self.storage.update_client_last_seen(&client_id).await?;
                Ok(ServerResponse::Ok)
            }
        }
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            crypto: CryptoManager::new(),
            storage: Storage::new("./data").expect("Failed to create storage"),
            active_connections: Arc::clone(&self.active_connections),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    println!("🔐 Secure Messaging Protocol Server");
    println!("=====================================");
    
    let server = Server::new()?;
    println!("✅ Server initialized successfully");
    println!("🚀 Starting server on 127.0.0.1:8080...");
    
    match server.run("127.0.0.1:8080").await {
        Ok(_) => {
            println!("✅ Server shutdown gracefully");
        }
        Err(e) => {
            eprintln!("❌ Server error: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
