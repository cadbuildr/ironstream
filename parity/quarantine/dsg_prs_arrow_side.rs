// FILE: dsg_prs_arrow_side.rs
// occt: DsgPrs_ArrowSide

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum DsgPrsArrowSide {
    #[default]
    None = 0,
    FirstAr = 1,
    LastAr = 2,
    BothAr = 3,
    FirstPt = 4,
    LastPt = 5,
    BothPt = 6,
    FirstAr_LastPt = 7,
    FirstPt_LastAr = 8,
}

impl DsgPrsArrowSide {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0..=8 => Some(unsafe { std::mem::transmute(val) }),
            _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn has_first_arrow(self) -> bool { matches!(self, Self::FirstAr | Self::BothAr) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_first() { assert!(DsgPrsArrowSide::FirstAr.has_first_arrow()); }
}
