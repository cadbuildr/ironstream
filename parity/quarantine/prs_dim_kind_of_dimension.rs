// FILE: prs_dim_kind_of_dimension.rs
// occt: PrsDim_KindOfDimension

/// Enumeration for PrsDim_KindOfDimension.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_KindOfDimension {
    PrsDim_KOD_NONE = 0,
    PrsDim_KOD_LENGTH = 1,
    PrsDim_KOD_PLANEANGLE = 2,
    PrsDim_KOD_SOLIDANGLE = 3,
    PrsDim_KOD_AREA = 4,
    PrsDim_KOD_VOLUME = 5,
    PrsDim_KOD_MASS = 6,
    PrsDim_KOD_TIME = 7,
    PrsDim_KOD_RADIUS = 8,
    PrsDim_KOD_DIAMETER = 9,
    PrsDim_KOD_CHAMF2D = 10,
    PrsDim_KOD_CHAMF3D = 11,
    PrsDim_KOD_OFFSET = 12,
    PrsDim_KOD_ELLIPSERADIUS = 13,
}

impl PrsDim_KindOfDimension {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_KindOfDimension::PrsDim_KOD_NONE),
            1 => Some(PrsDim_KindOfDimension::PrsDim_KOD_LENGTH),
            2 => Some(PrsDim_KindOfDimension::PrsDim_KOD_PLANEANGLE),
            3 => Some(PrsDim_KindOfDimension::PrsDim_KOD_SOLIDANGLE),
            4 => Some(PrsDim_KindOfDimension::PrsDim_KOD_AREA),
            5 => Some(PrsDim_KindOfDimension::PrsDim_KOD_VOLUME),
            6 => Some(PrsDim_KindOfDimension::PrsDim_KOD_MASS),
            7 => Some(PrsDim_KindOfDimension::PrsDim_KOD_TIME),
            8 => Some(PrsDim_KindOfDimension::PrsDim_KOD_RADIUS),
            9 => Some(PrsDim_KindOfDimension::PrsDim_KOD_DIAMETER),
            10 => Some(PrsDim_KindOfDimension::PrsDim_KOD_CHAMF2D),
            11 => Some(PrsDim_KindOfDimension::PrsDim_KOD_CHAMF3D),
            12 => Some(PrsDim_KindOfDimension::PrsDim_KOD_OFFSET),
            13 => Some(PrsDim_KindOfDimension::PrsDim_KOD_ELLIPSERADIUS),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_kind_of_dimension_sanity() {
        let v = PrsDim_KindOfDimension::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}