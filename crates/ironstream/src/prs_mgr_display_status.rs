// FILE: prs_mgr_display_status.rs
// occt: PrsMgr_DisplayStatus

//! To give the display status of an Interactive Object.
//! Old AIS_DS_* names are aliases of the new ones, expressed as
//! associated constants (Rust enums cannot repeat discriminants).

#![allow(non_upper_case_globals)]

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsMgr_DisplayStatus {
    /// the Interactive Object is displayed in the main viewer
    PrsMgr_DisplayStatus_Displayed = 0,
    /// the Interactive Object is hidden in main viewer
    PrsMgr_DisplayStatus_Erased = 1,
    /// the Interactive Object is nowhere displayed
    PrsMgr_DisplayStatus_None = 2,
}

impl PrsMgr_DisplayStatus {
    // old aliases
    pub const AIS_DS_Displayed: PrsMgr_DisplayStatus =
        PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Displayed;
    pub const AIS_DS_Erased: PrsMgr_DisplayStatus =
        PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Erased;
    pub const AIS_DS_None: PrsMgr_DisplayStatus =
        PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_None;

    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Displayed),
            1 => Some(PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Erased),
            2 => Some(PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_None),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_mgr_display_status_sanity() {
        let v = PrsMgr_DisplayStatus::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
        assert_eq!(v, PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Displayed);
    }

    #[test]
    fn all_values_roundtrip() {
        for i in 0..=2 {
            let v = PrsMgr_DisplayStatus::from_u32(i).unwrap();
            assert_eq!(v.as_u32(), i);
        }
        assert_eq!(PrsMgr_DisplayStatus::from_u32(3), None);
    }

    #[test]
    fn old_aliases_equal_new_names() {
        assert_eq!(
            PrsMgr_DisplayStatus::AIS_DS_Displayed,
            PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Displayed
        );
        assert_eq!(
            PrsMgr_DisplayStatus::AIS_DS_Erased,
            PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Erased
        );
        assert_eq!(
            PrsMgr_DisplayStatus::AIS_DS_None,
            PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_None
        );
    }
}
