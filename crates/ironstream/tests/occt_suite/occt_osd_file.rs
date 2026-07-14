// FILE: tests/occt_osd_file.rs
extern crate ironstream;

use ironstream::osd_file::{OsdDirectory, OsdError, OsdFile, OsdOpenMode, OsdPath};

#[test]
fn osd_path_name_extension_directory() {
    let p = OsdPath::new("/usr/local/data/model.step");
    assert_eq!(p.name(), "model.step");
    assert_eq!(p.extension(), "step");
    assert_eq!(p.directory(), "/usr/local/data");
    assert!(p.is_absolute());
}

#[test]
fn osd_path_relative_and_no_extension() {
    let p = OsdPath::new("models/assembly");
    assert!(!p.is_absolute());
    assert_eq!(p.name(), "assembly");
    assert_eq!(p.extension(), "");
}

#[test]
fn osd_file_open_write_bytes() {
    let mut f = OsdFile::new(OsdPath::new("/tmp/test.bin"));
    f.open(OsdOpenMode::InOut);
    assert!(f.is_open);
    assert!(!f.failed());
    f.write_bytes(b"hello world");
    assert_eq!(f.size_bytes, 11);
    f.close();
    assert!(!f.is_open);
}

#[test]
fn osd_file_write_denied_when_in_mode() {
    let mut f = OsdFile::new(OsdPath::new("/tmp/readonly.bin"));
    f.open(OsdOpenMode::In);
    f.write_bytes(b"should fail");
    // write should be denied in In mode
    assert_eq!(f.error, OsdError::AccessDenied);
}

#[test]
fn osd_directory_exists() {
    let d = OsdDirectory::new(OsdPath::new("/tmp/mydir"));
    assert!(d.exists(), "non-empty path directory should exist");

    let empty_d = OsdDirectory::new(OsdPath::new(""));
    assert!(!empty_d.exists(), "empty path directory should not exist");
}
