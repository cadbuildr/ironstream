// FILE: prs3d_datum_attribute.rs
// occt: Prs3d_DatumAttribute

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum Prs3dDatumAttribute {
    #[default]
    XAxisLength = 0,
    YAxisLength = 1,
    ZAxisLength = 2,
    ShadingTubeRadiusPercent = 3,
    ShadingConeRadiusPercent = 4,
    ShadingConeLengthPercent = 5,
    ShadingOriginRadiusPercent = 6,
    ShadingNumberOfFacettes = 7,
}

impl Prs3dDatumAttribute {
    pub const NB: u32 = 8;
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0..=7 => Some(unsafe { std::mem::transmute(val) }),
            _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn is_axis_length(self) -> bool { matches!(self, Self::XAxisLength | Self::YAxisLength | Self::ZAxisLength) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_axis() { assert!(Prs3dDatumAttribute::XAxisLength.is_axis_length()); }
}
