// FILE: prs3d_dimension_text_horizontal_position.rs
// occt: Prs3d_DimensionTextHorizontalPosition

/// Enumeration for Prs3d_DimensionTextHorizontalPosition.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_DimensionTextHorizontalPosition {
    Prs3d_DTHP_Left = 0,
    Prs3d_DTHP_Right = 1,
    Prs3d_DTHP_Center = 2,
    Prs3d_DTHP_Fit = 3,
}

impl Prs3d_DimensionTextHorizontalPosition {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_DimensionTextHorizontalPosition::Prs3d_DTHP_Left),
            1 => Some(Prs3d_DimensionTextHorizontalPosition::Prs3d_DTHP_Right),
            2 => Some(Prs3d_DimensionTextHorizontalPosition::Prs3d_DTHP_Center),
            3 => Some(Prs3d_DimensionTextHorizontalPosition::Prs3d_DTHP_Fit),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_dimension_text_horizontal_position_sanity() {
        let v = Prs3d_DimensionTextHorizontalPosition::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}