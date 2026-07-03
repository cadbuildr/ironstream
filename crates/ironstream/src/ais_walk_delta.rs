// FILE: ais_walk_delta.rs
// occt: AIS_WalkDelta

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisWalkTranslation { Forward = 0, Side = 1, Up = 2 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisWalkRotation { Yaw = 0, Pitch = 1, Roll = 2 }

#[derive(Clone, Copy, Debug)]
pub struct AisWalkPart { pub value: f64, pub pressure: f64, pub duration: f64 }

impl AisWalkPart {
    pub fn new(value: f64, pressure: f64, duration: f64) -> Self {
        Self { value, pressure, duration }
    }
    pub fn is_empty(&self) -> bool { self.value.abs() <= 1e-14 }
}

impl Default for AisWalkPart {
    fn default() -> Self { Self { value: 0.0, pressure: 1.0, duration: 0.0 } }
}

#[derive(Clone, Debug)]
pub struct AisWalkDelta {
    translation: [AisWalkPart; 3],
    rotation: [AisWalkPart; 3],
    is_jumping: bool,
}

impl AisWalkDelta {
    pub fn new() -> Self {
        Self {
            translation: [AisWalkPart::default(); 3],
            rotation: [AisWalkPart::default(); 3],
            is_jumping: false,
        }
    }
    pub fn is_jumping(&self) -> bool { self.is_jumping }
    pub fn set_jumping(&mut self, v: bool) { self.is_jumping = v; }
}

impl Default for AisWalkDelta {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_walk_part_default() {
        let part = AisWalkPart::default();
        assert!(part.is_empty());
    }
}
