// FILE: ais_display_status.rs
// occt: AIS_DisplayStatus

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayStatus {
    None = 0,
    Displayed = 1,
    Erased = 2,
}

impl DisplayStatus {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(DisplayStatus::None),
            1 => Some(DisplayStatus::Displayed),
            2 => Some(DisplayStatus::Erased),
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
        assert_eq!(DisplayStatus::None as u32, 0);
    }
}
