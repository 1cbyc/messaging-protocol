#!/usr/bin/env python3
"""
Very simple test to debug server connection
"""

import socket
import time

def simple_test():
    print("🔍 Simple connection test...")
    
    try:
        # Just connect and send a simple string
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        
        print("📡 Connecting...")
        sock.connect(('127.0.0.1', 8080))
        print("✅ Connected!")
        
        # Send a simple string
        print("📤 Sending: 'hello'")
        sock.send(b'hello')
        
        # Wait for any response
        print("📥 Waiting for response...")
        response = sock.recv(1024)
        print(f"📨 Got response: {response}")
        
        sock.close()
        print("✅ Test completed!")
        
    except Exception as e:
        print(f"❌ Test failed: {e}")

if __name__ == "__main__":
    simple_test() 