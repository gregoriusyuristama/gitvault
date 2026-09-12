//! Manifest module: backup metadata and integrity checksums.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub sha256: String,
    pub ciphertext_bytes: u64,
    pub gitvault_version: String,
}

impl Manifest {
    /// Build a manifest from the encrypted-blob bytes.
    pub fn new(name: &str, ciphertext: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(ciphertext);
        let sha256 = format!("{:x}", hasher.finalize());
        Self {
            name: name.to_string(),
            timestamp: Utc::now(),
            sha256,
            ciphertext_bytes: ciphertext.len() as u64,
            gitvault_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Verify a candidate ciphertext against the stored SHA-256.
    pub fn verify(&self, ciphertext: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(ciphertext);
        format!("{:x}", hasher.finalize()) == self.sha256
    }

    pub fn to_json_pretty(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
