// FILE: top_ope_b_rep_ds_check_status.rs
// occt: TopOpeBRepDS_CheckStatus

/// Check status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CheckStatus {
    /// Check passed OK
    OK = 0,
    /// Check failed
    NOK = 1,
}

impl CheckStatus {
    /// Convert from u8 value
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(CheckStatus::OK),
            1 => Some(CheckStatus::NOK),
            _ => None,
        }
    }

    /// Convert to u8 value
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_status_variants() {
        assert_eq!(CheckStatus::OK.as_u8(), 0);
        assert_eq!(CheckStatus::NOK.as_u8(), 1);
    }

    #[test]
    fn test_check_status_from_u8() {
        assert_eq!(CheckStatus::from_u8(0), Some(CheckStatus::OK));
        assert_eq!(CheckStatus::from_u8(1), Some(CheckStatus::NOK));
        assert_eq!(CheckStatus::from_u8(2), None);
    }

    #[test]
    fn test_check_status_clone_copy() {
        let s = CheckStatus::OK;
        let s2 = s;
        assert_eq!(s, s2);
    }
}
