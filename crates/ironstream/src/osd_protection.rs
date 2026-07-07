// FILE: osd_protection.rs
// occt: OSD_Protection

/// File protection flags.
#[derive(Clone, Copy, Debug)]
pub struct Protection {
    pub user: u32,
    pub group: u32,
    pub other: u32,
    pub system: u32,
}

impl Protection {
    pub fn new() -> Self {
        Self {
            user: 7,
            group: 5,
            other: 5,
            system: 0,
        }
    }

    pub fn read_write_all() -> Self {
        Self {
            user: 7,
            group: 7,
            other: 7,
            system: 0,
        }
    }

    pub fn read_only() -> Self {
        Self {
            user: 5,
            group: 5,
            other: 5,
            system: 0,
        }
    }
}

impl Default for Protection {
    fn default() -> Self {
        Self::new()
    }
}
