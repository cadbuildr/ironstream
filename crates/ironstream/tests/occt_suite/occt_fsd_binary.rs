extern crate ironstream;

use ironstream::fsd_binary::{BinaryFile, OpenMode, StorageDriver, StorageError, StorageSchema};

#[test]
fn binary_file_integer_round_trip() {
    let mut f = BinaryFile::new();
    f.open("test.bin", OpenMode::Write);
    assert_eq!(f.write_integer(-42), StorageError::Ok);
    f.reset_cursor();
    f.mode = Some(OpenMode::Read);
    assert_eq!(f.read_integer().unwrap(), -42);
}

#[test]
fn binary_file_real_round_trip() {
    let mut f = BinaryFile::new();
    f.open("test.bin", OpenMode::Write);
    f.write_real(2.718281828);
    f.reset_cursor();
    f.mode = Some(OpenMode::Read);
    let v = f.read_real().unwrap();
    assert!((v - 2.718281828).abs() < 1e-12);
}

#[test]
fn binary_file_eof_error() {
    let mut f = BinaryFile::new();
    f.open("test.bin", OpenMode::Read);
    assert!(matches!(f.read_integer(), Err(StorageError::EndOfFile)));
    assert!(matches!(f.read_real(), Err(StorageError::EndOfFile)));
}

#[test]
fn binary_file_write_without_open_mode() {
    let mut f = BinaryFile::new();
    // mode is None => write should fail
    assert_eq!(f.write_integer(1), StorageError::WriteError);
}

#[test]
fn storage_schema_version_string() {
    let s = StorageSchema::new("BinOcaf", 3, 7);
    assert_eq!(s.version_string(), "3.7");
    assert_eq!(s.name, "BinOcaf");
}
