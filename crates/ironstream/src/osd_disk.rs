// FILE: osd_disk.rs
// occt: OSD_Disk

/// Disk/volume information.
#[derive(Clone, Debug)]
pub struct Disk {
    name: String,
}

impl Disk {
    /// Create disk from name.
    pub fn new(name: &str) -> Self {
        Disk {
            name: name.to_string(),
        }
    }

    /// Get disk name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get available space (placeholder).
    pub fn free_space(&self) -> u64 {
        0
    }

    /// Get total space (placeholder).
    pub fn total_space(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_disk() {
        let disk = Disk::new("/");
        assert_eq!(disk.name(), "/");
    }
}
