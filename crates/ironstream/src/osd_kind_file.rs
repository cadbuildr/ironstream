// FILE: osd_kind_file.rs
// occt: OSD_KindFile

/// Specifies the type of files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindFile {
    /// Regular file
    File,
    /// Directory
    Directory,
    /// Symbolic link
    Link,
    /// Socket file
    Socket,
    /// Unknown type
    Unknown,
}

impl KindFile {
    /// Get string representation.
    pub fn as_str(&self) -> &str {
        match self {
            KindFile::File => "FILE",
            KindFile::Directory => "DIRECTORY",
            KindFile::Link => "LINK",
            KindFile::Socket => "SOCKET",
            KindFile::Unknown => "UNKNOWN",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_file_variants() {
        assert_eq!(KindFile::File.as_str(), "FILE");
        assert_eq!(KindFile::Directory.as_str(), "DIRECTORY");
        assert_eq!(KindFile::Link.as_str(), "LINK");
        assert_eq!(KindFile::Socket.as_str(), "SOCKET");
        assert_eq!(KindFile::Unknown.as_str(), "UNKNOWN");
    }

    #[test]
    fn test_equality() {
        assert_eq!(KindFile::File, KindFile::File);
        assert_ne!(KindFile::File, KindFile::Directory);
    }
}
