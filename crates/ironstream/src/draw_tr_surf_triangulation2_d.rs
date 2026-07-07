// FILE: draw_tr_surf_triangulation2_d.rs
// occt: DrawTrSurf_Triangulation2D

//! A drawable 2D triangulation for the Draw interface.

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
pub struct Triangulation2d {
    pub vertices: Vec<Point2d>,
    pub triangles: Vec<[usize; 3]>,
}

impl Triangulation2d {
    pub fn new(vertices: Vec<Point2d>, triangles: Vec<[usize; 3]>) -> Self {
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
pub struct DrawTrSurfTriangulation2d {
    triangulation: Triangulation2d,
}

impl DrawTrSurfTriangulation2d {
    pub fn new(triangulation: Triangulation2d) -> Self {
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
        let vertices = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0), Point2d::new(0.0, 1.0)];
        let triangles = vec![[0, 1, 2]];

        let triangulation = Triangulation2d::new(vertices, triangles);
        let drawable = DrawTrSurfTriangulation2d::new(triangulation);

        assert_eq!(drawable.vertex_count(), 3);
        assert_eq!(drawable.triangle_count(), 1);
    }
}
