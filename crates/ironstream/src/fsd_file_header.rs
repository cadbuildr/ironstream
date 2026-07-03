// FILE: fsd_file_header.rs
// occt: FSD_FileHeader

/// File header for storage format.
#[derive(Debug, Clone)]
pub struct FsdFileHeader {
    magic: u32,
    version: u32,
    flags: u32,
}

impl FsdFileHeader {
    /// Constructs a default file header.
    pub fn new() -> Self {
        Self {
            magic: 0xCADCAD00,
            version: 1,
            flags: 0,
        }
    }

    /// Constructs a file header with explicit values.
    pub fn with_values(magic: u32, version: u32, flags: u32) -> Self {
        Self {
            magic,
            version,
            flags,
        }
    }

    /// Returns the magic number.
    pub fn magic(&self) -> u32 {
        self.magic
    }

    /// Sets the magic number.
    pub fn set_magic(&mut self, magic: u32) {
        self.magic = magic;
    }

    /// Returns the version.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Sets the version.
    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    /// Returns the flags.
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// Sets the flags.
    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }

    /// Checks if header is valid.
    pub fn is_valid(&self) -> bool {
        self.magic == 0xCADCAD00 && self.version > 0
    }

    /// Serializes header to bytes.
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut result = [0u8; 12];
        result[0..4].copy_from_slice(&self.magic.to_le_bytes());
        result[4..8].copy_from_slice(&self.version.to_le_bytes());
        result[8..12].copy_from_slice(&self.flags.to_le_bytes());
        result
    }

    /// Deserializes header from bytes.
    pub fn from_bytes(bytes: &[u8; 12]) -> Self {
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let flags = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        Self {
            magic,
            version,
            flags,
        }
    }
}

impl Default for FsdFileHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let hdr = FsdFileHeader::new();
        assert_eq!(hdr.magic(), 0xCADCAD00);
        assert_eq!(hdr.version(), 1);
        assert_eq!(hdr.flags(), 0);
        assert!(hdr.is_valid());
    }

    #[test]
    fn test_with_values() {
        let hdr = FsdFileHeader::with_values(0x12345678, 2, 0xFF);
        assert_eq!(hdr.magic(), 0x12345678);
        assert_eq!(hdr.version(), 2);
        assert_eq!(hdr.flags(), 0xFF);
    }

    #[test]
    fn test_serialization() {
        let hdr = FsdFileHeader::with_values(0xCADCAD00, 1, 5);
        let bytes = hdr.to_bytes();
        let hdr2 = FsdFileHeader::from_bytes(&bytes);
        assert_eq!(hdr2.magic(), hdr.magic());
        assert_eq!(hdr2.version(), hdr.version());
        assert_eq!(hdr2.flags(), hdr.flags());
    }
}
