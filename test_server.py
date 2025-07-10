#!/usr/bin/env python3
"""
Simple test to check if the server is responding
"""

import socket
import json
import time

def test_server():
    print("🔍 Testing server connection...")
    
    try:
        # Create a socket connection
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        
        print("📡 Connecting to 127.0.0.1:8080...")
        sock.connect(('127.0.0.1', 8080))
        print("✅ Connected to server!")
        
        # Send a simple registration message
        register_msg = {
            "Register": {
                "client_id": "test_client",
                "public_key": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            }
        }
        
        print("📤 Sending registration message...")
        message = json.dumps(register_msg)
        print(f"📤 Message: {message}")
        sock.send(message.encode())
        
        # Wait for response
        print("📥 Waiting for response...")
        response = sock.recv(4096)
        print(f"📨 Received: {response.decode()}")
        
        sock.close()
        print("✅ Server test successful!")
        
    except Exception as e:
        print(f"❌ Server test failed: {e}")
        return False
    
    return True

if __name__ == "__main__":
    test_server() 