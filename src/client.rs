mod types;
mod crypto;

use crate::types::{ServerCommand, ServerResponse, Message};
use crate::crypto::CryptoManager;
use ed25519_dalek::PublicKey;
use hex;
use serde_json;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::{Result, anyhow};
use colored::*;
use log::{info, error};
use std::io::{self, Write};
use x25519_dalek::PublicKey as X25519PublicKey;

struct Client {
    id: String,
    crypto: CryptoManager,
    server_pubkey: Option<PublicKey>,
    connected_clients: std::collections::HashMap<String, X25519PublicKey>,
}

impl Client {
    fn new(id: &str) -> Self {
        let crypto = CryptoManager::new();
        Client {
            id: id.to_string(),
            crypto,
            server_pubkey: None,
            connected_clients: std::collections::HashMap::new(),
        }
    }

    async fn connect(&mut self, addr: &str) -> Result<()> {
        let mut stream = TcpStream::connect(addr).await?;
        info!("🔗 Connected to server at {}", addr);
        
        // Register with server
        let register_cmd = ServerCommand::Register {
            client_id: self.id.clone(),
            public_key: hex::encode(self.crypto.get_ed25519_public_key().as_bytes()),
        };
        
        let request = serde_json::to_string(&register_cmd)?;
        stream.write_all(request.as_bytes()).await?;
        
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[..n]);
        
