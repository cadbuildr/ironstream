// FILE: ais_type_of_attribute.rs
// occt: AIS_TypeOfAttribute

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisTypeOfAttribute {
    #[default]
    Line = 0,
    Dimension = 1,
    Wire = 2,
    Plane = 3,
    Vector = 4,
    UIso = 5,
    VIso = 6,
    Free = 7,
    UnFree = 8,
    Section = 9,
    Hidden = 10,
    Seen = 11,
    FaceBoundary = 12,
    FirstAxis = 13,
    SecondAxis = 14,
    ThirdAxis = 15,
}

impl AisTypeOfAttribute {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0..=15 => Some(unsafe { std::mem::transmute(val) }),
            _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn is_axis(self) -> bool { matches!(self, Self::FirstAxis | Self::SecondAxis | Self::ThirdAxis) }
    pub fn is_iso(self) -> bool { matches!(self, Self::UIso | Self::VIso) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_axis() { assert!(AisTypeOfAttribute::FirstAxis.is_axis()); }
}
