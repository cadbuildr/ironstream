// FILE: osd_sys_type.rs
// occt: OSD_SysType

/// Set of possible system types.
/// 'Default' means SysType of machine operating this process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsdSysType {
    /// Unknown system
    Unknown = 0,
    /// Default system (current machine)
    Default = 1,
    /// UNIX BSD-like system
    UnixBsd = 2,
    /// UNIX System V-like system
    UnixSystemV = 3,
    /// VMS system
    Vms = 4,
    /// OS/2 system
    Os2 = 5,
    /// OSF system
    Osf = 6,
    /// macOS system
    MacOs = 7,
    /// Taligent system
    Taligent = 8,
    /// Windows NT system
    WindowsNt = 9,
    /// Linux RedHat system
    LinuxRedhat = 10,
    /// AIX system
    Aix = 11,
}

impl OsdSysType {
    /// Creates OsdSysType from integer value
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(OsdSysType::Unknown),
            1 => Some(OsdSysType::Default),
            2 => Some(OsdSysType::UnixBsd),
            3 => Some(OsdSysType::UnixSystemV),
            4 => Some(OsdSysType::Vms),
            5 => Some(OsdSysType::Os2),
            6 => Some(OsdSysType::Osf),
            7 => Some(OsdSysType::MacOs),
            8 => Some(OsdSysType::Taligent),
            9 => Some(OsdSysType::WindowsNt),
            10 => Some(OsdSysType::LinuxRedhat),
            11 => Some(OsdSysType::Aix),
            _ => None,
        }
    }

    /// Gets numeric value of type
    pub fn as_int(self) -> i32 {
        self as i32
    }

    /// Returns true if this is a UNIX-like system
    pub fn is_unix_like(self) -> bool {
        matches!(self, OsdSysType::UnixBsd | OsdSysType::UnixSystemV)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sys_type_values() {
        assert_eq!(OsdSysType::Unknown.as_int(), 0);
        assert_eq!(OsdSysType::Default.as_int(), 1);
        assert_eq!(OsdSysType::UnixBsd.as_int(), 2);
        assert_eq!(OsdSysType::WindowsNt.as_int(), 9);
    }

    #[test]
    fn test_osd_sys_type_from_int() {
        assert_eq!(OsdSysType::from_int(0), Some(OsdSysType::Unknown));
        assert_eq!(OsdSysType::from_int(9), Some(OsdSysType::WindowsNt));
        assert_eq!(OsdSysType::from_int(12), None);
    }

    #[test]
    fn test_osd_sys_type_is_unix_like() {
        assert!(OsdSysType::UnixBsd.is_unix_like());
        assert!(OsdSysType::UnixSystemV.is_unix_like());
        assert!(!OsdSysType::WindowsNt.is_unix_like());
        assert!(!OsdSysType::Unknown.is_unix_like());
    }
}
