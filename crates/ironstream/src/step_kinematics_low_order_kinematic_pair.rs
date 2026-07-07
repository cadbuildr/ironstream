// FILE: step_kinematics_low_order_kinematic_pair.rs
// occt: StepKinematics_LowOrderKinematicPair

pub struct LowOrderKinematicPair {
    tx: bool,
    ty: bool,
    tz: bool,
    rx: bool,
    ry: bool,
    rz: bool,
}

impl LowOrderKinematicPair {
    pub fn new() -> Self {
        LowOrderKinematicPair {
            tx: false,
            ty: false,
            tz: false,
            rx: false,
            ry: false,
            rz: false,
        }
    }

    pub fn init(&mut self, tx: bool, ty: bool, tz: bool, rx: bool, ry: bool, rz: bool) {
        self.tx = tx;
        self.ty = ty;
        self.tz = tz;
        self.rx = rx;
        self.ry = ry;
        self.rz = rz;
    }

    pub fn tx(&self) -> bool {
        self.tx
    }

    pub fn set_tx(&mut self, value: bool) {
        self.tx = value;
    }

    pub fn ty(&self) -> bool {
        self.ty
    }

    pub fn set_ty(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn tz(&self) -> bool {
        self.tz
    }

    pub fn set_tz(&mut self, value: bool) {
        self.tz = value;
    }

    pub fn rx(&self) -> bool {
        self.rx
    }

    pub fn set_rx(&mut self, value: bool) {
        self.rx = value;
    }

    pub fn ry(&self) -> bool {
        self.ry
    }

    pub fn set_ry(&mut self, value: bool) {
        self.ry = value;
    }

    pub fn rz(&self) -> bool {
        self.rz
    }

    pub fn set_rz(&mut self, value: bool) {
        self.rz = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_order_kinematic_pair_creation() {
        let pair = LowOrderKinematicPair::new();
        assert_eq!(pair.tx(), false);
        assert_eq!(pair.ty(), false);
        assert_eq!(pair.tz(), false);
        assert_eq!(pair.rx(), false);
        assert_eq!(pair.ry(), false);
        assert_eq!(pair.rz(), false);
    }

    #[test]
    fn test_init() {
        let mut pair = LowOrderKinematicPair::new();
        pair.init(true, false, true, false, true, false);
        assert_eq!(pair.tx(), true);
        assert_eq!(pair.ty(), false);
        assert_eq!(pair.tz(), true);
        assert_eq!(pair.rx(), false);
        assert_eq!(pair.ry(), true);
        assert_eq!(pair.rz(), false);
    }

    #[test]
    fn test_setters() {
        let mut pair = LowOrderKinematicPair::new();
        pair.set_tx(true);
        pair.set_ry(true);
        assert_eq!(pair.tx(), true);
        assert_eq!(pair.ry(), true);
    }
}
