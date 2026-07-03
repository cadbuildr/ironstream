// FILE: ais_navigation_mode.rs
// occt: AIS_NavigationMode

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisNavigationMode {
    #[default]
    Orbit = 0,
    FirstPersonFlight = 1,
    FirstPersonWalk = 2,
}

impl AisNavigationMode {
    pub const LOWER: u32 = 0;
    pub const UPPER: u32 = 2;
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Orbit),
            1 => Some(Self::FirstPersonFlight),
            2 => Some(Self::FirstPersonWalk),
            _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() { assert_eq!(AisNavigationMode::default(), AisNavigationMode::Orbit); }
    #[test]
    fn test_from_u32() { assert_eq!(AisNavigationMode::from_u32(0), Some(AisNavigationMode::Orbit)); }
}
