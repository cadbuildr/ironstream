// FILE: prs_dim_dimension_selection_mode.rs
// occt: PrsDim_DimensionSelectionMode

/// Enumeration for PrsDim_DimensionSelectionMode.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_DimensionSelectionMode {
    PrsDim_DimensionSelectionMode_All = 0,
    PrsDim_DimensionSelectionMode_Line = 1,
    PrsDim_DimensionSelectionMode_Text = 2,
}

impl PrsDim_DimensionSelectionMode {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_DimensionSelectionMode::PrsDim_DimensionSelectionMode_All),
            1 => Some(PrsDim_DimensionSelectionMode::PrsDim_DimensionSelectionMode_Line),
            2 => Some(PrsDim_DimensionSelectionMode::PrsDim_DimensionSelectionMode_Text),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_dimension_selection_mode_sanity() {
        let v = PrsDim_DimensionSelectionMode::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}