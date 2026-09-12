use gitvault::manifest::Manifest;

#[test]
fn test_manifest_new_and_verify() {
    let data = b"encrypted blob bytes";
    let m = Manifest::new("homebridge", data);
    assert_eq!(m.name, "homebridge");
    assert_eq!(m.ciphertext_bytes, data.len() as u64);
    assert!(m.verify(data));
    assert!(!m.verify(b"tampered"));
}

#[test]
fn test_manifest_json_roundtrip() {
    let m = Manifest::new("n8n", b"x");
    let s = m.to_json_pretty().unwrap();
    let parsed = Manifest::from_json(&s).unwrap();
    assert_eq!(parsed.name, m.name);
    assert_eq!(parsed.sha256, m.sha256);
    assert_eq!(parsed.ciphertext_bytes, m.ciphertext_bytes);
}
