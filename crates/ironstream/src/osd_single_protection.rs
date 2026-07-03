// FILE: osd_single_protection.rs
// occt: OSD_SingleProtection

/// Access rights for files.
/// R means Read, W means Write, X means eXecute and D means Delete.
/// On UNIX, the right to Delete is combined with Write access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OsdSingleProtection {
    None = 0,
    R = 1,
    W = 2,
    RW = 3,
    X = 4,
    RX = 5,
    WX = 6,
    RWX = 7,
    D = 8,
    RD = 9,
    WD = 10,
    RWD = 11,
    XD = 12,
    RXD = 13,
    WXD = 14,
    RWXD = 15,
}

impl OsdSingleProtection {
    /// Checks if this protection has Read access
    pub fn has_read(self) -> bool {
        matches!(
            self,
            OsdSingleProtection::R
                | OsdSingleProtection::RW
                | OsdSingleProtection::RX
                | OsdSingleProtection::RWX
                | OsdSingleProtection::RD
                | OsdSingleProtection::RWD
                | OsdSingleProtection::RXD
                | OsdSingleProtection::RWXD
        )
    }

    /// Checks if this protection has Write access
    pub fn has_write(self) -> bool {
        matches!(
            self,
            OsdSingleProtection::W
                | OsdSingleProtection::RW
                | OsdSingleProtection::WX
                | OsdSingleProtection::RWX
                | OsdSingleProtection::WD
                | OsdSingleProtection::RWD
                | OsdSingleProtection::WXD
                | OsdSingleProtection::RWXD
        )
    }

    /// Checks if this protection has Execute access
    pub fn has_execute(self) -> bool {
        matches!(
            self,
            OsdSingleProtection::X
                | OsdSingleProtection::RX
                | OsdSingleProtection::WX
                | OsdSingleProtection::RWX
                | OsdSingleProtection::XD
                | OsdSingleProtection::RXD
                | OsdSingleProtection::WXD
                | OsdSingleProtection::RWXD
        )
    }

    /// Checks if this protection has Delete access
    pub fn has_delete(self) -> bool {
        matches!(
            self,
            OsdSingleProtection::D
                | OsdSingleProtection::RD
                | OsdSingleProtection::WD
                | OsdSingleProtection::RWD
                | OsdSingleProtection::XD
                | OsdSingleProtection::RXD
                | OsdSingleProtection::WXD
                | OsdSingleProtection::RWXD
        )
    }

    /// Returns union of two protections
    pub fn union(self, other: OsdSingleProtection) -> OsdSingleProtection {
        let mut result = 0u8;
        if self.has_read() || other.has_read() {
            result |= 0x1;
        }
        if self.has_write() || other.has_write() {
            result |= 0x2;
        }
        if self.has_execute() || other.has_execute() {
            result |= 0x4;
        }
        if self.has_delete() || other.has_delete() {
            result |= 0x8;
        }
        OsdSingleProtection::from_int(result as i32).unwrap_or(OsdSingleProtection::None)
    }

    /// Returns difference of two protections (self minus other)
    pub fn difference(self, other: OsdSingleProtection) -> OsdSingleProtection {
        let mut result = 0u8;
        if self.has_read() && !other.has_read() {
            result |= 0x1;
        }
        if self.has_write() && !other.has_write() {
            result |= 0x2;
        }
        if self.has_execute() && !other.has_execute() {
            result |= 0x4;
        }
        if self.has_delete() && !other.has_delete() {
            result |= 0x8;
        }
        OsdSingleProtection::from_int(result as i32).unwrap_or(OsdSingleProtection::None)
    }

    /// Creates OsdSingleProtection from integer value
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(OsdSingleProtection::None),
            1 => Some(OsdSingleProtection::R),
            2 => Some(OsdSingleProtection::W),
            3 => Some(OsdSingleProtection::RW),
            4 => Some(OsdSingleProtection::X),
            5 => Some(OsdSingleProtection::RX),
            6 => Some(OsdSingleProtection::WX),
            7 => Some(OsdSingleProtection::RWX),
            8 => Some(OsdSingleProtection::D),
            9 => Some(OsdSingleProtection::RD),
            10 => Some(OsdSingleProtection::WD),
            11 => Some(OsdSingleProtection::RWD),
            12 => Some(OsdSingleProtection::XD),
            13 => Some(OsdSingleProtection::RXD),
            14 => Some(OsdSingleProtection::WXD),
            15 => Some(OsdSingleProtection::RWXD),
            _ => None,
        }
    }

    /// Gets numeric value
    pub fn as_int(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_read() {
        assert!(OsdSingleProtection::R.has_read());
        assert!(OsdSingleProtection::RW.has_read());
        assert!(!OsdSingleProtection::W.has_read());
        assert!(!OsdSingleProtection::None.has_read());
    }

    #[test]
    fn test_has_write() {
        assert!(OsdSingleProtection::W.has_write());
        assert!(OsdSingleProtection::RW.has_write());
        assert!(!OsdSingleProtection::R.has_write());
    }

    #[test]
    fn test_has_execute() {
        assert!(OsdSingleProtection::X.has_execute());
        assert!(OsdSingleProtection::RWX.has_execute());
        assert!(!OsdSingleProtection::RW.has_execute());
    }

    #[test]
    fn test_has_delete() {
        assert!(OsdSingleProtection::D.has_delete());
        assert!(OsdSingleProtection::RWD.has_delete());
        assert!(!OsdSingleProtection::RW.has_delete());
    }

    #[test]
    fn test_union() {
        let r = OsdSingleProtection::R;
        let w = OsdSingleProtection::W;
        assert_eq!(r.union(w), OsdSingleProtection::RW);
    }

    #[test]
    fn test_difference() {
        let rw = OsdSingleProtection::RW;
        let w = OsdSingleProtection::W;
        assert_eq!(rw.difference(w), OsdSingleProtection::R);
    }

    #[test]
    fn test_from_int() {
        assert_eq!(OsdSingleProtection::from_int(0), Some(OsdSingleProtection::None));
        assert_eq!(OsdSingleProtection::from_int(3), Some(OsdSingleProtection::RW));
        assert_eq!(OsdSingleProtection::from_int(15), Some(OsdSingleProtection::RWXD));
        assert_eq!(OsdSingleProtection::from_int(99), None);
    }
}
