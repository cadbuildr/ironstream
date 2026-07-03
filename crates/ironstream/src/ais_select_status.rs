// FILE: ais_select_status.rs
// occt: AIS_SelectStatus

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisSelectStatus {
    #[default]
    Added = 0,
    Removed = 1,
    NotDone = 2,
}

impl AisSelectStatus {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Added),
            1 => Some(Self::Removed),
            2 => Some(Self::NotDone),
            _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn is_successful(self) -> bool { self != Self::NotDone }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_successful() { assert!(AisSelectStatus::Added.is_successful()); }
}
