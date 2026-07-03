// FILE: ais_selection_modes_concurrency.rs
// occt: AIS_SelectionModesConcurrency

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisSelectionModesConcurrency {
    #[default]
    Single = 0,
    GlobalOrLocal = 1,
    Multiple = 2,
}

impl AisSelectionModesConcurrency {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Single),
            1 => Some(Self::GlobalOrLocal),
            2 => Some(Self::Multiple),
            _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn allows_multiple(self) -> bool { !matches!(self, Self::Single) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_allows_multiple() { assert!(AisSelectionModesConcurrency::Multiple.allows_multiple()); }
}
