// FILE: ais_status_of_pick.rs
// occt: AIS_StatusOfPick

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisStatusOfPick {
    #[default]
    Error = 0,
    NothingSelected = 1,
    Removed = 2,
    OneSelected = 3,
    SeveralSelected = 4,
}

impl AisStatusOfPick {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Error), 1 => Some(Self::NothingSelected), 2 => Some(Self::Removed),
            3 => Some(Self::OneSelected), 4 => Some(Self::SeveralSelected), _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn is_successful(self) -> bool { self != Self::Error }
    pub fn has_selection(self) -> bool { matches!(self, Self::OneSelected | Self::SeveralSelected) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_selection() { assert!(AisStatusOfPick::OneSelected.has_selection()); }
}
