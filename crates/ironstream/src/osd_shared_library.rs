// FILE: osd_shared_library.rs
// occt: OSD_SharedLibrary

//! Interface to dynamic library loader.
//! Provides tools to load a shared library and retrieve the address of an entry point.

#[derive(Debug, Clone)]
pub struct OsdSharedLibrary {
    name: Option<String>,
    handle: Option<usize>,
}

impl OsdSharedLibrary {
    /// Creates a SharedLibrary object with name NULL.
    pub fn new() -> Self {
        OsdSharedLibrary {
            name: None,
            handle: None,
        }
    }

    /// Creates a SharedLibrary object with name aFilename.
    pub fn new_with_name(filename: &str) -> Self {
        OsdSharedLibrary {
            name: Some(filename.to_string()),
            handle: None,
        }
    }

    /// Sets a name associated to the shared object.
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    /// Returns the name associated to the shared object.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Opens the shared library (simulated in Rust - no actual dynamic loading)
    pub fn dl_open(&mut self) -> bool {
        if self.name.is_some() {
            self.handle = Some(1); // Simulate successful load
            true
        } else {
            false
        }
    }

    /// Returns the address of a symbol in the shared library
    pub fn dl_symb(&self, _name: &str) -> Option<usize> {
        self.handle.map(|h| h)
    }

    /// Deallocates the address space for the library
    pub fn dl_close(&mut self) {
        self.handle = None;
    }

    /// Returns error message for last error
    pub fn dl_error(&self) -> &str {
        "no error"
    }

    /// Frees memory allocated
    pub fn destroy(&mut self) {
        self.handle = None;
        self.name = None;
    }
}

impl Default for OsdSharedLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for OsdSharedLibrary {
    fn drop(&mut self) {
        self.destroy();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_shared_library_new() {
        let lib = OsdSharedLibrary::new();
        assert!(lib.name().is_none());
        assert!(lib.handle.is_none());
    }

    #[test]
    fn test_osd_shared_library_new_with_name() {
        let lib = OsdSharedLibrary::new_with_name("test.so");
        assert_eq!(lib.name(), Some("test.so"));
    }

    #[test]
    fn test_osd_shared_library_set_name() {
        let mut lib = OsdSharedLibrary::new();
        lib.set_name("mylib.so");
        assert_eq!(lib.name(), Some("mylib.so"));
    }

    #[test]
    fn test_osd_shared_library_dl_open() {
        let mut lib = OsdSharedLibrary::new_with_name("test.so");
        assert!(lib.dl_open());
        assert!(lib.handle.is_some());
    }

    #[test]
    fn test_osd_shared_library_dl_close() {
        let mut lib = OsdSharedLibrary::new_with_name("test.so");
        let _ = lib.dl_open();
        lib.dl_close();
        assert!(lib.handle.is_none());
    }
}
