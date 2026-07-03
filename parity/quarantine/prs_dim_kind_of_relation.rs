// FILE: prs_dim_kind_of_relation.rs
// occt: PrsDim_KindOfRelation

/// Enumeration for PrsDim_KindOfRelation.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_KindOfRelation {
    PrsDim_KOR_NONE = 0,
    PrsDim_KOR_CONCENTRIC = 1,
    PrsDim_KOR_EQUALDISTANCE = 2,
    PrsDim_KOR_EQUALRADIUS = 3,
    PrsDim_KOR_FIX = 4,
    PrsDim_KOR_IDENTIC = 5,
    PrsDim_KOR_OFFSET = 6,
    PrsDim_KOR_PARALLEL = 7,
    PrsDim_KOR_PERPENDICULAR = 8,
    PrsDim_KOR_TANGENT = 9,
    PrsDim_KOR_SYMMETRIC = 10,
}

impl PrsDim_KindOfRelation {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_KindOfRelation::PrsDim_KOR_NONE),
            0 => Some(PrsDim_KindOfRelation::PrsDim_KOR_CONCENTRIC),
            1 => Some(PrsDim_KindOfRelation::PrsDim_KOR_EQUALDISTANCE),
            2 => Some(PrsDim_KindOfRelation::PrsDim_KOR_EQUALRADIUS),
            3 => Some(PrsDim_KindOfRelation::PrsDim_KOR_FIX),
            4 => Some(PrsDim_KindOfRelation::PrsDim_KOR_IDENTIC),
            5 => Some(PrsDim_KindOfRelation::PrsDim_KOR_OFFSET),
            6 => Some(PrsDim_KindOfRelation::PrsDim_KOR_PARALLEL),
            7 => Some(PrsDim_KindOfRelation::PrsDim_KOR_PERPENDICULAR),
            8 => Some(PrsDim_KindOfRelation::PrsDim_KOR_TANGENT),
            9 => Some(PrsDim_KindOfRelation::PrsDim_KOR_SYMMETRIC),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_kind_of_relation_sanity() {
        let v = PrsDim_KindOfRelation::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}