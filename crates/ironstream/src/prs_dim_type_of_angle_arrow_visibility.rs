// FILE: prs_dim_type_of_angle_arrow_visibility.rs
// occt: PrsDim_TypeOfAngleArrowVisibility

/// Enumeration for PrsDim_TypeOfAngleArrowVisibility.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_TypeOfAngleArrowVisibility {
    PrsDim_TypeOfAngleArrowVisibility_Both = 0,
    PrsDim_TypeOfAngleArrowVisibility_First = 1,
    PrsDim_TypeOfAngleArrowVisibility_Second = 2,
    PrsDim_TypeOfAngleArrowVisibility_None = 3,
}

impl PrsDim_TypeOfAngleArrowVisibility {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_TypeOfAngleArrowVisibility::PrsDim_TypeOfAngleArrowVisibility_Both),
            1 => Some(PrsDim_TypeOfAngleArrowVisibility::PrsDim_TypeOfAngleArrowVisibility_First),
            2 => Some(PrsDim_TypeOfAngleArrowVisibility::PrsDim_TypeOfAngleArrowVisibility_Second),
            3 => Some(PrsDim_TypeOfAngleArrowVisibility::PrsDim_TypeOfAngleArrowVisibility_None),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_type_of_angle_arrow_visibility_sanity() {
        let v = PrsDim_TypeOfAngleArrowVisibility::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}