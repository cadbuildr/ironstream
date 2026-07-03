// FILE: int_imp_const_isoparametric.rs
// occt: IntImp_ConstIsoparametric

/// Enumeration of isoparametric directions on surfaces during intersection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConstIsoparametric {
    /// U-isoparametric curve on first surface
    UIsoparametricOnCaro1 = 0,
    /// V-isoparametric curve on first surface
    VIsoparametricOnCaro1 = 1,
    /// U-isoparametric curve on second surface
    UIsoparametricOnCaro2 = 2,
    /// V-isoparametric curve on second surface
    VIsoparametricOnCaro2 = 3,
}

impl ConstIsoparametric {
    /// Converts an integer to the corresponding ConstIsoparametric enum value.
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(ConstIsoparametric::UIsoparametricOnCaro1),
            1 => Some(ConstIsoparametric::VIsoparametricOnCaro1),
            2 => Some(ConstIsoparametric::UIsoparametricOnCaro2),
            3 => Some(ConstIsoparametric::VIsoparametricOnCaro2),
            _ => None,
        }
    }

    /// Converts the enum value to an integer.
    pub fn to_int(self) -> i32 {
        self as i32
    }

    /// Returns true if this is a U-isoparametric curve.
    pub fn is_u_iso(self) -> bool {
        matches!(
            self,
            ConstIsoparametric::UIsoparametricOnCaro1 | ConstIsoparametric::UIsoparametricOnCaro2
        )
    }

    /// Returns true if this is a V-isoparametric curve.
    pub fn is_v_iso(self) -> bool {
        matches!(
            self,
            ConstIsoparametric::VIsoparametricOnCaro1 | ConstIsoparametric::VIsoparametricOnCaro2
        )
    }

    /// Returns true if this is on the first surface.
    pub fn is_on_first_surface(self) -> bool {
        matches!(
            self,
            ConstIsoparametric::UIsoparametricOnCaro1 | ConstIsoparametric::VIsoparametricOnCaro1
        )
    }

    /// Returns true if this is on the second surface.
    pub fn is_on_second_surface(self) -> bool {
        matches!(
            self,
            ConstIsoparametric::UIsoparametricOnCaro2 | ConstIsoparametric::VIsoparametricOnCaro2
        )
    }

    /// Swaps U and V for this isoparametric.
    pub fn swap_uv(self) -> Self {
        match self {
            ConstIsoparametric::UIsoparametricOnCaro1 => ConstIsoparametric::VIsoparametricOnCaro1,
            ConstIsoparametric::VIsoparametricOnCaro1 => ConstIsoparametric::UIsoparametricOnCaro1,
            ConstIsoparametric::UIsoparametricOnCaro2 => ConstIsoparametric::VIsoparametricOnCaro2,
            ConstIsoparametric::VIsoparametricOnCaro2 => ConstIsoparametric::UIsoparametricOnCaro2,
        }
    }

    /// Swaps surface 1 and surface 2 for this isoparametric.
    pub fn swap_surfaces(self) -> Self {
        match self {
            ConstIsoparametric::UIsoparametricOnCaro1 => ConstIsoparametric::UIsoparametricOnCaro2,
            ConstIsoparametric::VIsoparametricOnCaro1 => ConstIsoparametric::VIsoparametricOnCaro2,
            ConstIsoparametric::UIsoparametricOnCaro2 => ConstIsoparametric::UIsoparametricOnCaro1,
            ConstIsoparametric::VIsoparametricOnCaro2 => ConstIsoparametric::VIsoparametricOnCaro1,
        }
    }

    /// Returns the opposing isoparametric (U vs V) on the other surface.
    pub fn complementary(self) -> Self {
        match self {
            ConstIsoparametric::UIsoparametricOnCaro1 => ConstIsoparametric::UIsoparametricOnCaro2,
            ConstIsoparametric::VIsoparametricOnCaro1 => ConstIsoparametric::VIsoparametricOnCaro2,
            ConstIsoparametric::UIsoparametricOnCaro2 => ConstIsoparametric::UIsoparametricOnCaro1,
            ConstIsoparametric::VIsoparametricOnCaro2 => ConstIsoparametric::VIsoparametricOnCaro1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        assert_eq!(ConstIsoparametric::from_int(0), Some(ConstIsoparametric::UIsoparametricOnCaro1));
        assert_eq!(ConstIsoparametric::from_int(1), Some(ConstIsoparametric::VIsoparametricOnCaro1));
        assert_eq!(ConstIsoparametric::from_int(2), Some(ConstIsoparametric::UIsoparametricOnCaro2));
        assert_eq!(ConstIsoparametric::from_int(3), Some(ConstIsoparametric::VIsoparametricOnCaro2));
        assert_eq!(ConstIsoparametric::from_int(4), None);
    }

    #[test]
    fn test_to_int() {
        assert_eq!(ConstIsoparametric::UIsoparametricOnCaro1.to_int(), 0);
        assert_eq!(ConstIsoparametric::VIsoparametricOnCaro1.to_int(), 1);
        assert_eq!(ConstIsoparametric::UIsoparametricOnCaro2.to_int(), 2);
        assert_eq!(ConstIsoparametric::VIsoparametricOnCaro2.to_int(), 3);
    }

    #[test]
    fn test_is_u_iso() {
        assert!(ConstIsoparametric::UIsoparametricOnCaro1.is_u_iso());
        assert!(!ConstIsoparametric::VIsoparametricOnCaro1.is_u_iso());
        assert!(ConstIsoparametric::UIsoparametricOnCaro2.is_u_iso());
    }

    #[test]
    fn test_is_v_iso() {
        assert!(!ConstIsoparametric::UIsoparametricOnCaro1.is_v_iso());
        assert!(ConstIsoparametric::VIsoparametricOnCaro1.is_v_iso());
        assert!(ConstIsoparametric::VIsoparametricOnCaro2.is_v_iso());
    }

    #[test]
    fn test_is_on_first_surface() {
        assert!(ConstIsoparametric::UIsoparametricOnCaro1.is_on_first_surface());
        assert!(ConstIsoparametric::VIsoparametricOnCaro1.is_on_first_surface());
        assert!(!ConstIsoparametric::UIsoparametricOnCaro2.is_on_first_surface());
    }

    #[test]
    fn test_is_on_second_surface() {
        assert!(!ConstIsoparametric::UIsoparametricOnCaro1.is_on_second_surface());
        assert!(ConstIsoparametric::UIsoparametricOnCaro2.is_on_second_surface());
        assert!(ConstIsoparametric::VIsoparametricOnCaro2.is_on_second_surface());
    }

    #[test]
    fn test_swap_uv() {
        assert_eq!(ConstIsoparametric::UIsoparametricOnCaro1.swap_uv(), ConstIsoparametric::VIsoparametricOnCaro1);
        assert_eq!(ConstIsoparametric::VIsoparametricOnCaro1.swap_uv(), ConstIsoparametric::UIsoparametricOnCaro1);
    }

    #[test]
    fn test_swap_surfaces() {
        assert_eq!(ConstIsoparametric::UIsoparametricOnCaro1.swap_surfaces(), ConstIsoparametric::UIsoparametricOnCaro2);
        assert_eq!(ConstIsoparametric::UIsoparametricOnCaro2.swap_surfaces(), ConstIsoparametric::UIsoparametricOnCaro1);
    }

    #[test]
    fn test_complementary() {
        assert_eq!(ConstIsoparametric::UIsoparametricOnCaro1.complementary(), ConstIsoparametric::UIsoparametricOnCaro2);
        assert_eq!(ConstIsoparametric::VIsoparametricOnCaro1.complementary(), ConstIsoparametric::VIsoparametricOnCaro2);
    }
}
