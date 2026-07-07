// FILE: step_visual_surface_side.rs
// occt: StepVisual_SurfaceSide

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceSide {
    Negative = 0,
    Positive = 1,
    Both = 2,
}

impl SurfaceSide {
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(SurfaceSide::Negative),
            1 => Some(SurfaceSide::Positive),
            2 => Some(SurfaceSide::Both),
            _ => None,
        }
    }
}

impl Default for SurfaceSide {
    fn default() -> Self {
        SurfaceSide::Negative
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(SurfaceSide::Negative as i32, 0);
        assert_eq!(SurfaceSide::Positive as i32, 1);
        assert_eq!(SurfaceSide::Both as i32, 2);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(SurfaceSide::Negative.to_i32(), 0);
        assert_eq!(SurfaceSide::Positive.to_i32(), 1);
        assert_eq!(SurfaceSide::Both.to_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(SurfaceSide::from_i32(0), Some(SurfaceSide::Negative));
        assert_eq!(SurfaceSide::from_i32(1), Some(SurfaceSide::Positive));
        assert_eq!(SurfaceSide::from_i32(2), Some(SurfaceSide::Both));
        assert_eq!(SurfaceSide::from_i32(99), None);
    }

    #[test]
    fn test_default() {
        assert_eq!(SurfaceSide::default(), SurfaceSide::Negative);
    }
}
