// FILE: ais_trihedron_selection_mode.rs
// occt: AIS_TrihedronSelectionMode

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisTrihedronSelectionMode {
    #[default]
    EntireObject = 0,
    Origin = 1,
    Axes = 2,
    MainPlanes = 3,
}

impl AisTrihedronSelectionMode {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::EntireObject), 1 => Some(Self::Origin),
            2 => Some(Self::Axes), 3 => Some(Self::MainPlanes), _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() { assert_eq!(AisTrihedronSelectionMode::default(), AisTrihedronSelectionMode::EntireObject); }
}
