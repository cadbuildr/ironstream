// FILE: fsd_binary.rs
// occt: FSD_BinaryFile, FSD_CmpFile
// occt-ref: Storage_BaseDriver

// occt: Storage_Error
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageError {
    Ok,
    OpenDrive,
    EndOfFile,
    BadData,
    WriteError,
    ReadError,
}

// occt: Storage_OpenMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenMode {
    Read,
    Write,
    Append,
}

// occt-ref: Storage_BaseDriver // (simplified trait)
pub trait StorageDriver {
    fn open(&mut self, path: &str, mode: OpenMode) -> StorageError;
    fn close(&mut self) -> StorageError;
    fn is_end(&self) -> bool;
    fn error(&self) -> StorageError;
}

// occt: FSD_BinaryFile
/// Binary OCAF/OCCT data file reader/writer.
#[derive(Debug)]
pub struct BinaryFile {
    pub path: String,
    pub mode: Option<OpenMode>,
    buffer: Vec<u8>,
    cursor: usize,
    last_error: StorageError,
}

impl BinaryFile {
    pub fn new() -> Self {
        Self { path: String::new(), mode: None, buffer: Vec::new(), cursor: 0, last_error: StorageError::Ok }
    }

    pub fn write_integer(&mut self, v: i32) -> StorageError {
        if self.mode != Some(OpenMode::Write) && self.mode != Some(OpenMode::Append) {
            self.last_error = StorageError::WriteError;
            return self.last_error;
        }
        self.buffer.extend_from_slice(&v.to_le_bytes());
        StorageError::Ok
    }

    pub fn read_integer(&mut self) -> Result<i32, StorageError> {
        if self.cursor + 4 > self.buffer.len() { return Err(StorageError::EndOfFile); }
        let bytes: [u8; 4] = self.buffer[self.cursor..self.cursor+4].try_into().unwrap();
        self.cursor += 4;
        Ok(i32::from_le_bytes(bytes))
    }

    pub fn write_real(&mut self, v: f64) -> StorageError {
        if self.mode != Some(OpenMode::Write) && self.mode != Some(OpenMode::Append) {
            return StorageError::WriteError;
        }
        self.buffer.extend_from_slice(&v.to_le_bytes());
        StorageError::Ok
    }

    pub fn read_real(&mut self) -> Result<f64, StorageError> {
        if self.cursor + 8 > self.buffer.len() { return Err(StorageError::EndOfFile); }
        let bytes: [u8; 8] = self.buffer[self.cursor..self.cursor+8].try_into().unwrap();
        self.cursor += 8;
        Ok(f64::from_le_bytes(bytes))
    }

    pub fn buffer_size(&self) -> usize { self.buffer.len() }
    pub fn reset_cursor(&mut self) { self.cursor = 0; }
}

impl Default for BinaryFile {
    fn default() -> Self { Self::new() }
}

impl StorageDriver for BinaryFile {
    fn open(&mut self, path: &str, mode: OpenMode) -> StorageError {
        self.path = path.to_string();
        self.mode = Some(mode);
        self.buffer.clear();
        self.cursor = 0;
        self.last_error = StorageError::Ok;
        StorageError::Ok
    }

    fn close(&mut self) -> StorageError {
        self.mode = None;
        StorageError::Ok
    }

    fn is_end(&self) -> bool {
        self.cursor >= self.buffer.len()
    }

    fn error(&self) -> StorageError { self.last_error }
}

// occt: FSD_CmpFile // (compound binary file — same driver, different magic)
pub type CmpFile = BinaryFile;

// occt-ref: Storage_Schema
/// Describes the schema version of a storage file.
#[derive(Clone, Debug)]
pub struct StorageSchema {
    pub name: String,
    pub version: (u32, u32),
}

impl StorageSchema {
    pub fn new(name: impl Into<String>, major: u32, minor: u32) -> Self {
        Self { name: name.into(), version: (major, minor) }
    }

    pub fn version_string(&self) -> String {
        format!("{}.{}", self.version.0, self.version.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_file_write_read_integer() {
        let mut f = BinaryFile::new();
        f.open("test.bin", OpenMode::Write);
        assert_eq!(f.write_integer(42), StorageError::Ok);
        assert_eq!(f.buffer_size(), 4);
        f.reset_cursor();
        f.mode = Some(OpenMode::Read);
        assert_eq!(f.read_integer().unwrap(), 42);
    }

    #[test]
    fn binary_file_write_read_real() {
        let mut f = BinaryFile::new();
        f.open("test.bin", OpenMode::Write);
        f.write_real(3.14159);
        f.reset_cursor();
        f.mode = Some(OpenMode::Read);
        let v = f.read_real().unwrap();
        assert!((v - 3.14159).abs() < 1e-10);
    }

    #[test]
    fn binary_file_eof() {
        let mut f = BinaryFile::new();
        f.open("test.bin", OpenMode::Read);
        assert!(f.read_integer().is_err());
    }

    #[test]
    fn storage_schema() {
        let s = StorageSchema::new("BinMDF", 2, 10);
        assert_eq!(s.version_string(), "2.10");
    }
}
