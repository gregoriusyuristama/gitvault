//! Crypto module: age passphrase encryption/decryption.

use anyhow::{Context, Result};
use secrecy::SecretString;
use std::io::{Read, Write};

/// Encrypt bytes from `reader` into `writer` using a passphrase (age).
pub fn encrypt_stream<R: Read, W: Write>(
    mut reader: R,
    writer: W,
    passphrase: &str,
) -> Result<()> {
    let encryptor = age::Encryptor::with_user_passphrase(SecretString::new(passphrase.to_string()));
    let mut output = encryptor
        .wrap_output(writer)
        .context("failed to initialize age encryptor")?;
    std::io::copy(&mut reader, &mut output).context("failed to write ciphertext")?;
    output.finish().context("failed to finalize ciphertext")?;
    Ok(())
}

/// Decrypt bytes from `reader` into `writer` using a passphrase (age).
pub fn decrypt_stream<R: Read, W: Write>(
    reader: R,
    mut writer: W,
    passphrase: &str,
) -> Result<()> {
    let decryptor = match age::Decryptor::new(reader).context("failed to parse age header")? {
        age::Decryptor::Passphrase(d) => d,
        _ => anyhow::bail!("unsupported encryption format: expected passphrase-encrypted payload"),
    };
    let mut reader = decryptor
        .decrypt(&SecretString::new(passphrase.to_string()), None)
        .context("decryption failed (wrong passphrase or corrupt ciphertext)")?;
    std::io::copy(&mut reader, &mut writer).context("failed to write plaintext")?;
    Ok(())
}
