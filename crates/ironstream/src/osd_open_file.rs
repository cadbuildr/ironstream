// FILE: osd_open_file.rs
// occt: OSD_OpenFile

use std::fs::{File, OpenOptions};
use std::io;

/// File opening utilities.
pub struct OpenFile;

impl OpenFile {
    pub fn open(path: &str) -> io::Result<File> {
        File::open(path)
    }

    pub fn create(path: &str) -> io::Result<File> {
        File::create(path)
    }

    pub fn open_read_append(path: &str) -> io::Result<File> {
        OpenOptions::new().read(true).append(true).open(path)
    }
}
