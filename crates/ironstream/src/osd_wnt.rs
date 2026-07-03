// FILE: osd_wnt.rs
// occt: OSD_WNT

/// Windows NT specific declarations and utilities.
/// This module provides platform-specific interfaces for Windows.

/// Directory response codes for Windows operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirResponse {
    /// Abort operation
    Abort = 0,
    /// Retry operation
    Retry = 1,
    /// Ignore error
    Ignore = 2,
}

impl DirResponse {
    /// Creates DirResponse from integer value
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(DirResponse::Abort),
            1 => Some(DirResponse::Retry),
            2 => Some(DirResponse::Ignore),
            _ => None,
        }
    }

    /// Gets numeric value
    pub fn as_int(self) -> i32 {
        self as i32
    }
}

/// File flags for Windows file operations
pub const FLAG_READ_PIPE: u32 = 0x00000001;
pub const FLAG_EOF: u32 = 0x00000002;
pub const FLAG_FILE: u32 = 0x00000004;
pub const FLAG_DIRECTORY: u32 = 0x00000008;
pub const FLAG_PIPE: u32 = 0x00000010;
pub const FLAG_SOCKET: u32 = 0x00000020;
pub const FLAG_NAMED_PIPE: u32 = 0x00000040;
pub const FLAG_DEVICE: u32 = 0x00000080;
pub const FLAG_TYPE: u32 = 0x0000007C;

/// Macro for low 32-bit of a 64-bit value
pub fn lodword(val: u64) -> u32 {
    (val & 0x00000000FFFFFFFF) as u32
}

/// Macro for high 32-bit of a 64-bit value
pub fn hidword(val: u64) -> u32 {
    ((val >> 32) & 0x00000000FFFFFFFF) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_response_values() {
        assert_eq!(DirResponse::Abort.as_int(), 0);
        assert_eq!(DirResponse::Retry.as_int(), 1);
        assert_eq!(DirResponse::Ignore.as_int(), 2);
    }

    #[test]
    fn test_dir_response_from_int() {
        assert_eq!(DirResponse::from_int(0), Some(DirResponse::Abort));
        assert_eq!(DirResponse::from_int(1), Some(DirResponse::Retry));
        assert_eq!(DirResponse::from_int(3), None);
    }

    #[test]
    fn test_file_flags() {
        assert_eq!(FLAG_READ_PIPE, 0x00000001);
        assert_eq!(FLAG_FILE, 0x00000004);
        assert_eq!(FLAG_DIRECTORY, 0x00000008);
    }

    #[test]
    fn test_lodword() {
        let val: u64 = 0x00000000FFFFFFFF;
        assert_eq!(lodword(val), 0xFFFFFFFF);
    }

    #[test]
    fn test_hidword() {
        let val: u64 = 0xFFFFFFFF00000000;
        assert_eq!(hidword(val), 0xFFFFFFFF);
    }
}
