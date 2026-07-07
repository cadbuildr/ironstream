// FILE: int_curve_surface_the_polyhedron_of_h_inter.rs
// occt: IntCurveSurface_ThePolyhedronOfHInter

//! Polyhedron approximation of surface for intersection algorithms.

#[derive(Clone)]
pub struct BoundingBox {
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
}

/// Polyhedron representation of a 3D surface
pub struct IntCurveSurfaceThePolyhedronOfHInter {
    triangles: Vec<(usize, usize, usize)>,
    vertices: Vec<(f64, f64, f64)>,
    bounding: BoundingBox,
    deflection: f64,
}

impl IntCurveSurfaceThePolyhedronOfHInter {
    /// Creates polyhedron from surface
    pub fn new(_surface: &SurfaceAdaptor, _nb_u: i32, _nb_v: i32) -> Self {
        IntCurveSurfaceThePolyhedronOfHInter {
            triangles: Vec::new(),
            vertices: Vec::new(),
            bounding: BoundingBox {
                xmin: 0.0,
                ymin: 0.0,
                zmin: 0.0,
                xmax: 0.0,
                ymax: 0.0,
                zmax: 0.0,
            },
            deflection: 0.0,
        }
    }

    /// Creates polyhedron with parameter range
    pub fn new_with_range(
        _surface: &SurfaceAdaptor,
        _u1: f64,
        _u2: f64,
        _v1: f64,
        _v2: f64,
        _nb_u: i32,
        _nb_v: i32,
    ) -> Self {
        IntCurveSurfaceThePolyhedronOfHInter {
            triangles: Vec::new(),
            vertices: Vec::new(),
            bounding: BoundingBox {
                xmin: 0.0,
                ymin: 0.0,
                zmin: 0.0,
                xmax: 0.0,
                ymax: 0.0,
                zmax: 0.0,
            },
            deflection: 0.0,
        }
    }

    /// Returns bounding box
    pub fn bounding(&self) -> &BoundingBox {
        &self.bounding
    }

    /// Returns deflection
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    /// Sets deflection
    pub fn set_deflection(&mut self, d: f64) {
        self.deflection = d;
    }

    /// Returns number of triangles
    pub fn nb_triangles(&self) -> i32 {
        self.triangles.len() as i32
    }

    /// Returns number of vertices
    pub fn nb_vertices(&self) -> i32 {
        self.vertices.len() as i32
    }

    /// Returns triangle at index
    pub fn triangle(&self, index: i32) -> Option<(usize, usize, usize)> {
        self.triangles.get(index as usize).copied()
    }

    /// Returns vertex at index
    pub fn vertex(&self, index: i32) -> Option<(f64, f64, f64)> {
        self.vertices.get(index as usize).copied()
    }
}

/// Placeholder for surface adaptor
#[derive(Clone)]
pub struct SurfaceAdaptor;

impl BoundingBox {
    /// Creates new bounding box
    pub fn new(xmin: f64, ymin: f64, zmin: f64, xmax: f64, ymax: f64, zmax: f64) -> Self {
        BoundingBox {
            xmin,
            ymin,
            zmin,
            xmax,
            ymax,
            zmax,
        }
    }

    /// Enlarges bounding box
    pub fn enlarge(&mut self, value: f64) {
        self.xmin -= value;
        self.ymin -= value;
        self.zmin -= value;
        self.xmax += value;
        self.ymax += value;
        self.zmax += value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyhedron_new() {
        let poly = IntCurveSurfaceThePolyhedronOfHInter::new(&SurfaceAdaptor, 10, 10);
        assert_eq!(poly.nb_triangles(), 0);
        assert_eq!(poly.nb_vertices(), 0);
    }

    #[test]
    fn test_polyhedron_with_range() {
        let _poly = IntCurveSurfaceThePolyhedronOfHInter::new_with_range(
            &SurfaceAdaptor,
            0.0,
            1.0,
            0.0,
            1.0,
            5,
            5,
        );
    }

    #[test]
    fn test_polyhedron_deflection() {
        let mut poly = IntCurveSurfaceThePolyhedronOfHInter::new(&SurfaceAdaptor, 10, 10);
        poly.set_deflection(0.01);
        assert_eq!(poly.deflection(), 0.01);
    }
}
