// FILE: storage_data.rs
// occt: Storage_Schema, Storage_Data, Storage_BaseDriver,
//       Storage_Error, Storage_HeaderData

/// Storage operation error code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageError {
    Ok,
    OpenError,
    CloseError,
    MixingDrivers,
    BadConfiguration,
    PersistentTypeMismatch,
    WrongFileDriver,
    WriteError,
    ReadError,
    NotFound,
    UnknownError,
}

impl Default for StorageError {
    fn default() -> Self { Self::Ok }
}

impl StorageError {
    pub fn is_ok(&self) -> bool { *self == StorageError::Ok }
    pub fn message(&self) -> &'static str {
        match self {
            Self::Ok                      => "Ok",
            Self::OpenError               => "Open error",
            Self::CloseError              => "Close error",
            Self::MixingDrivers          => "Mixing drivers",
            Self::BadConfiguration       => "Bad configuration",
            Self::PersistentTypeMismatch => "Persistent type mismatch",
            Self::WrongFileDriver        => "Wrong file driver",
            Self::WriteError             => "Write error",
            Self::ReadError              => "Read error",
            Self::NotFound               => "Not found",
            Self::UnknownError           => "Unknown error",
        }
    }
}

// occt: Storage_HeaderData — document header info
#[derive(Clone, Debug, Default)]
pub struct StorageHeaderData {
    pub creation_date: String,
    pub storage_version: String,
    pub schema_name: String,
    pub schema_version: String,
    pub application_name: String,
    pub application_version: String,
    pub data_type: String,
    pub user_info: Vec<(String, String)>,
    pub nb_objects: usize,
}

impl StorageHeaderData {
    pub fn new() -> Self {
        Self {
            creation_date: String::new(),
            storage_version: "1.0".to_owned(),
            schema_name: "Standard".to_owned(),
            schema_version: "1".to_owned(),
            ..Default::default()
        }
    }

    pub fn set_application(&mut self, name: &str, version: &str) {
        self.application_name = name.to_owned();
        self.application_version = version.to_owned();
    }

    pub fn add_to_user_info(&mut self, key: &str, value: &str) {
        self.user_info.push((key.to_owned(), value.to_owned()));
    }

    pub fn find_user_info(&self, key: &str) -> Option<&str> {
        self.user_info.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
}

// occt: Storage_Data — holds persistent object data
#[derive(Clone, Debug, Default)]
pub struct StorageData {
    pub header: StorageHeaderData,
    pub roots: Vec<String>,
    pub types: Vec<String>,
    pub error: StorageError,
}

impl StorageData {
    pub fn new() -> Self { Self::default() }

    pub fn add_root(&mut self, name: &str) {
        if !self.roots.contains(&name.to_owned()) {
            self.roots.push(name.to_owned());
        }
    }

    pub fn add_type(&mut self, type_name: &str) {
        if !self.types.contains(&type_name.to_owned()) {
            self.types.push(type_name.to_owned());
        }
    }

    pub fn remove_root(&mut self, name: &str) {
        self.roots.retain(|r| r != name);
    }

    pub fn is_root(&self, name: &str) -> bool { self.roots.iter().any(|r| r == name) }
    pub fn nb_roots(&self) -> usize { self.roots.len() }
    pub fn nb_types(&self) -> usize { self.types.len() }
    pub fn has_errors(&self) -> bool { !self.error.is_ok() }
}

/// Access mode for file drivers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageAccessMode {
    Read,
    Write,
    Append,
}

impl Default for StorageAccessMode {
    fn default() -> Self { Self::Read }
}

// occt: Storage_BaseDriver — base for file I/O drivers
#[derive(Clone, Debug)]
pub struct StorageBaseDriver {
    pub path: String,
    pub mode: StorageAccessMode,
    pub is_open: bool,
    pub error: StorageError,
    pub read_bytes: usize,
    pub written_bytes: usize,
}

impl StorageBaseDriver {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            mode: StorageAccessMode::Read,
            is_open: false,
            error: StorageError::Ok,
            read_bytes: 0,
            written_bytes: 0,
        }
    }

    pub fn open(&mut self, path: &str, mode: StorageAccessMode) -> StorageError {
        self.path = path.to_owned();
        self.mode = mode;
        self.is_open = true;
        self.error = StorageError::Ok;
        StorageError::Ok
    }

    pub fn close(&mut self) -> StorageError {
        if !self.is_open {
            self.error = StorageError::CloseError;
            return self.error;
        }
        self.is_open = false;
        self.error = StorageError::Ok;
        StorageError::Ok
    }

    pub fn read_real(&mut self) -> f64 {
        self.read_bytes += 8;
        0.0
    }

    pub fn write_real(&mut self, _v: f64) {
        self.written_bytes += 8;
    }

    pub fn read_integer(&mut self) -> i32 {
        self.read_bytes += 4;
        0
    }

    pub fn write_integer(&mut self, _v: i32) {
        self.written_bytes += 4;
    }

    pub fn read_string(&mut self) -> String {
        self.read_bytes += 4;
        String::new()
    }

    pub fn write_string(&mut self, s: &str) {
        self.written_bytes += s.len() + 4;
    }

    pub fn is_end(&self) -> bool { !self.is_open }
}

impl Default for StorageBaseDriver {
    fn default() -> Self { Self::new() }
}

// occt: Storage_Schema — registry of type names for serialization
#[derive(Clone, Debug, Default)]
pub struct StorageSchema {
    pub name: String,
    pub version: String,
    pub registered_types: Vec<String>,
}

impl StorageSchema {
    pub fn new(name: &str, version: &str) -> Self {
        Self { name: name.to_owned(), version: version.to_owned(), ..Default::default() }
    }

    pub fn register(&mut self, type_name: &str) {
        if !self.registered_types.contains(&type_name.to_owned()) {
            self.registered_types.push(type_name.to_owned());
        }
    }

    pub fn is_registered(&self, type_name: &str) -> bool {
        self.registered_types.iter().any(|t| t == type_name)
    }

    pub fn nb_types(&self) -> usize { self.registered_types.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_error() {
        assert!(StorageError::Ok.is_ok());
        assert!(!StorageError::ReadError.is_ok());
    }

    #[test]
    fn storage_header() {
        let mut h = StorageHeaderData::new();
        h.set_application("TestApp", "1.0");
        h.add_to_user_info("author", "Alice");
        assert_eq!(h.find_user_info("author"), Some("Alice"));
        assert_eq!(h.application_name, "TestApp");
    }

    #[test]
    fn storage_data_roots() {
        let mut d = StorageData::new();
        d.add_root("Root1");
        d.add_root("Root2");
        d.add_root("Root1"); // duplicate
        assert_eq!(d.nb_roots(), 2);
        assert!(d.is_root("Root1"));
        d.remove_root("Root1");
        assert!(!d.is_root("Root1"));
    }

    #[test]
    fn storage_driver_open_close() {
        let mut drv = StorageBaseDriver::new();
        let err = drv.open("/tmp/test.cbf", StorageAccessMode::Write);
        assert!(err.is_ok());
        assert!(drv.is_open);
        drv.write_real(3.14);
        assert_eq!(drv.written_bytes, 8);
        let err2 = drv.close();
        assert!(err2.is_ok());
        assert!(!drv.is_open);
    }

    #[test]
    fn storage_schema() {
        let mut schema = StorageSchema::new("Standard", "1");
        schema.register("gp_Pnt");
        schema.register("gp_Vec");
        schema.register("gp_Pnt"); // duplicate
        assert_eq!(schema.nb_types(), 2);
        assert!(schema.is_registered("gp_Vec"));
    }
}
