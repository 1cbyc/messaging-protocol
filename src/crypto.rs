use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use anyhow::{Result, anyhow};

pub struct CryptoManager {
    ed25519_keypair: Keypair,
    x25519_secret: StaticSecret,
    x25519_public: X25519PublicKey,
}

impl CryptoManager {
    pub fn new() -> Self {
        let ed25519_keypair = Keypair::generate(&mut OsRng);
        let x25519_secret = StaticSecret::new(OsRng);
        let x25519_public = X25519PublicKey::from(&x25519_secret);
        
        Self {
            ed25519_keypair,
            x25519_secret,
            x25519_public,
        }
    }

    pub fn get_ed25519_public_key(&self) -> PublicKey {
        self.ed25519_keypair.public
    }

    pub fn get_x25519_public_key(&self) -> X25519PublicKey {
        self.x25519_public
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.ed25519_keypair.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature, public_key: &PublicKey) -> Result<()> {
        public_key.verify(message, signature)?;
        Ok(())
    }

    pub fn encrypt_message(&self, recipient_public_key: &X25519PublicKey, message: &str) -> Result<Vec<u8>> {
        // Generate shared secret
        let shared_secret = self.x25519_secret.diffie_hellman(recipient_public_key);
        
        // Derive encryption key from shared secret
        let key = Key::from_slice(&shared_secret.as_bytes()[..32]);
        let cipher = ChaCha20Poly1305::new(key);
        
        // Generate random nonce
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt message
        let encrypted = cipher.encrypt(nonce, message.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        // Combine nonce and encrypted data
        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&encrypted);
        
        Ok(result)
    }

    pub fn decrypt_message(&self, sender_public_key: &X25519PublicKey, encrypted_data: &[u8]) -> Result<String> {
        if encrypted_data.len() < 12 {
            return Err(anyhow!("Invalid encrypted data length"));
        }
        
        // Extract nonce and encrypted data
        let nonce_bytes = &encrypted_data[..12];
        let encrypted = &encrypted_data[12..];
        
        // Generate shared secret
        let shared_secret = self.x25519_secret.diffie_hellman(sender_public_key);
        
        // Derive decryption key from shared secret
        let key = Key::from_slice(&shared_secret.as_bytes()[..32]);
        let cipher = ChaCha20Poly1305::new(key);
        
        // Decrypt message
        let nonce = Nonce::from_slice(nonce_bytes);
        let decrypted = cipher.decrypt(nonce, encrypted)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        
        String::from_utf8(decrypted)
            .map_err(|e| anyhow!("Invalid UTF-8 in decrypted message: {}", e))
    }
} 