use gitvault::crypto::{decrypt_stream, encrypt_stream};
use std::io::Cursor;

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let passphrase = "super-secret-password-123";
    let data = b"Hello, secure backup world! Testing age encryption.";

    let mut encrypted = Vec::new();
    encrypt_stream(&data[..], &mut encrypted, passphrase).expect("encryption failed");
    assert_ne!(
        &encrypted[..],
        &data[..],
        "ciphertext should differ from plaintext"
    );

    let mut decrypted = Vec::new();
    decrypt_stream(Cursor::new(encrypted), &mut decrypted, passphrase).expect("decryption failed");
    assert_eq!(&decrypted[..], data);
}

#[test]
fn test_decrypt_wrong_passphrase_fails() {
    let data = b"Secret data";
    let mut encrypted = Vec::new();
    encrypt_stream(&data[..], &mut encrypted, "correct-pass").unwrap();

    let mut decrypted = Vec::new();
    let result = decrypt_stream(Cursor::new(encrypted), &mut decrypted, "wrong-pass");
    assert!(
        result.is_err(),
        "decryption with wrong passphrase must fail"
    );
}

#[test]
fn test_encrypt_empty_input() {
    let data: &[u8] = &[];
    let mut encrypted = Vec::new();
    encrypt_stream(data, &mut encrypted, "pass").unwrap();

    let mut decrypted = Vec::new();
    decrypt_stream(Cursor::new(encrypted), &mut decrypted, "pass").unwrap();
    assert!(decrypted.is_empty());
}
