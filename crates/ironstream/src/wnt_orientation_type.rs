// FILE: wnt_orientation_type.rs
// occt: WNT_OrientationType

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrientationType {
    Landscape = 0,
    Portrait = 1,
    LandscapeFlipped = 2,
    PortraitFlipped = 3,
}

impl OrientationType {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(OrientationType::Landscape),
            1 => Some(OrientationType::Portrait),
            2 => Some(OrientationType::LandscapeFlipped),
            3 => Some(OrientationType::PortraitFlipped),
            _ => None,
        }
    }

    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(OrientationType::Landscape as u32, 0);
    }
}
