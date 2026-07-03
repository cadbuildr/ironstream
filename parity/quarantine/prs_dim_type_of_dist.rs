// FILE: prs_dim_type_of_dist.rs
// occt: PrsDim_TypeOfDist

/// Enumeration for PrsDim_TypeOfDist.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_TypeOfDist {
    PrsDim_TypeOfDist_Unknown = 0,
    PrsDim_TypeOfDist_Horizontal = 1,
    PrsDim_TypeOfDist_Vertical = 2,
}

impl PrsDim_TypeOfDist {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_TypeOfDist::PrsDim_TypeOfDist_Unknown),
            1 => Some(PrsDim_TypeOfDist::PrsDim_TypeOfDist_Horizontal),
            2 => Some(PrsDim_TypeOfDist::PrsDim_TypeOfDist_Vertical),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_type_of_dist_sanity() {
        let v = PrsDim_TypeOfDist::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}