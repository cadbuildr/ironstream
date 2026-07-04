// FILE: ldom_mem_manager.rs
// occt: LDOM_MemManager

use std::collections::HashMap;

/// Memory manager for LDOM documents.
/// Handles allocation of memory blocks and string hashing.
pub struct LDOMMemManager {
    block_size: usize,
    allocated_blocks: Vec<Vec<u8>>,
    string_cache: HashMap<String, String>,
    root_element: Option<String>,
}

impl LDOMMemManager {
    /// Constructor
    pub fn new(block_size: usize) -> Self {
        LDOMMemManager {
            block_size,
            allocated_blocks: Vec::new(),
            string_cache: HashMap::new(),
            root_element: None,
        }
    }

    /// General memory allocator
    pub fn allocate(&mut self, size: usize) -> usize {
        if size == 0 {
            return 0;
        }

        let block = vec![0u8; size];
        let ptr = self.allocated_blocks.len();
        self.allocated_blocks.push(block);
        ptr
    }

    /// Memory allocation with access via hash table
    pub fn hashed_allocate(&mut self, string: &str) -> (String, i32) {
        let hash = Self::hash(string);

        if let Some(existing) = self.string_cache.get(string) {
            return (existing.clone(), hash);
        }

        self.string_cache.insert(string.to_string(), string.to_string());
        (string.to_string(), hash)
    }

    /// Compute hash of a string
    pub fn hash(string: &str) -> i32 {
        let mut hash: i32 = 0;
        for (i, c) in string.as_bytes().iter().enumerate() {
            hash = hash.wrapping_mul(31).wrapping_add(*c as i32);
            if i > 100 {
                break; // Limit hash computation
            }
        }
        hash
    }

    /// Get the root element
    pub fn root_element(&self) -> Option<&str> {
        self.root_element.as_deref()
    }

    /// Set the root element
    pub fn set_root_element(&mut self, name: &str) {
        self.root_element = Some(name.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_manager_creation() {
        let mgr = LDOMMemManager::new(1024);
        assert_eq!(mgr.block_size, 1024);
    }

    #[test]
    fn test_allocate() {
        let mut mgr = LDOMMemManager::new(1024);
        let ptr1 = mgr.allocate(100);
        let ptr2 = mgr.allocate(200);
        assert_ne!(ptr1, ptr2);
    }

    #[test]
    fn test_hashed_allocate() {
        let mut mgr = LDOMMemManager::new(1024);
        let (s1, hash1) = mgr.hashed_allocate("test");
        let (s2, hash2) = mgr.hashed_allocate("test");
        assert_eq!(s1, s2);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash() {
        let hash1 = LDOMMemManager::hash("hello");
        let hash2 = LDOMMemManager::hash("hello");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_root_element() {
        let mut mgr = LDOMMemManager::new(1024);
        mgr.set_root_element("root");
        assert_eq!(mgr.root_element(), Some("root"));
    }
}
