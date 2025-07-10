# 🔐 Secure Messaging Protocol

A powerful, end-to-end encrypted messaging system built in Rust with modern cryptographic primitives.

## ✨ Features

### 🔒 Security
- **End-to-End Encryption**: Messages are encrypted using X25519 key exchange + ChaCha20-Poly1305
- **Digital Signatures**: Ed25519 signatures ensure message authenticity and integrity
- **Perfect Forward Secrecy**: Each message uses a new ephemeral key
- **Zero-Knowledge Server**: Server cannot decrypt messages, only forwards encrypted data

### 🚀 Performance
- **Async/Await**: Built on Tokio for high-performance concurrent connections
- **JSON Protocol**: Clean, structured communication between clients and server
- **Persistent Storage**: Messages and client data stored on disk
- **Connection Pooling**: Efficient resource management

### 🛠️ Developer Experience
- **Interactive CLI**: Rich command-line interface for testing
- **Comprehensive Logging**: Detailed logs with colored output
- **Error Handling**: Robust error handling with meaningful messages
- **Modular Architecture**: Clean separation of concerns

## 🏗️ Architecture

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Client A  │    │    Server   │    │   Client B  │
│             │    │             │    │             │
│ Ed25519 Key │◄──►│  Message    │◄──►│ Ed25519 Key │
│ X25519 Key  │    │  Storage    │    │ X25519 Key  │
│             │    │  Routing    │    │             │
└─────────────┘    └─────────────┘    └─────────────┘
```

### Message Flow
1. **Registration**: Clients register with server using Ed25519 public keys
2. **Key Exchange**: Clients exchange X25519 public keys (out-of-band)
3. **Encryption**: Sender encrypts message using recipient's X25519 public key
4. **Signing**: Sender signs encrypted message with Ed25519 private key
5. **Transmission**: Server receives, verifies signature, and stores message
6. **Retrieval**: Recipient fetches and decrypts messages

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with Cargo
- Windows/Linux/macOS

### Installation
```bash
git clone https://github.com/1cbyc/messaging-protocol.git
cd messaging-protocol
cargo build --release
```

### Running the Server
```bash
# Terminal 1
cargo run --bin server
```

### Running Clients
```bash
# Terminal 2 - Alice
cargo run --bin client alice

# Terminal 3 - Bob  
cargo run --bin client bob
```

## 📖 Usage Guide

### Server Commands
The server runs automatically and handles:
- Client registration
- Message storage and retrieval
- Signature verification
- Connection management

### Client Commands
Once connected, use these interactive commands:

```bash
# Send encrypted message
send bob Hello, this is a secret message!

# Check for new messages
receive

# List online contacts
contacts

# Add a contact (you need their X25519 public key)
add bob 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef

# Exit the client
quit
```

### Key Exchange
To communicate securely, clients must exchange X25519 public keys:

1. **Alice** shares her X25519 key: `alice > add bob <bob's_x25519_key>`
2. **Bob** shares his X25519 key: `bob > add alice <alice's_x25519_key>`
3. Now they can send encrypted messages to each other

## 🔧 Technical Details

### Cryptographic Primitives
- **Ed25519**: Digital signatures for authentication
- **X25519**: Key exchange for encryption
- **ChaCha20-Poly1305**: Authenticated encryption
- **SHA-256**: Key derivation

### Protocol Messages
```json
// Registration
{
  "Register": {
    "client_id": "alice",
    "public_key": "ed25519_public_key_hex"
  }
}

// Send Message
{
  "Send": {
    "sender_id": "alice",
    "recipient_id": "bob", 
    "encrypted_content": "hex_encoded_encrypted_message",
    "signature": "ed25519_signature_hex",
    "message_id": "uuid"
  }
}
```

### Data Storage
- **Messages**: `./data/messages.json`
- **Clients**: `./data/clients.json`
- **Format**: JSON with timestamps and metadata

## 🛡️ Security Features

### End-to-End Encryption
- Messages are encrypted with recipient's public key
- Only the intended recipient can decrypt
- Server cannot read message contents

### Authentication
- Ed25519 signatures verify message authenticity
- Prevents message tampering and impersonation
- Each client has a unique identity

### Perfect Forward Secrecy
- Each message uses a new ephemeral key
- Compromised keys don't affect past messages
- Future messages remain secure

## 🔍 Troubleshooting

### Common Issues

**Connection Refused**
```bash
# Make sure server is running
cargo run --bin server
```

**Unknown Recipient**
```bash
# Add the recipient's public key first
add recipient_id their_x25519_public_key_hex
```

**Invalid Signature**
```bash
# Check that you're using the correct Ed25519 key
# Re-register if needed
```

### Debug Mode
```bash
# Enable detailed logging
RUST_LOG=debug cargo run --bin client
```

## 🚧 Future Enhancements

- [ ] **WebSocket Support**: Real-time messaging
- [ ] **Group Chats**: Multi-recipient messages
- [ ] **File Transfer**: Encrypted file sharing
- [ ] **Mobile App**: iOS/Android clients
- [ ] **Web Interface**: Browser-based client
- [ ] **Message History**: Persistent chat history
- [ ] **Offline Support**: Message queuing
- [ ] **Key Rotation**: Automatic key updates

## 📄 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 🔗 Related Projects

- [Signal Protocol](https://signal.org/docs/)
- [Matrix Protocol](https://matrix.org/docs/)
- [XMPP](https://xmpp.org/)

---

**Built with ❤️ in Rust** 