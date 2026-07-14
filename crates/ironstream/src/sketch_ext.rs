// FILE: sketch_ext.rs
// occt-ref: Sketcher_Sketch, Sketcher_Constraint, Sketcher_SketchSolver
//       Sketcher_SketchGeometry

/// Constraint type for sketcher (dimensional and geometric).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SketchConstraintType {
    #[default]
    Horizontal,
    Vertical,
    Distance,
    Angle,
    Radius,
    Diameter,
    Coincident,
    Parallel,
    Perpendicular,
    Tangent,
    Equal,
    Symmetric,
    FixedPoint,
    FixedLine,
}

impl SketchConstraintType {
    pub fn is_dimensional(&self) -> bool {
        matches!(self, Self::Distance | Self::Angle | Self::Radius | Self::Diameter)
    }
    pub fn is_geometric(&self) -> bool { !self.is_dimensional() }
}

/// A constraint in a sketch.
// occt-ref: Sketcher_Constraint
#[derive(Clone, Debug)]
pub struct SketchConstraint {
    pub constraint_id: u32,
    pub constraint_type: SketchConstraintType,
    pub value: Option<f64>,
    pub geometry_1: Option<u32>,
    pub geometry_2: Option<u32>,
    pub is_driving: bool,
    pub is_satisfied: bool,
}

impl SketchConstraint {
    pub fn new(cid: u32, ctype: SketchConstraintType) -> Self {
        Self {
            constraint_id: cid,
            constraint_type: ctype,
            value: None,
            geometry_1: None,
            geometry_2: None,
            is_driving: true,
            is_satisfied: true,
        }
    }

    pub fn set_value(&mut self, v: f64) { self.value = Some(v); }
    pub fn set_geometry_1(&mut self, g: u32) { self.geometry_1 = Some(g); }
    pub fn set_geometry_2(&mut self, g: u32) { self.geometry_2 = Some(g); }
    pub fn set_driven(&mut self, driven: bool) { self.is_driving = !driven; }

    pub fn has_value(&self) -> bool { self.value.is_some() }
}

/// Sketch geometry element (point, line, circle, arc, etc.).
// occt-ref: Sketcher_SketchGeometry
#[derive(Clone, Debug)]
pub struct SketchGeometry {
    pub geom_id: u32,
    pub geom_type: u8,  // 1=point 2=line 3=circle 4=arc 5=ellipse
    pub data: Vec<f64>, // position/radius/params (variable length)
    pub is_construction: bool,
    pub is_external: bool,
}

impl SketchGeometry {
    pub fn new(geom_id: u32, geom_type: u8) -> Self {
        Self {
            geom_id,
            geom_type,
            data: Vec::new(),
            is_construction: false,
            is_external: false,
        }
    }

    pub fn point(geom_id: u32, x: f64, y: f64) -> Self {
        let mut g = Self::new(geom_id, 1);
        g.data = vec![x, y];
        g
    }

    pub fn line(geom_id: u32, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        let mut g = Self::new(geom_id, 2);
        g.data = vec![x1, y1, x2, y2];
        g
    }

    pub fn circle(geom_id: u32, cx: f64, cy: f64, radius: f64) -> Self {
        let mut g = Self::new(geom_id, 3);
        g.data = vec![cx, cy, radius];
        g
    }

    pub fn is_point(&self) -> bool { self.geom_type == 1 }
    pub fn is_line(&self) -> bool { self.geom_type == 2 }
    pub fn is_circle(&self) -> bool { self.geom_type == 3 }

    pub fn set_construction(&mut self, v: bool) { self.is_construction = v; }
}

/// Sketch solver and constraint satisfaction.
// occt-ref: Sketcher_SketchSolver
#[derive(Clone, Debug, Default)]
pub struct SketchSolver {
    pub geometries: Vec<SketchGeometry>,
    pub constraints: Vec<SketchConstraint>,
    pub degrees_of_freedom: i32,
    pub is_solved: bool,
}

impl SketchSolver {
    pub fn new() -> Self { Self::default() }

    pub fn add_geometry(&mut self, g: SketchGeometry) { self.geometries.push(g); }
    pub fn add_constraint(&mut self, c: SketchConstraint) { self.constraints.push(c); }

    pub fn nb_geometries(&self) -> usize { self.geometries.len() }
    pub fn nb_constraints(&self) -> usize { self.constraints.len() }

    pub fn solve(&mut self) -> bool {
        // Simplified: always succeeds and sets degrees of freedom
        let dof = (self.nb_geometries() as i32 * 2) - self.nb_constraints() as i32;
        self.degrees_of_freedom = dof.max(0);
        self.is_solved = true;
        true
    }

    pub fn is_under_constrained(&self) -> bool { self.degrees_of_freedom > 0 }
    pub fn is_fully_constrained(&self) -> bool { self.degrees_of_freedom == 0 }
    pub fn is_over_constrained(&self) -> bool { self.degrees_of_freedom < 0 }

    pub fn clear(&mut self) {
        self.geometries.clear();
        self.constraints.clear();
        self.degrees_of_freedom = 0;
        self.is_solved = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_type_classification() {
        assert!(SketchConstraintType::Distance.is_dimensional());
        assert!(SketchConstraintType::Parallel.is_geometric());
        assert!(!SketchConstraintType::Radius.is_geometric());
    }

    #[test]
    fn sketch_constraint() {
        let mut c = SketchConstraint::new(1, SketchConstraintType::Distance);
        c.set_value(5.0);
        c.set_geometry_1(10);
        c.set_geometry_2(11);
        assert!(c.has_value());
        assert_eq!(c.value, Some(5.0));
        assert_eq!(c.geometry_1, Some(10));
    }

    #[test]
    fn sketch_geometry_point_line_circle() {
        let p = SketchGeometry::point(1, 1.0, 2.0);
        assert!(p.is_point());
        assert_eq!(p.data.len(), 2);

        let l = SketchGeometry::line(2, 0.0, 0.0, 1.0, 1.0);
        assert!(l.is_line());
        assert_eq!(l.data.len(), 4);

        let c = SketchGeometry::circle(3, 5.0, 5.0, 2.0);
        assert!(c.is_circle());
        assert_eq!(c.data.len(), 3);
    }

    #[test]
    fn sketch_solver_dof() {
        let mut s = SketchSolver::new();
        s.add_geometry(SketchGeometry::point(1, 0.0, 0.0));
        s.add_geometry(SketchGeometry::point(2, 1.0, 1.0));
        s.add_constraint(SketchConstraint::new(1, SketchConstraintType::Distance));
        s.solve();
        assert!(s.is_solved);
        assert!(s.is_under_constrained());
    }

    #[test]
    fn sketch_construction_flag() {
        let mut g = SketchGeometry::line(1, 0.0, 0.0, 1.0, 0.0);
        assert!(!g.is_construction);
        g.set_construction(true);
        assert!(g.is_construction);
    }
}
