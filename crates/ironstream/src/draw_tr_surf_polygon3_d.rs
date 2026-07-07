// FILE: draw_tr_surf_polygon3_d.rs
// occt: DrawTrSurf_Polygon3D

//! A drawable 3D polygon for the Draw interface.

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct Polygon3d {
    pub vertices: Vec<Point>,
}

impl Polygon3d {
    pub fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

#[derive(Clone, Debug)]
pub struct DrawTrSurfPolygon3d {
    polygon: Polygon3d,
}

impl DrawTrSurfPolygon3d {
    pub fn new(polygon: Polygon3d) -> Self {
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
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 1.0, 0.0),
        ];
        let polygon = Polygon3d::new(vertices);
        let drawable = DrawTrSurfPolygon3d::new(polygon);

        assert_eq!(drawable.vertex_count(), 3);
    }
}
