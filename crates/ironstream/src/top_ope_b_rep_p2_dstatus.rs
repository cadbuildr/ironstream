// FILE: top_ope_b_rep_p2_dstatus.rs
// occt: TopOpeBRep_P2Dstatus

/// 2D point status enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum P2Dstatus {
    /// Unknown status
    Unknown = 0,
    /// Interior point
    Int = 1,
    /// Singular on face
    Sgf = 2,
    /// Singular on line
    Sgl = 3,
    /// New point
    New = 4,
}

impl P2Dstatus {
    /// Convert from raw u32 value.
    pub fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(P2Dstatus::Unknown),
            1 => Some(P2Dstatus::Int),
            2 => Some(P2Dstatus::Sgf),
            3 => Some(P2Dstatus::Sgl),
            4 => Some(P2Dstatus::New),
            _ => None,
        }
    }

    /// Convert to raw u32 value.
    pub fn to_raw(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2dstatus_values() {
        assert_eq!(P2Dstatus::Unknown as u32, 0);
        assert_eq!(P2Dstatus::Int as u32, 1);
        assert_eq!(P2Dstatus::Sgf as u32, 2);
        assert_eq!(P2Dstatus::Sgl as u32, 3);
        assert_eq!(P2Dstatus::New as u32, 4);
    }

    #[test]
    fn test_from_raw() {
        assert_eq!(P2Dstatus::from_raw(0), Some(P2Dstatus::Unknown));
        assert_eq!(P2Dstatus::from_raw(1), Some(P2Dstatus::Int));
        assert_eq!(P2Dstatus::from_raw(2), Some(P2Dstatus::Sgf));
        assert_eq!(P2Dstatus::from_raw(3), Some(P2Dstatus::Sgl));
        assert_eq!(P2Dstatus::from_raw(4), Some(P2Dstatus::New));
        assert_eq!(P2Dstatus::from_raw(5), None);
    }

    #[test]
    fn test_to_raw() {
        assert_eq!(P2Dstatus::Unknown.to_raw(), 0);
        assert_eq!(P2Dstatus::Int.to_raw(), 1);
        assert_eq!(P2Dstatus::Sgf.to_raw(), 2);
        assert_eq!(P2Dstatus::Sgl.to_raw(), 3);
        assert_eq!(P2Dstatus::New.to_raw(), 4);
    }
}
