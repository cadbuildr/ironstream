// FILE: prs3d_invalid_angle.rs
// occt: Prs3d_InvalidAngle

/// Stub for Prs3d_InvalidAngle from OCCT.
#[derive(Clone, Debug)]
pub struct Prs3d_InvalidAngle {}

impl Prs3d_InvalidAngle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Prs3d_InvalidAngle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_invalid_angle_creation() {
        let _obj = Prs3d_InvalidAngle::new();
        let _def = Prs3d_InvalidAngle::default();
    }
}
