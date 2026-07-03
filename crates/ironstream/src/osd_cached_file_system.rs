// FILE: osd_cached_file_system.rs
// occt: OSD_CachedFileSystem

/// Cached file system wrapper for performance.
use std::collections::HashMap;

pub struct CachedFileSystem {
    cache: std::sync::Mutex<HashMap<String, bool>>,
}

impl CachedFileSystem {
    /// Create new cached file system.
    pub fn new() -> Self {
        CachedFileSystem {
            cache: std::sync::Mutex::new(HashMap::new()),
        }
    }

    /// Check if path exists (with caching).
    pub fn exists(&self, path: &str) -> bool {
        if let Ok(mut cache) = self.cache.lock() {
            if let Some(&result) = cache.get(path) {
                return result;
            }
            let exists = std::path::Path::new(path).exists();
            cache.insert(path.to_string(), exists);
            exists
        } else {
            std::path::Path::new(path).exists()
        }
    }

    /// Clear cache.
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Get cache size.
    pub fn cache_size(&self) -> usize {
        if let Ok(cache) = self.cache.lock() {
            cache.len()
        } else {
            0
        }
    }
}

impl Default for CachedFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cached_fs() {
        let fs = CachedFileSystem::new();
        let _ = fs.exists("/tmp");
        assert!(fs.cache_size() > 0);
    }

    #[test]
    fn test_clear_cache() {
        let fs = CachedFileSystem::new();
        let _ = fs.exists("/tmp");
        fs.clear_cache();
        assert_eq!(fs.cache_size(), 0);
    }
}
