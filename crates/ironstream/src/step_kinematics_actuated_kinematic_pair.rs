// FILE: step_kinematics_actuated_kinematic_pair.rs
// occt: StepKinematics_ActuatedKinematicPair

use super::step_kinematics_actuated_direction::ActuatedDirection;

pub struct ActuatedKinematicPair {
    tx: Option<ActuatedDirection>,
    ty: Option<ActuatedDirection>,
    tz: Option<ActuatedDirection>,
    rx: Option<ActuatedDirection>,
    ry: Option<ActuatedDirection>,
    rz: Option<ActuatedDirection>,
}

impl ActuatedKinematicPair {
    pub fn new() -> Self {
        ActuatedKinematicPair {
            tx: None,
            ty: None,
            tz: None,
            rx: None,
            ry: None,
            rz: None,
        }
    }

    pub fn tx(&self) -> Option<ActuatedDirection> {
        self.tx
    }

    pub fn set_tx(&mut self, value: Option<ActuatedDirection>) {
        self.tx = value;
    }

    pub fn has_tx(&self) -> bool {
        self.tx.is_some()
    }

    pub fn ty(&self) -> Option<ActuatedDirection> {
        self.ty
    }

    pub fn set_ty(&mut self, value: Option<ActuatedDirection>) {
        self.ty = value;
    }

    pub fn has_ty(&self) -> bool {
        self.ty.is_some()
    }

    pub fn tz(&self) -> Option<ActuatedDirection> {
        self.tz
    }

    pub fn set_tz(&mut self, value: Option<ActuatedDirection>) {
        self.tz = value;
    }

    pub fn has_tz(&self) -> bool {
        self.tz.is_some()
    }

    pub fn rx(&self) -> Option<ActuatedDirection> {
        self.rx
    }

    pub fn set_rx(&mut self, value: Option<ActuatedDirection>) {
        self.rx = value;
    }

    pub fn has_rx(&self) -> bool {
        self.rx.is_some()
    }

    pub fn ry(&self) -> Option<ActuatedDirection> {
        self.ry
    }

    pub fn set_ry(&mut self, value: Option<ActuatedDirection>) {
        self.ry = value;
    }

    pub fn has_ry(&self) -> bool {
        self.ry.is_some()
    }

    pub fn rz(&self) -> Option<ActuatedDirection> {
        self.rz
    }

    pub fn set_rz(&mut self, value: Option<ActuatedDirection>) {
        self.rz = value;
    }

    pub fn has_rz(&self) -> bool {
        self.rz.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuated_kinematic_pair_creation() {
        let pair = ActuatedKinematicPair::new();
        assert_eq!(pair.has_tx(), false);
        assert_eq!(pair.has_ty(), false);
        assert_eq!(pair.has_tz(), false);
        assert_eq!(pair.has_rx(), false);
        assert_eq!(pair.has_ry(), false);
        assert_eq!(pair.has_rz(), false);
    }

    #[test]
    fn test_set_tx() {
        let mut pair = ActuatedKinematicPair::new();
        pair.set_tx(Some(ActuatedDirection::Bidirectional));
        assert_eq!(pair.has_tx(), true);
        assert_eq!(pair.tx(), Some(ActuatedDirection::Bidirectional));
    }

    #[test]
    fn test_set_all_directions() {
        let mut pair = ActuatedKinematicPair::new();
        pair.set_tx(Some(ActuatedDirection::PositiveOnly));
        pair.set_ty(Some(ActuatedDirection::NegativeOnly));
        pair.set_tz(Some(ActuatedDirection::NotActuated));
        pair.set_rx(Some(ActuatedDirection::Bidirectional));
        pair.set_ry(Some(ActuatedDirection::PositiveOnly));
        pair.set_rz(Some(ActuatedDirection::NegativeOnly));

        assert_eq!(pair.has_tx(), true);
        assert_eq!(pair.has_ty(), true);
        assert_eq!(pair.has_tz(), true);
        assert_eq!(pair.has_rx(), true);
        assert_eq!(pair.has_ry(), true);
        assert_eq!(pair.has_rz(), true);
    }
}
