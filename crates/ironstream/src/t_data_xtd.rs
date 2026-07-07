// FILE: t_data_xtd.rs
// occt: TDataXtd

/// Geometry enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDataXtdGeometryEnum {
    Point = 0,
    Line = 1,
    Circle = 2,
    Ellipse = 3,
    Plane = 4,
    Cylinder = 5,
    Cone = 6,
    Sphere = 7,
    Torus = 8,
}

/// Constraint enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDataXtdConstraintEnum {
    Distance = 0,
    Angle = 1,
    Tangent = 2,
    Perpendicular = 3,
    Parallel = 4,
    Concentric = 5,
    Coincident = 6,
    Equal = 7,
    Symmetric = 8,
    Midpoint = 9,
}

/// This package defines extension of standard attributes for modelling
pub struct TDataXtd;

impl TDataXtd {
    pub fn id_list(list: &mut std::vec::Vec<u8>) {
        list.push(1);
        list.push(2);
        list.push(3);
    }

    pub fn print_geometry(geo: TDataXtdGeometryEnum) -> &'static str {
        match geo {
            TDataXtdGeometryEnum::Point => "Point",
            TDataXtdGeometryEnum::Line => "Line",
            TDataXtdGeometryEnum::Circle => "Circle",
            TDataXtdGeometryEnum::Ellipse => "Ellipse",
            TDataXtdGeometryEnum::Plane => "Plane",
            TDataXtdGeometryEnum::Cylinder => "Cylinder",
            TDataXtdGeometryEnum::Cone => "Cone",
            TDataXtdGeometryEnum::Sphere => "Sphere",
            TDataXtdGeometryEnum::Torus => "Torus",
        }
    }

    pub fn print_constraint(ctr: TDataXtdConstraintEnum) -> &'static str {
        match ctr {
            TDataXtdConstraintEnum::Distance => "Distance",
            TDataXtdConstraintEnum::Angle => "Angle",
            TDataXtdConstraintEnum::Tangent => "Tangent",
            TDataXtdConstraintEnum::Perpendicular => "Perpendicular",
            TDataXtdConstraintEnum::Parallel => "Parallel",
            TDataXtdConstraintEnum::Concentric => "Concentric",
            TDataXtdConstraintEnum::Coincident => "Coincident",
            TDataXtdConstraintEnum::Equal => "Equal",
            TDataXtdConstraintEnum::Symmetric => "Symmetric",
            TDataXtdConstraintEnum::Midpoint => "Midpoint",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_list() {
        let mut list = std::vec::Vec::new();
        TDataXtd::id_list(&mut list);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_print_geometry_point() {
        let result = TDataXtd::print_geometry(TDataXtdGeometryEnum::Point);
        assert_eq!(result, "Point");
    }

    #[test]
    fn test_print_geometry_plane() {
        let result = TDataXtd::print_geometry(TDataXtdGeometryEnum::Plane);
        assert_eq!(result, "Plane");
    }

    #[test]
    fn test_print_constraint_distance() {
        let result = TDataXtd::print_constraint(TDataXtdConstraintEnum::Distance);
        assert_eq!(result, "Distance");
    }

    #[test]
    fn test_print_constraint_tangent() {
        let result = TDataXtd::print_constraint(TDataXtdConstraintEnum::Tangent);
        assert_eq!(result, "Tangent");
    }
}
