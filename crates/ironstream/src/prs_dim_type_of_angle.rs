// FILE: prs_dim_type_of_angle.rs
// occt: PrsDim_TypeOfAngle

/// Enumeration for PrsDim_TypeOfAngle.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_TypeOfAngle {
    PrsDim_TypeOfAngle_Interior = 0,
    PrsDim_TypeOfAngle_Exterior = 1,
}

impl PrsDim_TypeOfAngle {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_TypeOfAngle::PrsDim_TypeOfAngle_Interior),
            1 => Some(PrsDim_TypeOfAngle::PrsDim_TypeOfAngle_Exterior),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_type_of_angle_sanity() {
        let v = PrsDim_TypeOfAngle::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}