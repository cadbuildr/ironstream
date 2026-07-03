// FILE: ais_manipulator.rs
// occt: AIS_Manipulator, AIS_ManipulatorMode, AIS_ManipulatorObjectSequence

/// Manipulation mode for the 3D manipulator widget.
/// occt: AIS_ManipulatorMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisManipulatorMode {
    None,
    Translation,
    Rotation,
    Scaling,
}

impl Default for AisManipulatorMode {
    fn default() -> Self { Self::None }
}

/// Axis index for manipulator parts (0=X, 1=Y, 2=Z, 3=XYZ for scaling).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisManipulatorAxis {
    X = 0,
    Y = 1,
    Z = 2,
    All = 3,
}

/// A part of the manipulator (one axis + one mode).
/// occt: AIS_Manipulator::AxisPart
#[derive(Clone, Copy, Debug)]
pub struct AisManipulatorPart {
    pub axis: AisManipulatorAxis,
    pub mode: AisManipulatorMode,
}

impl AisManipulatorPart {
    pub fn new(axis: AisManipulatorAxis, mode: AisManipulatorMode) -> Self {
        Self { axis, mode }
    }
}

/// 3D interactive manipulator (move/rotate/scale attached shape).
/// occt: AIS_Manipulator
#[derive(Clone, Debug)]
pub struct AisManipulator {
    pub object_id: u32,
    pub attached_shape_ids: Vec<u32>,
    pub position: [f64; 3],
    pub orientation: [f64; 9],  // 3×3 rotation matrix, row-major
    pub mode: AisManipulatorMode,
    pub size: f64,
    pub is_visible: bool,
    pub enable_rotation: [bool; 3],
    pub enable_translation: [bool; 3],
    pub enable_scaling: bool,
}

impl AisManipulator {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            attached_shape_ids: Vec::new(),
            position: [0.0; 3],
            orientation: [1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0],
            mode: AisManipulatorMode::None,
            size: 1.0,
            is_visible: true,
            enable_rotation: [true; 3],
            enable_translation: [true; 3],
            enable_scaling: false,
        }
    }

    pub fn attach(&mut self, shape_id: u32) {
        if !self.attached_shape_ids.contains(&shape_id) {
            self.attached_shape_ids.push(shape_id);
        }
    }

    pub fn detach(&mut self) { self.attached_shape_ids.clear(); }

    pub fn is_attached(&self) -> bool { !self.attached_shape_ids.empty_check() }

    pub fn set_position(&mut self, p: [f64; 3]) { self.position = p; }
    pub fn set_size(&mut self, s: f64) { self.size = s.max(1e-6); }
    pub fn set_mode(&mut self, m: AisManipulatorMode) { self.mode = m; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }

    pub fn enable_axis_rotation(&mut self, axis: usize, v: bool) {
        if axis < 3 { self.enable_rotation[axis] = v; }
    }

    pub fn enable_axis_translation(&mut self, axis: usize, v: bool) {
        if axis < 3 { self.enable_translation[axis] = v; }
    }

    pub fn set_enable_scaling(&mut self, v: bool) { self.enable_scaling = v; }

    pub fn mode(&self) -> AisManipulatorMode { self.mode }
    pub fn is_visible(&self) -> bool { self.is_visible }
    pub fn size(&self) -> f64 { self.size }
    pub fn nb_attached(&self) -> usize { self.attached_shape_ids.len() }

    /// Compute new position after translation delta.
    pub fn apply_translation(&self, delta: [f64; 3]) -> [f64; 3] {
        [self.position[0]+delta[0], self.position[1]+delta[1], self.position[2]+delta[2]]
    }
}

trait EmptyCheck { fn empty_check(&self) -> bool; }
impl<T> EmptyCheck for Vec<T> { fn empty_check(&self) -> bool { self.is_empty() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manipulator_defaults() {
        let m = AisManipulator::new(1);
        assert_eq!(m.mode(), AisManipulatorMode::None);
        assert!(m.is_visible());
        assert_eq!(m.nb_attached(), 0);
    }

    #[test]
    fn manipulator_attach_detach() {
        let mut m = AisManipulator::new(1);
        m.attach(10);
        m.attach(20);
        assert_eq!(m.nb_attached(), 2);
        m.detach();
        assert_eq!(m.nb_attached(), 0);
    }

    #[test]
    fn manipulator_set_mode() {
        let mut m = AisManipulator::new(1);
        m.set_mode(AisManipulatorMode::Translation);
        assert_eq!(m.mode(), AisManipulatorMode::Translation);
    }

    #[test]
    fn manipulator_translation() {
        let mut m = AisManipulator::new(1);
        m.set_position([1.0, 2.0, 3.0]);
        let new_pos = m.apply_translation([0.5, -1.0, 0.0]);
        assert!((new_pos[0] - 1.5).abs() < 1e-10);
        assert!((new_pos[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn manipulator_axis_flags() {
        let mut m = AisManipulator::new(1);
        m.enable_axis_rotation(1, false);
        assert!(!m.enable_rotation[1]);
        m.set_enable_scaling(true);
        assert!(m.enable_scaling);
    }
}
