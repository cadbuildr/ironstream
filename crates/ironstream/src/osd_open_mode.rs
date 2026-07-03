// FILE: osd_open_mode.rs
// occt: OSD_OpenMode

/// Specifies the file open mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsdOpenMode {
    /// Read-only mode
    ReadOnly = 0,
    /// Write-only mode
    WriteOnly = 1,
    /// Read-write mode
    ReadWrite = 2,
}

impl OsdOpenMode {
    /// Converts OsdOpenMode to mode string for file operations
    pub fn to_mode_string(self) -> &'static str {
        match self {
            OsdOpenMode::ReadOnly => "r",
            OsdOpenMode::WriteOnly => "w",
            OsdOpenMode::ReadWrite => "r+",
        }
    }

    /// Creates OsdOpenMode from integer value
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(OsdOpenMode::ReadOnly),
            1 => Some(OsdOpenMode::WriteOnly),
            2 => Some(OsdOpenMode::ReadWrite),
            _ => None,
        }
    }

    /// Gets numeric value of mode
    pub fn as_int(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_open_mode_read_only() {
        let mode = OsdOpenMode::ReadOnly;
        assert_eq!(mode.to_mode_string(), "r");
        assert_eq!(mode.as_int(), 0);
    }

    #[test]
    fn test_osd_open_mode_write_only() {
        let mode = OsdOpenMode::WriteOnly;
        assert_eq!(mode.to_mode_string(), "w");
        assert_eq!(mode.as_int(), 1);
    }

    #[test]
    fn test_osd_open_mode_read_write() {
        let mode = OsdOpenMode::ReadWrite;
        assert_eq!(mode.to_mode_string(), "r+");
        assert_eq!(mode.as_int(), 2);
    }

    #[test]
    fn test_osd_open_mode_from_int() {
        assert_eq!(OsdOpenMode::from_int(0), Some(OsdOpenMode::ReadOnly));
        assert_eq!(OsdOpenMode::from_int(1), Some(OsdOpenMode::WriteOnly));
        assert_eq!(OsdOpenMode::from_int(2), Some(OsdOpenMode::ReadWrite));
        assert_eq!(OsdOpenMode::from_int(3), None);
    }
}
