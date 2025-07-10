#!/usr/bin/env python3
"""
Simple registration test
"""

import socket
import json
import time

def test_register():
    print("🔍 Testing registration...")
    
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        
        print("📡 Connecting...")
        sock.connect(('127.0.0.1', 8080))
        print("✅ Connected!")
        
        # Send a minimal registration message
        register_msg = {
            "Register": {
                "client_id": "test",
                "public_key": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            }
        }
        
        message = json.dumps(register_msg)
        print(f"📤 Sending: {message}")
        sock.send(message.encode())
        
        print("📥 Waiting for response...")
        response = sock.recv(4096)
        print(f"📨 Response: {response.decode()}")
        
        sock.close()
        print("✅ Test completed!")
        
    except Exception as e:
        print(f"❌ Test failed: {e}")

if __name__ == "__main__":
    test_register() 