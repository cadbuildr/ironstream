// FILE: hlr_algo_poly_mask.rs
// occt: HLRAlgo_PolyMask

/// Flags for polygon triangle masking.
pub mod flags {
    pub const EMSK_OUTLIN1: i32 = 1;
    pub const EMSK_OUTLIN2: i32 = 2;
    pub const EMSK_OUTLIN3: i32 = 4;
    pub const EMSK_GRALIN1: i32 = 8;
    pub const EMSK_GRALIN2: i32 = 16;
    pub const EMSK_GRALIN3: i32 = 32;
    pub const FMSK_BACK: i32 = 64;
    pub const FMSK_SIDE: i32 = 128;
    pub const FMSK_HIDING: i32 = 256;
    pub const FMSK_FLAT: i32 = 512;
    pub const FMSK_ONOUTL: i32 = 1024;
    pub const FMSK_ORBACK: i32 = 2048;
    pub const FMSK_FRBACK: i32 = 4096;
}

/// Utility functions for manipulating polygon masks.
pub struct PolyMask;

impl PolyMask {
    /// Check if outline flag 1 is set.
    pub fn is_outline1(mask: i32) -> bool {
        (mask & flags::EMSK_OUTLIN1) != 0
    }

    /// Check if outline flag 2 is set.
    pub fn is_outline2(mask: i32) -> bool {
        (mask & flags::EMSK_OUTLIN2) != 0
    }

    /// Check if outline flag 3 is set.
    pub fn is_outline3(mask: i32) -> bool {
        (mask & flags::EMSK_OUTLIN3) != 0
    }

    /// Check if back flag is set.
    pub fn is_back(mask: i32) -> bool {
        (mask & flags::FMSK_BACK) != 0
    }

    /// Check if side flag is set.
    pub fn is_side(mask: i32) -> bool {
        (mask & flags::FMSK_SIDE) != 0
    }

    /// Check if hiding flag is set.
    pub fn is_hiding(mask: i32) -> bool {
        (mask & flags::FMSK_HIDING) != 0
    }

    /// Set outline flag 1.
    pub fn set_outline1(mask: &mut i32) {
        *mask |= flags::EMSK_OUTLIN1;
    }

    /// Clear outline flag 1.
    pub fn clear_outline1(mask: &mut i32) {
        *mask &= !flags::EMSK_OUTLIN1;
    }

    /// Set back flag.
    pub fn set_back(mask: &mut i32) {
        *mask |= flags::FMSK_BACK;
    }

    /// Clear back flag.
    pub fn clear_back(mask: &mut i32) {
        *mask &= !flags::FMSK_BACK;
    }

    /// Set hiding flag.
    pub fn set_hiding(mask: &mut i32) {
        *mask |= flags::FMSK_HIDING;
    }

    /// Clear hiding flag.
    pub fn clear_hiding(mask: &mut i32) {
        *mask &= !flags::FMSK_HIDING;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outline_flags() {
        let mut mask = 0i32;
        assert!(!PolyMask::is_outline1(mask));
        PolyMask::set_outline1(&mut mask);
        assert!(PolyMask::is_outline1(mask));
        PolyMask::clear_outline1(&mut mask);
        assert!(!PolyMask::is_outline1(mask));
    }

    #[test]
    fn test_back_flag() {
        let mut mask = 0i32;
        assert!(!PolyMask::is_back(mask));
        PolyMask::set_back(&mut mask);
        assert!(PolyMask::is_back(mask));
        PolyMask::clear_back(&mut mask);
        assert!(!PolyMask::is_back(mask));
    }

    #[test]
    fn test_hiding_flag() {
        let mut mask = 0i32;
        assert!(!PolyMask::is_hiding(mask));
        PolyMask::set_hiding(&mut mask);
        assert!(PolyMask::is_hiding(mask));
        PolyMask::clear_hiding(&mut mask);
        assert!(!PolyMask::is_hiding(mask));
    }

    #[test]
    fn test_multiple_flags() {
        let mut mask = 0i32;
        PolyMask::set_outline1(&mut mask);
        PolyMask::set_back(&mut mask);
        assert!(PolyMask::is_outline1(mask));
        assert!(PolyMask::is_back(mask));
        assert!(!PolyMask::is_side(mask));
    }
}
