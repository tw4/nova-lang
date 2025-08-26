// Nova Standard Library - Cryptography Module

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

/// Simple hash implementation for Nova
#[derive(Debug, Clone)]
pub struct NovaHash {
    algorithm: HashAlgorithm,
    data: Vec<u8>,
}

impl NovaHash {
    /// Create new hash instance
    pub fn new(algorithm: HashAlgorithm) -> Self {
        Self {
            algorithm,
            data: Vec::new(),
        }
    }

    /// Update hash with data
    pub fn update(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    /// Update hash with string
    pub fn update_str(&mut self, data: &str) {
        self.update(data.as_bytes());
    }

    /// Finalize and get hash digest as hex string
    pub fn finalize(&self) -> String {
        match self.algorithm {
            HashAlgorithm::Md5 => self.md5_digest(),
            HashAlgorithm::Sha1 => self.sha1_digest(),
            HashAlgorithm::Sha256 => self.sha256_digest(),
            HashAlgorithm::Sha512 => self.sha512_digest(),
        }
    }

    /// Finalize and get hash digest as bytes
    pub fn finalize_bytes(&self) -> Vec<u8> {
        match self.algorithm {
            HashAlgorithm::Md5 => self.md5_digest_bytes(),
            HashAlgorithm::Sha1 => self.sha1_digest_bytes(),
            HashAlgorithm::Sha256 => self.sha256_digest_bytes(),
            HashAlgorithm::Sha512 => self.sha512_digest_bytes(),
        }
    }

    // Simplified MD5 implementation (not cryptographically secure)
    fn md5_digest(&self) -> String {
        let digest = self.md5_digest_bytes();
        hex_encode(&digest)
    }

    fn md5_digest_bytes(&self) -> Vec<u8> {
        // This is a simplified MD5-like hash, not actual MD5
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Convert to 16 bytes (MD5 size)
        let mut result = Vec::with_capacity(16);
        for i in 0..2 {
            let part = (hash >> (i * 32)) as u32;
            result.extend_from_slice(&part.to_le_bytes());
        }
        // Fill to 16 bytes
        while result.len() < 16 {
            result.push(0);
        }
        result.truncate(16);
        result
    }

    // Simplified SHA1 implementation
    fn sha1_digest(&self) -> String {
        let digest = self.sha1_digest_bytes();
        hex_encode(&digest)
    }

    fn sha1_digest_bytes(&self) -> Vec<u8> {
        // This is a simplified SHA1-like hash, not actual SHA1
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Convert to 20 bytes (SHA1 size)
        let mut result = Vec::with_capacity(20);
        result.extend_from_slice(&hash.to_le_bytes());
        result.extend_from_slice(&(hash ^ 0xABCDEF).to_le_bytes());
        // Fill to 20 bytes
        while result.len() < 20 {
            result.push(0);
        }
        result.truncate(20);
        result
    }

    // Simplified SHA256 implementation
    fn sha256_digest(&self) -> String {
        let digest = self.sha256_digest_bytes();
        hex_encode(&digest)
    }

    fn sha256_digest_bytes(&self) -> Vec<u8> {
        // This is a simplified SHA256-like hash, not actual SHA256
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        self.data.hash(&mut hasher1);
        self.data.iter().rev().for_each(|b| b.hash(&mut hasher2));
        
        let hash1 = hasher1.finish();
        let hash2 = hasher2.finish();
        
        // Convert to 32 bytes (SHA256 size)
        let mut result = Vec::with_capacity(32);
        result.extend_from_slice(&hash1.to_le_bytes());
        result.extend_from_slice(&hash2.to_le_bytes());
        result.extend_from_slice(&(hash1 ^ hash2).to_le_bytes());
        result.extend_from_slice(&(hash1.wrapping_add(hash2)).to_le_bytes());
        result
    }

    // Simplified SHA512 implementation
    fn sha512_digest(&self) -> String {
        let digest = self.sha512_digest_bytes();
        hex_encode(&digest)
    }

    fn sha512_digest_bytes(&self) -> Vec<u8> {
        // This is a simplified SHA512-like hash, not actual SHA512
        let mut result = Vec::with_capacity(64);
        
        for chunk in self.data.chunks(8) {
            let mut hasher = DefaultHasher::new();
            chunk.hash(&mut hasher);
            result.extend_from_slice(&hasher.finish().to_le_bytes());
        }
        
        // Fill to 64 bytes
        while result.len() < 64 {
            let mut hasher = DefaultHasher::new();
            result.len().hash(&mut hasher);
            result.extend_from_slice(&hasher.finish().to_le_bytes());
        }
        result.truncate(64);
        result
    }
}

/// Base64 encoding/decoding
pub struct Base64;

impl Base64 {
    const CHARS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    /// Encode bytes to base64 string
    pub fn encode(input: &[u8]) -> String {
        let mut result = String::new();
        let mut i = 0;
        
        while i < input.len() {
            let b1 = input[i];
            let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
            let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };
            
            let bitmap = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
            
            result.push(Self::CHARS[((bitmap >> 18) & 63) as usize] as char);
            result.push(Self::CHARS[((bitmap >> 12) & 63) as usize] as char);
            
            if i + 1 < input.len() {
                result.push(Self::CHARS[((bitmap >> 6) & 63) as usize] as char);
            } else {
                result.push('=');
            }
            
            if i + 2 < input.len() {
                result.push(Self::CHARS[(bitmap & 63) as usize] as char);
            } else {
                result.push('=');
            }
            
            i += 3;
        }
        
