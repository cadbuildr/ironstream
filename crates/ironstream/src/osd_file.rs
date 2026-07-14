// FILE: osd_file.rs
// occt: OSD_File, OSD_FileIterator
// occt-ref: OSD_Path, OSD_Directory

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsdOpenMode {
    In,         // read only
    Out,        // write only
    Append,
    InOut,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsdError {
    Ok,
    FileNotFound,
    AccessDenied,
    InvalidPath,
    Busy,
    IsDirectory,
    IsFile,
    NoSpace,
    Unknown,
}

// occt-ref: OSD_Path
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct OsdPath {
    pub path: String,
}

impl OsdPath {
    pub fn new(path: impl Into<String>) -> Self { Self { path: path.into() } }

    pub fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
            .rsplit('\\').next().unwrap_or(&self.path)
    }

    pub fn extension(&self) -> &str {
        let name = self.name();
        if let Some(dot) = name.rfind('.') { &name[dot+1..] } else { "" }
    }

    pub fn directory(&self) -> &str {
        let n = self.name();
        let dir_len = self.path.len().saturating_sub(n.len());
        let d = &self.path[..dir_len];
        if d.ends_with('/') || d.ends_with('\\') { &d[..d.len()-1] } else { d }
    }

    pub fn is_absolute(&self) -> bool {
        self.path.starts_with('/') || self.path.len() >= 2 && &self.path[1..2] == ":"
    }

    pub fn append(&self, child: &str) -> OsdPath {
        if self.path.is_empty() { return OsdPath::new(child); }
        let sep = if self.path.ends_with('/') || self.path.ends_with('\\') { "" } else { "/" };
        OsdPath::new(format!("{}{}{}", self.path, sep, child))
    }

    pub fn is_empty(&self) -> bool { self.path.is_empty() }
}

// occt: OSD_File
#[derive(Clone, Debug)]
pub struct OsdFile {
    pub path: OsdPath,
    pub is_open: bool,
    pub open_mode: Option<OsdOpenMode>,
    pub error: OsdError,
    pub size_bytes: u64,
    pub content: Vec<u8>,
}

impl Default for OsdFile {
    fn default() -> Self {
        Self { path: OsdPath::default(), is_open: false, open_mode: None, error: OsdError::Ok, size_bytes: 0, content: Vec::new() }
    }
}

impl OsdFile {
    pub fn new(path: OsdPath) -> Self { Self { path, error: OsdError::Ok, ..Default::default() } }

    pub fn open(&mut self, mode: OsdOpenMode) {
        if self.path.is_empty() { self.error = OsdError::InvalidPath; return; }
        self.open_mode = Some(mode);
        self.is_open = true;
        self.error = OsdError::Ok;
    }

    pub fn close(&mut self) { self.is_open = false; self.open_mode = None; }

    pub fn write_bytes(&mut self, data: &[u8]) {
        if !self.is_open || self.open_mode == Some(OsdOpenMode::In) {
            self.error = OsdError::AccessDenied; return;
        }
        self.content.extend_from_slice(data);
        self.size_bytes = self.content.len() as u64;
        self.error = OsdError::Ok;
    }

    pub fn read_bytes(&mut self, buf: &mut Vec<u8>, n: usize) {
        if !self.is_open { self.error = OsdError::AccessDenied; return; }
        let take = n.min(self.content.len());
        buf.extend_from_slice(&self.content[..take]);
        self.error = OsdError::Ok;
    }

    pub fn is_readable(&self) -> bool { self.open_mode.map(|m| m != OsdOpenMode::Out).unwrap_or(false) }
    pub fn is_writable(&self) -> bool { self.open_mode.map(|m| m != OsdOpenMode::In).unwrap_or(false) }
    pub fn failed(&self) -> bool { self.error != OsdError::Ok }
}

// occt-ref: OSD_Directory
#[derive(Clone, Debug)]
pub struct OsdDirectory {
    pub path: OsdPath,
    pub error: OsdError,
}

impl Default for OsdDirectory {
    fn default() -> Self { Self { path: OsdPath::default(), error: OsdError::Ok } }
}

impl OsdDirectory {
    pub fn new(path: OsdPath) -> Self { Self { path, error: OsdError::Ok } }

    pub fn create(&mut self) {
        if self.path.is_empty() { self.error = OsdError::InvalidPath; return; }
        self.error = OsdError::Ok;
    }

    pub fn destroy(&mut self) { self.error = OsdError::Ok; }
    pub fn exists(&self) -> bool { !self.path.is_empty() }
    pub fn failed(&self) -> bool { self.error != OsdError::Ok }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn osd_path_name_extension() {
        let p = OsdPath::new("/usr/local/file.step");
        assert_eq!(p.name(), "file.step");
        assert_eq!(p.extension(), "step");
        assert_eq!(p.directory(), "/usr/local");
        assert!(p.is_absolute());
    }

    #[test]
    fn osd_path_append() {
        let p = OsdPath::new("/data");
        let child = p.append("model.stp");
        assert_eq!(child.path, "/data/model.stp");
    }

    #[test]
    fn osd_file_write_read() {
        let mut f = OsdFile::new(OsdPath::new("test.bin"));
        f.open(OsdOpenMode::InOut);
        assert!(f.is_open);
        f.write_bytes(b"hello");
        assert_eq!(f.size_bytes, 5);
        assert!(!f.failed());
        f.close();
        assert!(!f.is_open);
    }

    #[test]
    fn osd_directory_basic() {
        let mut d = OsdDirectory::new(OsdPath::new("/tmp/testdir"));
        d.create();
        assert!(d.exists());
        assert!(!d.failed());
    }
}