        let server_response: ServerResponse = serde_json::from_str(&response)?;
        match server_response {
            ServerResponse::Registered { server_public_key } => {
                self.server_pubkey = Some(PublicKey::from_bytes(&hex::decode(&server_public_key)?)?);
                info!("✅ Successfully registered with server");
                info!("🔑 Server public key: {}", server_public_key.yellow());
                Ok(())
            }
            _ => Err(anyhow!("Unexpected response from server"))
        }
    }

    async fn send_message(&self, addr: &str, recipient: &str, message: &str) -> Result<()> {
        // Get recipient's public key (in a real app, this would be from a key server)
        let recipient_pubkey = self.connected_clients.get(recipient)
            .ok_or_else(|| anyhow!("Recipient {} not found. You need to exchange keys first.", recipient))?;
        
        // Encrypt message for recipient
        let encrypted_content = self.crypto.encrypt_message(recipient_pubkey, message)?;
        let encrypted_hex = hex::encode(&encrypted_content);
        
        // Sign the encrypted content
        let signature = self.crypto.sign(encrypted_content.as_slice());
        
        let send_cmd = ServerCommand::Send {
            sender_id: self.id.clone(),
            recipient_id: recipient.to_string(),
            encrypted_content: encrypted_hex,
            signature: hex::encode(signature.to_bytes()),
            message_id: uuid::Uuid::new_v4().to_string(),
        };
        
        let mut stream = TcpStream::connect(addr).await?;
        let request = serde_json::to_string(&send_cmd)?;
        stream.write_all(request.as_bytes()).await?;
        
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[..n]);
        
        let server_response: ServerResponse = serde_json::from_str(&response)?;
        match server_response {
            ServerResponse::MessageSent { message_id } => {
                info!("✅ Message sent successfully (ID: {})", message_id);
                Ok(())
            }
            ServerResponse::Error { message } => {
                error!("❌ Failed to send message: {}", message);
                Err(anyhow!("Server error: {}", message))
            }
            _ => Err(anyhow!("Unexpected response from server"))
        }
    }

    async fn receive_messages(&self, addr: &str) -> Result<Vec<Message>> {
        let get_messages_cmd = ServerCommand::GetMessages {
            client_id: self.id.clone(),
        };
        
        let mut stream = TcpStream::connect(addr).await?;
        let request = serde_json::to_string(&get_messages_cmd)?;
        stream.write_all(request.as_bytes()).await?;
        
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[..n]);
        
        let server_response: ServerResponse = serde_json::from_str(&response)?;
        match server_response {
            ServerResponse::MessageReceived { message } => {
                Ok(vec![message])
            }
            ServerResponse::Error { message } => {
                if message.contains("No messages found") {
                    Ok(vec![])
                } else {
                    Err(anyhow!("Server error: {}", message))
                }
            }
            _ => Err(anyhow!("Unexpected response from server"))
        }
    }

    async fn get_online_clients(&self, addr: &str) -> Result<Vec<String>> {
        let get_clients_cmd = ServerCommand::GetClients;
        
        let mut stream = TcpStream::connect(addr).await?;
        let request = serde_json::to_string(&get_clients_cmd)?;
        stream.write_all(request.as_bytes()).await?;
        
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[..n]);
        
        let server_response: ServerResponse = serde_json::from_str(&response)?;
        match server_response {
            ServerResponse::ClientList { clients } => {
                Ok(clients)
            }
            _ => Err(anyhow!("Unexpected response from server"))
        }
    }

    fn add_contact(&mut self, contact_id: String, public_key: X25519PublicKey) {
        self.connected_clients.insert(contact_id, public_key);
        info!("👤 Added contact with public key: {}", hex::encode(public_key.as_bytes()).yellow());
    }

    async fn interactive_mode(&mut self, addr: &str) -> Result<()> {
        println!("\n🔐 Secure Messaging Client - Interactive Mode");
        println!("=============================================");
        println!("Commands:");
        println!("  send <recipient> <message>  - Send encrypted message");
        println!("  receive                     - Check for new messages");
        println!("  contacts                    - List online contacts");
        println!("  add <contact_id> <pubkey>   - Add contact (hex encoded X25519 key)");
        println!("  quit                        - Exit");
        println!();

        loop {
            print!("{} > ", self.id.green());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "send" => {
                    if parts.len() < 3 {
                        println!("❌ Usage: send <recipient> <message>");
                        continue;
                    }
                    let recipient = parts[1];
                    let message = parts[2..].join(" ");
                    
                    match self.send_message(addr, recipient, &message).await {
                        Ok(_) => println!("✅ Message sent to {}", recipient),
                        Err(e) => println!("❌ Failed to send message: {}", e),
                    }
                }
                
                "receive" => {
                    match self.receive_messages(addr).await {
                        Ok(messages) => {
                            if messages.is_empty() {
                                println!("📭 No new messages");
                            } else {
                                println!("📥 Received {} message(s):", messages.len());
                                for msg in messages {
                                    println!("  From: {} at {}", msg.sender_id, msg.timestamp);
                                    if let Some(signature) = &msg.signature {
                                        println!("  Signature: {}", signature);
                                    }
                                }
                            }
                        }
                        Err(e) => println!("❌ Failed to receive messages: {}", e),
                    }
                }
                
                "contacts" => {
                    match self.get_online_clients(addr).await {
                        Ok(clients) => {
                            println!("👥 Online contacts:");
                            for client in clients {
                                if client != self.id {
                                    println!("  - {}", client);
                                }
                            }
                        }
                        Err(e) => println!("❌ Failed to get contacts: {}", e),
                    }
                }
                
                "add" => {
                    if parts.len() != 3 {
                        println!("❌ Usage: add <contact_id> <pubkey>");
                        continue;
                    }
                    let contact_id = parts[1];
                    let pubkey_hex = parts[2];
                    
                    match hex::decode(pubkey_hex) {
                        Ok(bytes) => {
                            if bytes.len() == 32 {
                                // Create X25519PublicKey from bytes
                                let mut key_bytes = [0u8; 32];
                                key_bytes.copy_from_slice(&bytes);
                                let pubkey = X25519PublicKey::from(key_bytes);
                                self.add_contact(contact_id.to_string(), pubkey);
                            } else {
                                println!("❌ Invalid public key length");
                            }
                        }
                        Err(_) => println!("❌ Invalid hex encoding"),
                    }
                }
                
                "quit" => {
                    println!("👋 Goodbye!");
                    break;
                }
                
                _ => {
                    println!("❌ Unknown command. Type 'quit' to exit.");
                }
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    let default_name = "anonymous".to_string();
    let client_id = args.get(1).unwrap_or(&default_name);
    
    let mut client = Client::new(client_id);
    
    println!("🔐 Secure Messaging Client");
    println!("==========================");
    println!("Client ID: {}", client_id.green());
    println!("Public Key: {}", hex::encode(client.crypto.get_ed25519_public_key().as_bytes()).yellow());
    println!("X25519 Key: {}", hex::encode(client.crypto.get_x25519_public_key().as_bytes()).cyan());
    
    // Connect to server
    match client.connect("127.0.0.1:8080").await {
        Ok(_) => {
            println!("✅ Connected to server successfully!");
            
            // Start interactive mode
            client.interactive_mode("127.0.0.1:8080").await?;
        }
        Err(e) => {
            error!("❌ Failed to connect to server: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}