// FILE: tdataxtd.rs
// occt: TDataXtd_Geometry, TDataXtd_Pattern, TDataXtd_Constraint
//       TDataXtd_Point, TDataXtd_Axis, TDataXtd_Plane

/// Extended geometry type stored on an OCAF label.
/// occt: TDataXtd_GeometryEnum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TDataXtdGeometryType {
    #[default]
    Any,
    Point,
    Axis,
    Line,
    Circle,
    Ellipse,
    SpLine,
    Plane,
    Cylinder,
}

/// Geometry attribute: stores type + optional parameters.
/// occt: TDataXtd_Geometry
#[derive(Clone, Debug, Default)]
pub struct TDataXtdGeometry {
    pub label_id: u32,
    pub geometry_type: TDataXtdGeometryType,
}

impl TDataXtdGeometry {
    pub fn new(label_id: u32) -> Self { Self { label_id, geometry_type: TDataXtdGeometryType::Any } }

    pub fn set_type(&mut self, t: TDataXtdGeometryType) { self.geometry_type = t; }
    pub fn get_type(&self) -> TDataXtdGeometryType { self.geometry_type }

    pub fn is_point(&self)    -> bool { self.geometry_type == TDataXtdGeometryType::Point }
    pub fn is_axis(&self)     -> bool { self.geometry_type == TDataXtdGeometryType::Axis }
    pub fn is_plane(&self)    -> bool { self.geometry_type == TDataXtdGeometryType::Plane }
    pub fn is_cylinder(&self) -> bool { self.geometry_type == TDataXtdGeometryType::Cylinder }
}

/// Constraint type for dimensional/geometric constraints.
/// occt: TDataXtd_ConstraintEnum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TDataXtdConstraintType {
    #[default]
    Radius,
    Diameter,
    MinRadius,
    MaxRadius,
    Angular,
    Perpendicular,
    Tangent,
    Parallel,
    Concentric,
    Coincident,
    Distance,
    Equal,
    Fix,
    Symmetry,
    MidPoint,
    EqualRadius,
    Offset,
}

impl TDataXtdConstraintType {
    pub fn is_dimensional(&self) -> bool {
        matches!(self, Self::Radius | Self::Diameter | Self::Distance | Self::Angular | Self::Offset)
    }
}

/// Constraint attribute on an OCAF label.
/// occt: TDataXtd_Constraint
#[derive(Clone, Debug)]
pub struct TDataXtdConstraint {
    pub label_id: u32,
    pub constraint_type: TDataXtdConstraintType,
    pub value: Option<f64>,
    pub geometry_ids: Vec<u32>,  // label ids of constrained geometries
    pub plane_id: Option<u32>,
    pub is_inverted: bool,
    pub is_verified: bool,
}

impl TDataXtdConstraint {
    pub fn new(label_id: u32, constraint_type: TDataXtdConstraintType) -> Self {
        Self {
            label_id,
            constraint_type,
            value: None,
            geometry_ids: Vec::new(),
            plane_id: None,
            is_inverted: false,
            is_verified: false,
        }
    }

    pub fn set_value(&mut self, v: f64) { self.value = Some(v); }
    pub fn add_geometry(&mut self, lid: u32) { self.geometry_ids.push(lid); }
    pub fn set_plane(&mut self, lid: u32) { self.plane_id = Some(lid); }
    pub fn invert(&mut self) { self.is_inverted = !self.is_inverted; }

    pub fn nb_geometries(&self) -> usize { self.geometry_ids.len() }
    pub fn has_value(&self) -> bool { self.value.is_some() }
}

/// Pattern (linear or circular array).
/// occt: TDataXtd_Pattern
#[derive(Clone, Debug)]
pub struct TDataXtdPatternDefinition {
    pub label_id: u32,
    pub is_circular: bool,
    pub nb_instances1: u32,
    pub nb_instances2: u32,
    pub step1: f64,
    pub step2: f64,
    pub axis1_id: u32,
    pub axis2_id: Option<u32>,
}

impl TDataXtdPatternDefinition {
    pub fn linear(label_id: u32, axis_id: u32, nb: u32, step: f64) -> Self {
        Self {
            label_id,
            is_circular: false,
            nb_instances1: nb,
            nb_instances2: 1,
            step1: step,
            step2: 0.0,
            axis1_id: axis_id,
            axis2_id: None,
        }
    }

    pub fn circular(label_id: u32, axis_id: u32, nb: u32, angle_step: f64) -> Self {
        Self {
            label_id,
            is_circular: true,
            nb_instances1: nb,
            nb_instances2: 1,
            step1: angle_step,
            step2: 0.0,
            axis1_id: axis_id,
            axis2_id: None,
        }
    }

    pub fn total_instances(&self) -> u32 { self.nb_instances1 * self.nb_instances2 }
}

/// Point attribute (3D position on a label).
/// occt: TDataXtd_Point
#[derive(Clone, Debug, Default)]
pub struct TDataXtdPoint {
    pub label_id: u32,
    pub position: [f64; 3],
}

impl TDataXtdPoint {
    pub fn new(label_id: u32, position: [f64; 3]) -> Self { Self { label_id, position } }
    pub fn set_position(&mut self, p: [f64; 3]) { self.position = p; }
}

/// Axis attribute (line direction on a label).
/// occt: TDataXtd_Axis
#[derive(Clone, Debug)]
pub struct TDataXtdAxis {
    pub label_id: u32,
    pub origin: [f64; 3],
    pub direction: [f64; 3],
}

impl Default for TDataXtdAxis {
    fn default() -> Self { Self { label_id: 0, origin: [0.0; 3], direction: [0.0, 0.0, 1.0] } }
}

impl TDataXtdAxis {
    pub fn new(label_id: u32, origin: [f64; 3], direction: [f64; 3]) -> Self {
        Self { label_id, origin, direction }
    }

    pub fn z_axis(label_id: u32) -> Self {
        Self::new(label_id, [0.0; 3], [0.0, 0.0, 1.0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_types() {
        let mut g = TDataXtdGeometry::new(1);
        g.set_type(TDataXtdGeometryType::Point);
        assert!(g.is_point());
        assert!(!g.is_axis());
        assert_eq!(g.get_type(), TDataXtdGeometryType::Point);
    }

    #[test]
    fn constraint_dimensional() {
        let mut c = TDataXtdConstraint::new(2, TDataXtdConstraintType::Radius);
        assert!(c.constraint_type.is_dimensional());
        c.set_value(5.0);
        assert!(c.has_value());
        assert!((c.value.unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn constraint_geometries() {
        let mut c = TDataXtdConstraint::new(3, TDataXtdConstraintType::Parallel);
        c.add_geometry(10);
        c.add_geometry(11);
        assert_eq!(c.nb_geometries(), 2);
        assert!(!c.constraint_type.is_dimensional());
    }

    #[test]
    fn pattern_linear_total() {
        let p = TDataXtdPatternDefinition::linear(5, 1, 6, 10.0);
        assert!(!p.is_circular);
        assert_eq!(p.total_instances(), 6);
    }

    #[test]
    fn pattern_circular() {
        let p = TDataXtdPatternDefinition::circular(6, 2, 4, 90.0);
        assert!(p.is_circular);
        assert_eq!(p.total_instances(), 4);
    }

    #[test]
    fn axis_z() {
        let a = TDataXtdAxis::z_axis(1);
        assert!((a.direction[2] - 1.0).abs() < 1e-14);
    }
}
