// FILE: prs_mgr_display_status.rs
// occt: PrsMgr_DisplayStatus

/// Enumeration for PrsMgr_DisplayStatus.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsMgr_DisplayStatus {
    PrsMgr_DisplayStatus_Displayed = 0,
    PrsMgr_DisplayStatus_Erased = 1,
    PrsMgr_DisplayStatus_None = 2,
    AIS_DS_Displayed = PrsMgr_DisplayStatus_Displayed,
    AIS_DS_Erased = PrsMgr_DisplayStatus_Erased,
    AIS_DS_None = PrsMgr_DisplayStatus_None,
}

impl PrsMgr_DisplayStatus {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Displayed),
            1 => Some(PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_Erased),
            2 => Some(PrsMgr_DisplayStatus::PrsMgr_DisplayStatus_None),
            PrsMgr_DisplayStatus_Displayed => Some(PrsMgr_DisplayStatus::AIS_DS_Displayed),
            PrsMgr_DisplayStatus_Erased => Some(PrsMgr_DisplayStatus::AIS_DS_Erased),
            PrsMgr_DisplayStatus_None => Some(PrsMgr_DisplayStatus::AIS_DS_None),
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
    }
}