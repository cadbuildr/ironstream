// FILE: draw_tr_surf_polygon2_d.rs
// occt: DrawTrSurf_Polygon2D

//! A drawable 2D polygon for the Draw interface.

#[derive(Clone, Copy, Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct Polygon2d {
    pub vertices: Vec<Point2d>,
}

impl Polygon2d {
    pub fn new(vertices: Vec<Point2d>) -> Self {
        Self { vertices }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

#[derive(Clone, Debug)]
pub struct DrawTrSurfPolygon2d {
    polygon: Polygon2d,
}

impl DrawTrSurfPolygon2d {
    pub fn new(polygon: Polygon2d) -> Self {
        Self { polygon }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn vertex_count(&self) -> usize {
        self.polygon.vertex_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let vertices = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0), Point2d::new(0.5, 1.0)];
        let polygon = Polygon2d::new(vertices);
        let drawable = DrawTrSurfPolygon2d::new(polygon);

        assert_eq!(drawable.vertex_count(), 3);
    }

    #[test]
    fn test_empty() {
        let polygon = Polygon2d::new(Vec::new());
        let drawable = DrawTrSurfPolygon2d::new(polygon);

        assert_eq!(drawable.vertex_count(), 0);
    }
}
