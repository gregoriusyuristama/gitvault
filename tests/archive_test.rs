use gitvault::archive::{pack_directory, unpack_directory};
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_pack_unpack_flat_directory() {
    let src = tempdir().unwrap();
    let mut f = File::create(src.path().join("test.txt")).unwrap();
    f.write_all(b"sample file content").unwrap();

    let dest = tempdir().unwrap();
    let mut buf = Vec::new();
    pack_directory(src.path(), &mut buf).unwrap();
    assert!(!buf.is_empty(), "archive buffer should be non-empty");
    unpack_directory(&buf[..], dest.path()).unwrap();

    let restored = dest.path().join("test.txt");
    assert!(restored.exists());
    assert_eq!(fs::read_to_string(restored).unwrap(), "sample file content");
}

#[test]
fn test_pack_unpack_nested_directory() {
    let src = tempdir().unwrap();
    fs::create_dir_all(src.path().join("sub/deep")).unwrap();
    fs::write(src.path().join("root.txt"), "root").unwrap();
    fs::write(src.path().join("sub/a.txt"), "alpha").unwrap();
    fs::write(src.path().join("sub/deep/b.txt"), "bravo").unwrap();

    let dest = tempdir().unwrap();
    let mut buf = Vec::new();
    pack_directory(src.path(), &mut buf).unwrap();
    unpack_directory(&buf[..], dest.path()).unwrap();

    assert_eq!(fs::read_to_string(dest.path().join("root.txt")).unwrap(), "root");
    assert_eq!(fs::read_to_string(dest.path().join("sub/a.txt")).unwrap(), "alpha");
    assert_eq!(fs::read_to_string(dest.path().join("sub/deep/b.txt")).unwrap(), "bravo");
}
