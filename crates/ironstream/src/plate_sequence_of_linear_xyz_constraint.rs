// FILE: plate_sequence_of_linear_xyz_constraint.rs
// occt: Plate_SequenceOfLinearXYZConstraint

//! Deprecated: Plate_SequenceOfLinearXYZConstraint is a sequence type alias.

use std::collections::VecDeque;

/// 3D point
#[derive(Debug, Clone, Copy)]
pub struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Linear XYZ constraint
#[derive(Debug, Clone)]
pub struct LinearXYZConstraint {
    point: Point3d,
}

impl LinearXYZConstraint {
    pub fn new(point: Point3d) -> Self {
        Self { point }
    }

    pub fn point(&self) -> Point3d {
        self.point
    }
}

/// Sequence of constraints
#[derive(Debug, Clone)]
pub struct Sequence {
    constraints: VecDeque<LinearXYZConstraint>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            constraints: VecDeque::new(),
        }
    }

    pub fn append(&mut self, c: LinearXYZConstraint) {
        self.constraints.push_back(c);
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn value(&self, index: usize) -> Option<&LinearXYZConstraint> {
        self.constraints.get(index)
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

pub type PlateSequenceOfLinearXYZConstraint = Sequence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut seq = Sequence::new();
        let point = Point3d::new(1.0, 2.0, 3.0);
        seq.append(LinearXYZConstraint::new(point));
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_value() {
        let mut seq = Sequence::new();
        let point = Point3d::new(1.5, 2.5, 3.5);
        seq.append(LinearXYZConstraint::new(point));

        let c = seq.value(0).unwrap();
        assert_eq!(c.point().x, 1.5);
    }
}
