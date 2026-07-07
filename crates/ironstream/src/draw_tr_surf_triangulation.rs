// FILE: draw_tr_surf_triangulation.rs
// occt: DrawTrSurf_Triangulation

//! A drawable triangulation (mesh) for the Draw interface.

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
pub struct Triangulation {
    pub vertices: Vec<Point>,
    pub triangles: Vec<[usize; 3]>,
}

impl Triangulation {
    pub fn new(vertices: Vec<Point>, triangles: Vec<[usize; 3]>) -> Self {
        Self { vertices, triangles }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }
}

#[derive(Clone, Debug)]
pub struct DrawTrSurfTriangulation {
    triangulation: Triangulation,
}

impl DrawTrSurfTriangulation {
    pub fn new(triangulation: Triangulation) -> Self {
        Self { triangulation }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn vertex_count(&self) -> usize {
        self.triangulation.vertex_count()
    }

    pub fn triangle_count(&self) -> usize {
        self.triangulation.triangle_count()
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
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangles = vec![[0, 1, 2]];

        let triangulation = Triangulation::new(vertices, triangles);
        let drawable = DrawTrSurfTriangulation::new(triangulation);

        assert_eq!(drawable.vertex_count(), 3);
        assert_eq!(drawable.triangle_count(), 1);
    }
}
