// FILE: image_compressed_format.rs
// occt: Image_CompressedFormat

//! Compressed pixel formats supported by graphics hardware.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompressedFormat {
    Unknown = 0,
    RgbS3tcDxt1 = 1,
    RgbaS3tcDxt1 = 2,
    RgbaS3tcDxt3 = 3,
    RgbaS3tcDxt5 = 4,
}

impl CompressedFormat {
    pub const NB: usize = 5;

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(CompressedFormat::Unknown),
            1 => Some(CompressedFormat::RgbS3tcDxt1),
            2 => Some(CompressedFormat::RgbaS3tcDxt1),
            3 => Some(CompressedFormat::RgbaS3tcDxt3),
            4 => Some(CompressedFormat::RgbaS3tcDxt5),
            _ => None,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(CompressedFormat::Unknown as u32, 0);
        assert_eq!(CompressedFormat::RgbaS3tcDxt5 as u32, 4);
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(CompressedFormat::from_u32(0), Some(CompressedFormat::Unknown));
        assert_eq!(CompressedFormat::from_u32(4), Some(CompressedFormat::RgbaS3tcDxt5));
        assert_eq!(CompressedFormat::from_u32(10), None);
    }
}
