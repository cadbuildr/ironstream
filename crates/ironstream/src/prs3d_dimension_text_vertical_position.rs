// FILE: prs3d_dimension_text_vertical_position.rs
// occt: Prs3d_DimensionTextVerticalPosition

/// Enumeration for Prs3d_DimensionTextVerticalPosition.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_DimensionTextVerticalPosition {
    Prs3d_DTVP_Above = 0,
    Prs3d_DTVP_Below = 1,
    Prs3d_DTVP_Center = 2,
}

impl Prs3d_DimensionTextVerticalPosition {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_DimensionTextVerticalPosition::Prs3d_DTVP_Above),
            1 => Some(Prs3d_DimensionTextVerticalPosition::Prs3d_DTVP_Below),
            2 => Some(Prs3d_DimensionTextVerticalPosition::Prs3d_DTVP_Center),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_dimension_text_vertical_position_sanity() {
        let v = Prs3d_DimensionTextVerticalPosition::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}