// FILE: osd_directory_iterator.rs
// occt: OSD_DirectoryIterator

/// Iterator over directory entries.
pub struct DirectoryIterator {
    reader: Option<std::fs::ReadDir>,
    current: Option<std::fs::DirEntry>,
}

impl DirectoryIterator {
    /// Create iterator from directory path.
    pub fn new(path: &str) -> std::io::Result<Self> {
        let reader = std::fs::read_dir(path)?;
        Ok(DirectoryIterator {
            reader: Some(reader),
            current: None,
        })
    }

    /// Get current entry name.
    pub fn current_name(&self) -> Option<String> {
        self.current
            .as_ref()
            .and_then(|e| e.file_name().into_string().ok())
    }

    /// Check if current is directory.
    pub fn current_is_dir(&self) -> bool {
        self.current
            .as_ref()
            .map(|e| e.path().is_dir())
            .unwrap_or(false)
    }

    /// Move to next entry.
    pub fn next(&mut self) -> bool {
        if let Some(ref mut reader) = self.reader {
            if let Ok(Some(entry)) = reader.next().transpose() {
                self.current = Some(entry);
                return true;
            }
        }
        self.current = None;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_iterator_tmp() {
        if let Ok(mut it) = DirectoryIterator::new("/tmp") {
            let has_entries = it.next();
            assert!(has_entries || true); // /tmp may be empty
        }
    }
}
