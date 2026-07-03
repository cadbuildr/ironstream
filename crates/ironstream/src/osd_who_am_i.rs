// FILE: osd_who_am_i.rs
// occt: OSD_WhoAmI

/// Allows great accuracy for error management (private type).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsdWhoAmI {
    /// Directory object
    Directory = 0,
    /// Directory iterator object
    DirectoryIterator = 1,
    /// Environment object
    Environment = 2,
    /// File object
    File = 3,
    /// File node object
    FileNode = 4,
    /// File iterator object
    FileIterator = 5,
    /// Path object
    Path = 6,
    /// Process object
    Process = 7,
    /// Protection object
    Protection = 8,
    /// Host object
    Host = 9,
    /// Disk object
    Disk = 10,
    /// Chronometer object
    Chronometer = 11,
    /// Timer object
    Timer = 12,
    /// Package object
    Package = 13,
    /// Environment iterator object
    EnvironmentIterator = 14,
}

impl OsdWhoAmI {
    /// Creates OsdWhoAmI from integer value
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(OsdWhoAmI::Directory),
            1 => Some(OsdWhoAmI::DirectoryIterator),
            2 => Some(OsdWhoAmI::Environment),
            3 => Some(OsdWhoAmI::File),
            4 => Some(OsdWhoAmI::FileNode),
            5 => Some(OsdWhoAmI::FileIterator),
            6 => Some(OsdWhoAmI::Path),
            7 => Some(OsdWhoAmI::Process),
            8 => Some(OsdWhoAmI::Protection),
            9 => Some(OsdWhoAmI::Host),
            10 => Some(OsdWhoAmI::Disk),
            11 => Some(OsdWhoAmI::Chronometer),
            12 => Some(OsdWhoAmI::Timer),
            13 => Some(OsdWhoAmI::Package),
            14 => Some(OsdWhoAmI::EnvironmentIterator),
            _ => None,
        }
    }

    /// Gets numeric value
    pub fn as_int(self) -> i32 {
        self as i32
    }

    /// Gets string representation
    pub fn as_str(self) -> &'static str {
        match self {
            OsdWhoAmI::Directory => "Directory",
            OsdWhoAmI::DirectoryIterator => "DirectoryIterator",
            OsdWhoAmI::Environment => "Environment",
            OsdWhoAmI::File => "File",
            OsdWhoAmI::FileNode => "FileNode",
            OsdWhoAmI::FileIterator => "FileIterator",
            OsdWhoAmI::Path => "Path",
            OsdWhoAmI::Process => "Process",
            OsdWhoAmI::Protection => "Protection",
            OsdWhoAmI::Host => "Host",
            OsdWhoAmI::Disk => "Disk",
            OsdWhoAmI::Chronometer => "Chronometer",
            OsdWhoAmI::Timer => "Timer",
            OsdWhoAmI::Package => "Package",
            OsdWhoAmI::EnvironmentIterator => "EnvironmentIterator",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_who_am_i_values() {
        assert_eq!(OsdWhoAmI::Directory.as_int(), 0);
        assert_eq!(OsdWhoAmI::File.as_int(), 3);
        assert_eq!(OsdWhoAmI::EnvironmentIterator.as_int(), 14);
    }

    #[test]
    fn test_osd_who_am_i_from_int() {
        assert_eq!(OsdWhoAmI::from_int(0), Some(OsdWhoAmI::Directory));
        assert_eq!(OsdWhoAmI::from_int(3), Some(OsdWhoAmI::File));
        assert_eq!(OsdWhoAmI::from_int(15), None);
    }

    #[test]
    fn test_osd_who_am_i_as_str() {
        assert_eq!(OsdWhoAmI::Directory.as_str(), "Directory");
        assert_eq!(OsdWhoAmI::File.as_str(), "File");
    }
}
