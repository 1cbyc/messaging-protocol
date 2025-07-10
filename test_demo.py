#!/usr/bin/env python3
"""
Demo script for the Secure Messaging Protocol
This simulates the key exchange and messaging process
"""

import json
import hashlib
import hmac
import base64
from datetime import datetime
from typing import Dict, List

class MockCrypto:
    """Mock cryptographic operations for demonstration"""
    
    def __init__(self, name: str):
        self.name = name
        self.ed25519_key = f"{name}_ed25519_key_1234567890abcdef"
        self.x25519_key = f"{name}_x25519_key_abcdef1234567890"
        
    def sign(self, message: str) -> str:
        """Mock Ed25519 signature"""
        signature = hmac.new(
            self.ed25519_key.encode(),
            message.encode(),
            hashlib.sha256
        ).hexdigest()[:64]
        return signature
    
    def encrypt(self, message: str, recipient_key: str) -> str:
        """Mock X25519 + ChaCha20-Poly1305 encryption"""
        # In reality, this would use X25519 key exchange + ChaCha20-Poly1305
        encrypted = base64.b64encode(message.encode()).decode()
        return f"encrypted_{encrypted}_{recipient_key[:8]}"
    
    def decrypt(self, encrypted_msg: str, sender_key: str) -> str:
        """Mock decryption"""
        if encrypted_msg.startswith("encrypted_"):
            parts = encrypted_msg.split("_")
            if len(parts) >= 3:
                return base64.b64decode(parts[1]).decode()
        return "DECRYPTION_FAILED"

class MockServer:
    """Mock server for demonstration"""
    
    def __init__(self):
        self.clients: Dict[str, str] = {}  # client_id -> ed25519_public_key
        self.messages: List[Dict] = []
        
    def register_client(self, client_id: str, public_key: str) -> Dict:
        """Register a new client"""
        self.clients[client_id] = public_key
        return {
            "type": "Registered",
            "server_public_key": "server_ed25519_key_abcdef1234567890"
        }
    
    def send_message(self, sender_id: str, recipient_id: str, 
                    encrypted_content: str, signature: str, message_id: str) -> Dict:
        """Process a message send request"""
        # Verify sender exists
        if sender_id not in self.clients:
            return {"type": "Error", "message": f"Unknown sender: {sender_id}"}
        
        # Verify signature (mock)
        expected_signature = hmac.new(
            self.clients[sender_id].encode(),
            encrypted_content.encode(),
            hashlib.sha256
        ).hexdigest()[:64]
        
        if signature != expected_signature:
            return {"type": "Error", "message": "Invalid signature"}
        
        # Store message
        message = {
            "id": message_id,
            "sender_id": sender_id,
            "recipient_id": recipient_id,
            "content": encrypted_content,
            "timestamp": datetime.now().isoformat(),
            "signature": signature
        }
        self.messages.append(message)
        
        return {"type": "MessageSent", "message_id": message_id}
    
    def get_messages(self, client_id: str) -> Dict:
        """Get messages for a client"""
        client_messages = [msg for msg in self.messages if msg["recipient_id"] == client_id]
        if client_messages:
            return {"type": "MessageReceived", "message": client_messages[-1]}
        else:
            return {"type": "Error", "message": "No messages found"}

def demo_messaging():
    """Demonstrate the secure messaging protocol"""
    
    print("🔐 Secure Messaging Protocol Demo")
    print("==================================")
    print()
    
    # Initialize server and clients
    server = MockServer()
    alice = MockCrypto("alice")
    bob = MockCrypto("bob")
    
    print("📝 Step 1: Client Registration")
    print("-" * 40)
    
    # Register clients
    alice_reg = server.register_client("alice", alice.ed25519_key)
    bob_reg = server.register_client("bob", bob.ed25519_key)
    
    print(f"✅ Alice registered: {alice_reg['server_public_key'][:20]}...")
    print(f"✅ Bob registered: {bob_reg['server_public_key'][:20]}...")
    print()
    
    print("🔑 Step 2: Key Exchange (Out-of-Band)")
    print("-" * 40)
    print(f"Alice's X25519 key: {alice.x25519_key}")
    print(f"Bob's X25519 key: {bob.x25519_key}")
    print("(In real implementation, these would be exchanged securely)")
    print()
    
    print("📤 Step 3: Sending Encrypted Messages")
    print("-" * 40)
    
    # Alice sends message to Bob
    message = "Hello Bob! This is a secret message from Alice."
    print(f"📝 Alice's message: {message}")
    
    # Encrypt for Bob
    encrypted = alice.encrypt(message, bob.x25519_key)
    print(f"🔒 Encrypted content: {encrypted[:50]}...")
    
    # Sign the encrypted content
    signature = alice.sign(encrypted)
    print(f"✍️  Signature: {signature[:20]}...")
    
    # Send to server
    result = server.send_message("alice", "bob", encrypted, signature, "msg_001")
    print(f"📤 Server response: {result['type']}")
    print()
    
    print("📥 Step 4: Receiving Messages")
    print("-" * 40)
    
    # Bob retrieves his messages
    bob_messages = server.get_messages("bob")
    if bob_messages["type"] == "MessageReceived":
        msg = bob_messages["message"]
        print(f"📨 Bob received message from: {msg['sender_id']}")
        print(f"🔒 Encrypted content: {msg['content'][:50]}...")
        
        # Decrypt the message
        decrypted = bob.decrypt(msg['content'], alice.x25519_key)
        print(f"📖 Decrypted message: {decrypted}")
        print(f"✍️  Signature verified: {msg['signature'][:20]}...")
    else:
        print("❌ No messages found")
    
    print()
    print("🔒 Security Features Demonstrated:")
    print("✅ End-to-End Encryption")
    print("✅ Digital Signatures")
    print("✅ Message Authentication")
    print("✅ Perfect Forward Secrecy (conceptually)")
    print("✅ Zero-Knowledge Server")
    print()
    print("🚀 The real implementation uses:")
    print("   - Ed25519 for signatures")
    print("   - X25519 for key exchange")
    print("   - ChaCha20-Poly1305 for encryption")
    print("   - JSON protocol over TCP")
    print("   - Persistent storage")
    print("   - Interactive CLI")

if __name__ == "__main__":
    demo_messaging() 