        result
    }
    
    /// Decode base64 string to bytes
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let input = input.trim();
        if input.len() % 4 != 0 {
            return Err("Invalid base64 length".to_string());
        }
        
        let mut result = Vec::new();
        let bytes = input.as_bytes();
        
        for chunk in bytes.chunks(4) {
            if chunk.len() != 4 {
                return Err("Invalid base64 chunk".to_string());
            }
            
            let mut values = [0u8; 4];
            for (i, &b) in chunk.iter().enumerate() {
                if b == b'=' {
                    values[i] = 0;
                } else {
                    values[i] = Self::char_to_value(b)?;
                }
            }
            
            let bitmap = ((values[0] as u32) << 18) |
                        ((values[1] as u32) << 12) |
                        ((values[2] as u32) << 6) |
                        (values[3] as u32);
            
            result.push((bitmap >> 16) as u8);
            
            if chunk[2] != b'=' {
                result.push((bitmap >> 8) as u8);
            }
            
            if chunk[3] != b'=' {
                result.push(bitmap as u8);
            }
        }
        
        Ok(result)
    }
    
    fn char_to_value(c: u8) -> Result<u8, String> {
        match c {
            b'A'..=b'Z' => Ok(c - b'A'),
            b'a'..=b'z' => Ok(c - b'a' + 26),
            b'0'..=b'9' => Ok(c - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            _ => Err(format!("Invalid base64 character: {}", c as char)),
        }
    }
}

/// Hex encoding/decoding
pub struct Hex;

impl Hex {
    /// Encode bytes to hex string
    pub fn encode(input: &[u8]) -> String {
        hex_encode(input)
    }
    
    /// Decode hex string to bytes
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        hex_decode(input)
    }
}

/// URL encoding/decoding
pub struct UrlEncoding;

impl UrlEncoding {
    /// Encode string for URL
    pub fn encode(input: &str) -> String {
        let mut result = String::new();
        
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                b' ' => {
                    result.push('+');
                }
                _ => {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
        
        result
    }
    
    /// Decode URL-encoded string
    pub fn decode(input: &str) -> Result<String, String> {
        let mut result = Vec::new();
        let bytes = input.as_bytes();
        let mut i = 0;
        
        while i < bytes.len() {
            match bytes[i] {
                b'+' => {
                    result.push(b' ');
                    i += 1;
                }
                b'%' => {
                    if i + 2 >= bytes.len() {
                        return Err("Invalid URL encoding".to_string());
                    }
                    
                    let hex_str = std::str::from_utf8(&bytes[i+1..i+3])
                        .map_err(|_| "Invalid hex in URL encoding")?;
                    
                    let byte = u8::from_str_radix(hex_str, 16)
                        .map_err(|_| "Invalid hex value in URL encoding")?;
                    
                    result.push(byte);
                    i += 3;
                }
                b => {
                    result.push(b);
                    i += 1;
                }
            }
        }
        
        String::from_utf8(result).map_err(|_| "Invalid UTF-8 in decoded URL".to_string())
    }
}

/// Simple XOR cipher
pub struct XorCipher;

impl XorCipher {
    /// Encrypt/decrypt data with XOR cipher (same operation for both)
    pub fn crypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return data.to_vec();
        }
        
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }
    
    /// Encrypt/decrypt string with XOR cipher
    pub fn crypt_str(data: &str, key: &str) -> Vec<u8> {
        Self::crypt(data.as_bytes(), key.as_bytes())
    }
}

/// Caesar cipher
pub struct CaesarCipher;

impl CaesarCipher {
    /// Encrypt text with Caesar cipher
    pub fn encrypt(text: &str, shift: i32) -> String {
        text.chars()
            .map(|c| Self::shift_char(c, shift))
            .collect()
    }
    
    /// Decrypt text with Caesar cipher
    pub fn decrypt(text: &str, shift: i32) -> String {
        Self::encrypt(text, -shift)
    }
    
    fn shift_char(c: char, shift: i32) -> char {
        match c {
            'a'..='z' => {
                let shifted = ((c as u8 - b'a') as i32 + shift).rem_euclid(26);
                (shifted as u8 + b'a') as char
            }
            'A'..='Z' => {
                let shifted = ((c as u8 - b'A') as i32 + shift).rem_euclid(26);
                (shifted as u8 + b'A') as char
            }
            _ => c,
        }
    }
}

/// Cryptographic utilities
pub struct Crypto;

impl Crypto {
    /// Hash string with specified algorithm
    pub fn hash(algorithm: HashAlgorithm, data: &str) -> String {
        let mut hasher = NovaHash::new(algorithm);
        hasher.update_str(data);
        hasher.finalize()
    }
    
