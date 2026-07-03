// FILE: v3d_stereo_dump_options.rs
// occt: V3d_StereoDumpOptions

/// Options to be used with image dumping.
/// Notice that the value will have no effect with disabled stereo output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V3dStereoDumpOptions {
    /// Ignore stereo mode and dump monographic projection for stereo camera
    Mono = 0,
    /// Dump only left eye projection for stereo camera
    LeftEye = 1,
    /// Dump only right eye projection for stereo camera
    RightEye = 2,
    /// Dump blended pair specific to the active device output Graphic3d_StereoMode
    /// (result will be undefined for modes like Graphic3d_StereoMode_QuadBuffer)
    Blended = 3,
}

impl V3dStereoDumpOptions {
    /// Convert from integer to enum variant
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(V3dStereoDumpOptions::Mono),
            1 => Some(V3dStereoDumpOptions::LeftEye),
            2 => Some(V3dStereoDumpOptions::RightEye),
            3 => Some(V3dStereoDumpOptions::Blended),
            _ => None,
        }
    }

    /// Convert to integer value
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(V3dStereoDumpOptions::Mono as i32, 0);
        assert_eq!(V3dStereoDumpOptions::LeftEye as i32, 1);
        assert_eq!(V3dStereoDumpOptions::RightEye as i32, 2);
        assert_eq!(V3dStereoDumpOptions::Blended as i32, 3);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            V3dStereoDumpOptions::from_i32(0),
            Some(V3dStereoDumpOptions::Mono)
        );
        assert_eq!(
            V3dStereoDumpOptions::from_i32(1),
            Some(V3dStereoDumpOptions::LeftEye)
        );
        assert_eq!(
            V3dStereoDumpOptions::from_i32(2),
            Some(V3dStereoDumpOptions::RightEye)
        );
        assert_eq!(
            V3dStereoDumpOptions::from_i32(3),
            Some(V3dStereoDumpOptions::Blended)
        );
        assert_eq!(V3dStereoDumpOptions::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(V3dStereoDumpOptions::Mono.to_i32(), 0);
        assert_eq!(V3dStereoDumpOptions::LeftEye.to_i32(), 1);
        assert_eq!(V3dStereoDumpOptions::RightEye.to_i32(), 2);
        assert_eq!(V3dStereoDumpOptions::Blended.to_i32(), 3);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for i in 0..=3 {
            if let Some(variant) = V3dStereoDumpOptions::from_i32(i) {
                assert_eq!(variant.to_i32(), i);
            }
        }
    }

    #[test]
    fn test_clone_and_copy() {
        let opt = V3dStereoDumpOptions::LeftEye;
        let opt2 = opt;
        assert_eq!(opt, opt2);
    }
}
