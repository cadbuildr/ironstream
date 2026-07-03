// FILE: ais_rotation_mode.rs
// occt: AIS_RotationMode

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisRotationMode {
    #[default]
    BndBoxActive = 0,
    PickLast = 1,
    PickCenter = 2,
    CameraAt = 3,
    BndBoxScene = 4,
}

impl AisRotationMode {
    pub const LOWER: u32 = 0;
    pub const UPPER: u32 = 4;
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::BndBoxActive),
            1 => Some(Self::PickLast),
            2 => Some(Self::PickCenter),
            3 => Some(Self::CameraAt),
            4 => Some(Self::BndBoxScene),
            _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() { assert_eq!(AisRotationMode::default(), AisRotationMode::BndBoxActive); }
}
