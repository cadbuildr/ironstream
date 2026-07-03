// FILE: ais_type_of_plane.rs
// occt: AIS_TypeOfPlane

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisTypeOfPlane {
    #[default]
    Unknown = 0,
    XyPlane = 1,
    XzPlane = 2,
    YzPlane = 3,
}

impl AisTypeOfPlane {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Unknown), 1 => Some(Self::XyPlane),
            2 => Some(Self::XzPlane), 3 => Some(Self::YzPlane), _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn is_known(self) -> bool { self != Self::Unknown }
    pub fn normal(&self) -> [f64; 3] {
        match self {
            Self::Unknown => [0.0, 0.0, 0.0],
            Self::XyPlane => [0.0, 0.0, 1.0],
            Self::XzPlane => [0.0, 1.0, 0.0],
            Self::YzPlane => [1.0, 0.0, 0.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() { assert_eq!(AisTypeOfPlane::XyPlane.normal(), [0.0, 0.0, 1.0]); }
}
