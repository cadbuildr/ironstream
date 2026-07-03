// FILE: ais_status_of_detection.rs
// occt: AIS_StatusOfDetection

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AisStatusOfDetection {
    #[default]
    Error = 0,
    Nothing = 1,
    AllBad = 2,
    Selected = 3,
    OnlyOneDetected = 4,
    OnlyOneGood = 5,
    SeveralGood = 6,
}

impl AisStatusOfDetection {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Error), 1 => Some(Self::Nothing), 2 => Some(Self::AllBad),
            3 => Some(Self::Selected), 4 => Some(Self::OnlyOneDetected), 5 => Some(Self::OnlyOneGood),
            6 => Some(Self::SeveralGood), _ => None,
        }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
    pub fn is_successful(self) -> bool { !matches!(self, Self::Error | Self::Nothing) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_successful() { assert!(AisStatusOfDetection::OnlyOneGood.is_successful()); }
}
