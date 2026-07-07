// FILE: step_fea_curve_element_end_release.rs
// occt: StepFEA_CurveElementEndRelease

/// Representation of STEP entity CurveElementEndRelease.
#[derive(Clone)]
pub struct CurveElementEndRelease {
    free_translational_x: bool,
    free_translational_y: bool,
    free_translational_z: bool,
    free_rotational_x: bool,
    free_rotational_y: bool,
    free_rotational_z: bool,
}

impl CurveElementEndRelease {
    pub fn new() -> Self {
        Self {
            free_translational_x: false,
            free_translational_y: false,
            free_translational_z: false,
            free_rotational_x: false,
            free_rotational_y: false,
            free_rotational_z: false,
        }
    }

    pub fn init(
        &mut self,
        ftx: bool,
        fty: bool,
        ftz: bool,
        frx: bool,
        fry: bool,
        frz: bool,
    ) {
        self.free_translational_x = ftx;
        self.free_translational_y = fty;
        self.free_translational_z = ftz;
        self.free_rotational_x = frx;
        self.free_rotational_y = fry;
        self.free_rotational_z = frz;
    }

    pub fn free_translational_x(&self) -> bool {
        self.free_translational_x
    }

    pub fn set_free_translational_x(&mut self, b: bool) {
        self.free_translational_x = b;
    }

    pub fn free_translational_y(&self) -> bool {
        self.free_translational_y
    }

    pub fn set_free_translational_y(&mut self, b: bool) {
        self.free_translational_y = b;
    }

    pub fn free_translational_z(&self) -> bool {
        self.free_translational_z
    }

    pub fn set_free_translational_z(&mut self, b: bool) {
        self.free_translational_z = b;
    }

    pub fn free_rotational_x(&self) -> bool {
        self.free_rotational_x
    }

    pub fn set_free_rotational_x(&mut self, b: bool) {
        self.free_rotational_x = b;
    }

    pub fn free_rotational_y(&self) -> bool {
        self.free_rotational_y
    }

    pub fn set_free_rotational_y(&mut self, b: bool) {
        self.free_rotational_y = b;
    }

    pub fn free_rotational_z(&self) -> bool {
        self.free_rotational_z
    }

    pub fn set_free_rotational_z(&mut self, b: bool) {
        self.free_rotational_z = b;
    }
}

impl Default for CurveElementEndRelease {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let release = CurveElementEndRelease::new();
        assert!(!release.free_translational_x());
        assert!(!release.free_rotational_z());
    }

    #[test]
    fn test_init() {
        let mut release = CurveElementEndRelease::new();
        release.init(true, false, true, false, true, false);

        assert!(release.free_translational_x());
        assert!(!release.free_translational_y());
        assert!(release.free_translational_z());
    }

    #[test]
    fn test_setters() {
        let mut release = CurveElementEndRelease::new();
        release.set_free_translational_x(true);
        release.set_free_rotational_z(true);

        assert!(release.free_translational_x());
        assert!(release.free_rotational_z());
    }
}
