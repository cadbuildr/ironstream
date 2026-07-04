// FILE: prs_mgr_type_of_presentation3d.rs
// occt: PrsMgr_TypeOfPresentation3d

/// Enumeration for PrsMgr_TypeOfPresentation3d.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsMgr_TypeOfPresentation3d {
    PrsMgr_TOP_AllView = 0,
    PrsMgr_TOP_ProjectorDependent = 1,
}

impl PrsMgr_TypeOfPresentation3d {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsMgr_TypeOfPresentation3d::PrsMgr_TOP_AllView),
            1 => Some(PrsMgr_TypeOfPresentation3d::PrsMgr_TOP_ProjectorDependent),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_mgr_type_of_presentation3d_sanity() {
        let v = PrsMgr_TypeOfPresentation3d::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}