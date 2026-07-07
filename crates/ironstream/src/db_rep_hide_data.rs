// FILE: db_rep_hide_data.rs
// occt: DBRep_HideData

/// 2D point for display
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Default for Point2D {
    fn default() -> Self {
        Point2D { x: 0.0, y: 0.0 }
    }
}

/// 3x3 transformation matrix
#[derive(Debug, Clone, Copy)]
pub struct Matrix3x3 {
    pub m: [[f64; 3]; 3],
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Matrix3x3 {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

/// Transformation (translation + rotation/scale matrix)
#[derive(Debug, Clone, Copy)]
pub struct Transformation {
    pub matrix: Matrix3x3,
    pub translation: [f64; 3],
}

impl Default for Transformation {
    fn default() -> Self {
        Transformation {
            matrix: Matrix3x3::default(),
            translation: [0.0; 3],
        }
    }
}

impl PartialEq for Transformation {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if (self.matrix.m[i][j] - other.matrix.m[i][j]).abs() > 1e-15 {
                    return false;
                }
            }
            if (self.translation[i] - other.translation[i]).abs() > 1e-15 {
                return false;
            }
        }
        true
    }
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

/// Shape reference (stub for testing)
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub id: u64,
}

impl Default for Shape {
    fn default() -> Self {
        Shape { id: 0 }
    }
}

/// Bi-point (line segment) in 3D
#[derive(Debug, Clone, Copy)]
pub struct BiPoint {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
}

/// Storage for hidden line information
pub struct DBRepHideData {
    view_id: i32,
    trsf: Transformation,
    focal: f64,
    angle: f64,
    visible_points: Vec<BiPoint>,
    hidden_points: Vec<BiPoint>,
    pick_shape: Shape,
}

impl DBRepHideData {
    /// Create a new DBRepHideData with defaults.
    pub fn new() -> Self {
        DBRepHideData {
            view_id: -1,
            trsf: Transformation::default(),
            focal: 0.0,
            angle: 0.0,
            visible_points: Vec::new(),
            hidden_points: Vec::new(),
            pick_shape: Shape::default(),
        }
    }

    /// Set the hide data with view ID, projection, focal length, shape and angle.
    pub fn set(&mut self, view_id: i32, trsf: Transformation, focal: f64, _shape: &Shape, angle: f64) {
        self.view_id = view_id;
        self.trsf = trsf;
        self.focal = focal;
        self.angle = angle;
        // In a real implementation, we would process the shape and compute
        // visible and hidden line segments. For now, we initialize empty.
        self.visible_points.clear();
        self.hidden_points.clear();
        self.pick_shape = Shape::default();
    }

    /// Get the view ID.
    pub fn view_id(&self) -> i32 {
        self.view_id
    }

    /// Get the angle.
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Check if the projection is the same (focal distance and transformation matrix).
    pub fn is_same(&self, other_trsf: Transformation, other_focal: f64) -> bool {
        if other_focal > 0.0 {
            if self.focal <= 0.0 {
                return false;
            }
            if (self.focal - other_focal).abs() > 1e-15 {
                return false;
            }
        }
        self.trsf == other_trsf
    }

    /// Get the last picked shape.
    pub fn last_pick(&self) -> &Shape {
        &self.pick_shape
    }
}

impl Default for DBRepHideData {
    fn default() -> Self {
        DBRepHideData::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = DBRepHideData::new();
        assert_eq!(data.view_id(), -1);
        assert_eq!(data.angle(), 0.0);
    }

    #[test]
    fn test_set() {
        let mut data = DBRepHideData::new();
        let trsf = Transformation::default();
        let shape = Shape { id: 42 };

        data.set(5, trsf, 10.0, &shape, 1.5);

        assert_eq!(data.view_id(), 5);
        assert_eq!(data.angle(), 1.5);
    }

    #[test]
    fn test_is_same_perspective() {
        let mut data = DBRepHideData::new();
        let trsf1 = Transformation::default();
        let trsf2 = Transformation::default();
        let shape = Shape::default();

        data.set(1, trsf1, 10.0, &shape, 0.0);

        assert!(data.is_same(trsf2, 10.0));
        assert!(!data.is_same(trsf2, 20.0));
    }

    #[test]
    fn test_is_same_parallel_vs_perspective() {
        let mut data = DBRepHideData::new();
        let trsf = Transformation::default();
        let shape = Shape::default();

        data.set(1, trsf, 0.0, &shape, 0.0);

        assert!(data.is_same(trsf, -1.0));
        assert!(data.is_same(trsf, 0.0));
        assert!(!data.is_same(trsf, 10.0));
    }

    #[test]
    fn test_last_pick() {
        let data = DBRepHideData::new();
        assert_eq!(data.last_pick(), &Shape::default());
    }

    #[test]
    fn test_transformation_equality() {
        let mut t1 = Transformation::default();
        let mut t2 = Transformation::default();

        assert_eq!(t1, t2);

        t1.translation[0] = 1.0;
        assert_ne!(t1, t2);

        t2.translation[0] = 1.0;
        assert_eq!(t1, t2);
    }
}