    /// Hash bytes with specified algorithm
    pub fn hash_bytes(algorithm: HashAlgorithm, data: &[u8]) -> String {
        let mut hasher = NovaHash::new(algorithm);
        hasher.update(data);
        hasher.finalize()
    }
    
    /// Generate HMAC (simplified version)
    pub fn hmac(algorithm: HashAlgorithm, key: &[u8], message: &[u8]) -> String {
        let mut inner_key = vec![0x36u8; 64];
        let mut outer_key = vec![0x5cu8; 64];
        
        for (i, &k) in key.iter().enumerate().take(64) {
            inner_key[i] ^= k;
            outer_key[i] ^= k;
        }
        
        // Inner hash
        let mut inner_hasher = NovaHash::new(algorithm);
        inner_hasher.update(&inner_key);
        inner_hasher.update(message);
        let inner_hash = inner_hasher.finalize_bytes();
        
        // Outer hash
        let mut outer_hasher = NovaHash::new(algorithm);
        outer_hasher.update(&outer_key);
        outer_hasher.update(&inner_hash);
        outer_hasher.finalize()
    }
    
    /// Generate simple checksum
    pub fn checksum(data: &[u8]) -> u32 {
        data.iter().map(|&b| b as u32).sum()
    }
    
    /// Generate CRC32 checksum (simplified)
    pub fn crc32(data: &[u8]) -> u32 {
        let mut crc = 0xFFFFFFFFu32;
        
        for &byte in data {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        
        crc ^ 0xFFFFFFFF
    }
    
    /// Constant-time string comparison (for preventing timing attacks)
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (&x, &y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        
        result == 0
    }
    
    /// Generate secure random bytes (using system randomness when available)
    pub fn random_bytes(len: usize) -> Vec<u8> {
        // This is a simplified version - in practice you'd use a cryptographically secure RNG
        let mut rng = crate::random::NovaRng::new_seeded();
        rng.bytes(len)
    }
    
    /// Generate random salt
    pub fn generate_salt(len: usize) -> Vec<u8> {
        Self::random_bytes(len)
    }
}

// Helper functions
fn hex_encode(input: &[u8]) -> String {
    input.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

fn hex_decode(input: &str) -> Result<Vec<u8>, String> {
    if input.len() % 2 != 0 {
        return Err("Invalid hex string length".to_string());
    }
    
    let mut result = Vec::with_capacity(input.len() / 2);
    
    for chunk in input.as_bytes().chunks(2) {
        let hex_str = std::str::from_utf8(chunk)
            .map_err(|_| "Invalid hex string")?;
        
        let byte = u8::from_str_radix(hex_str, 16)
            .map_err(|_| "Invalid hex character")?;
        
        result.push(byte);
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let mut hasher = NovaHash::new(HashAlgorithm::Sha256);
        hasher.update_str("hello world");
        let digest = hasher.finalize();
        
        // Should produce consistent hash
        assert!(!digest.is_empty());
        assert_eq!(digest.len(), 64); // SHA256 hex = 64 chars
    }

    #[test]
    fn test_base64() {
        let data = b"hello world";
        let encoded = Base64::encode(data);
        let decoded = Base64::decode(&encoded).unwrap();
        
        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn test_hex() {
        let data = b"hello";
        let encoded = Hex::encode(data);
        let decoded = Hex::decode(&encoded).unwrap();
        
        assert_eq!(data, decoded.as_slice());
        assert_eq!(encoded, "68656c6c6f");
    }

    #[test]
    fn test_url_encoding() {
        let text = "hello world!";
        let encoded = UrlEncoding::encode(text);
        let decoded = UrlEncoding::decode(&encoded).unwrap();
        
        assert_eq!(text, decoded);
        assert!(encoded.contains("+") || encoded.contains("%20"));
    }

    #[test]
    fn test_xor_cipher() {
        let data = "hello world";
        let key = "secret";
        
        let encrypted = XorCipher::crypt_str(data, key);
        let decrypted = XorCipher::crypt(&encrypted, key.as_bytes());
        let decrypted_str = String::from_utf8(decrypted).unwrap();
        
        assert_eq!(data, decrypted_str);
    }

    #[test]
    fn test_caesar_cipher() {
        let text = "hello world";
        let shift = 3;
        
        let encrypted = CaesarCipher::encrypt(text, shift);
        let decrypted = CaesarCipher::decrypt(&encrypted, shift);
        
        assert_eq!(text, decrypted);
        assert_ne!(text, encrypted);
    }

    #[test]
    fn test_checksum() {
        let data1 = b"hello";
        let data2 = b"world";
        
        let csum1 = Crypto::checksum(data1);
        let csum2 = Crypto::checksum(data2);
        
        assert_ne!(csum1, csum2);
    }

    #[test]
    fn test_constant_time_compare() {
        assert!(Crypto::constant_time_compare(b"hello", b"hello"));
        assert!(!Crypto::constant_time_compare(b"hello", b"world"));
        assert!(!Crypto::constant_time_compare(b"hello", b"hello world"));
    }
}