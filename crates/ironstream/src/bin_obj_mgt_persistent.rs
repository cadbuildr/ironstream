// FILE: bin_obj_mgt_persistent.rs
// occt: BinObjMgt_Persistent

/// Binary persistent representation of an object.
/// Used as a buffer for read/write operations with support for little/big endian conversion.
pub struct BinObjMgtPersistent {
    id: i32,
    type_id: i32,
    data: Vec<u8>,
    position: usize,
    is_error: bool,
}

impl BinObjMgtPersistent {
    /// Creates an empty persistent object.
    pub fn new() -> Self {
        BinObjMgtPersistent {
            id: 0,
            type_id: 0,
            data: Vec::new(),
            position: 0,
            is_error: false,
        }
    }

    /// Returns the current position for get/put operations.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Sets the current position for get/put operations.
    pub fn set_position(&mut self, pos: usize) -> bool {
        if pos <= self.data.len() {
            self.position = pos;
            self.is_error = false;
            true
        } else {
            self.is_error = true;
            false
        }
    }

    /// Truncates the buffer by current position.
    pub fn truncate(&mut self) {
        self.data.truncate(self.position);
    }

    /// Indicates an error after Get methods or SetPosition.
    pub fn is_error(&self) -> bool {
        self.is_error
    }

    /// Indicates a good state after Get methods or SetPosition.
    pub fn is_ok(&self) -> bool {
        !self.is_error
    }

    /// Initializes the object to reuse again.
    pub fn init(&mut self) {
        self.id = 0;
        self.type_id = 0;
        self.data.clear();
        self.position = 0;
        self.is_error = false;
    }

    /// Sets the Id of the object.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Sets the Id of the type of the object.
    pub fn set_type_id(&mut self, type_id: i32) {
        self.type_id = type_id;
    }

    /// Returns the Id of the object.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Returns the Id of the type of the object.
    pub fn type_id(&self) -> i32 {
        self.type_id
    }

    /// Returns the length of data.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Frees the allocated memory.
    pub fn destroy(&mut self) {
        self.data.clear();
    }
}

impl Default for BinObjMgtPersistent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistent_creation() {
        let p = BinObjMgtPersistent::new();
        assert_eq!(p.id(), 0);
        assert_eq!(p.type_id(), 0);
        assert_eq!(p.length(), 0);
        assert!(p.is_ok());
    }

    #[test]
    fn test_persistent_set_id() {
        let mut p = BinObjMgtPersistent::new();
        p.set_id(42);
        assert_eq!(p.id(), 42);
    }

    #[test]
    fn test_persistent_set_type_id() {
        let mut p = BinObjMgtPersistent::new();
        p.set_type_id(100);
        assert_eq!(p.type_id(), 100);
    }

    #[test]
    fn test_persistent_position() {
        let mut p = BinObjMgtPersistent::new();
        p.data.resize(10, 0);
        assert_eq!(p.position(), 0);
        assert!(p.set_position(5));
        assert_eq!(p.position(), 5);
        assert!(!p.is_error());
    }

    #[test]
    fn test_persistent_invalid_position() {
        let mut p = BinObjMgtPersistent::new();
        p.data.resize(10, 0);
        assert!(!p.set_position(20));
        assert!(p.is_error());
    }

    #[test]
    fn test_persistent_init() {
        let mut p = BinObjMgtPersistent::new();
        p.set_id(42);
        p.set_type_id(100);
        p.data.resize(10, 1);
        p.init();
        assert_eq!(p.id(), 0);
        assert_eq!(p.type_id(), 0);
        assert_eq!(p.length(), 0);
        assert_eq!(p.position(), 0);
        assert!(p.is_ok());
    }

    #[test]
    fn test_persistent_destroy() {
        let mut p = BinObjMgtPersistent::new();
        p.data.resize(10, 1);
        p.destroy();
        assert_eq!(p.length(), 0);
    }

    #[test]
    fn test_persistent_truncate() {
        let mut p = BinObjMgtPersistent::new();
        p.data.resize(10, 1);
        assert!(p.set_position(5));
        p.truncate();
        assert_eq!(p.length(), 5);
    }
}
