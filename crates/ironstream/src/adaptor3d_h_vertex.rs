// FILE: adaptor3d_h_vertex.rs
// occt: Adaptor3d_HVertex

pub struct Adaptor3dHVertex {
    x: f64,
    y: f64,
    z: f64,
}

impl Adaptor3dHVertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Adaptor3dHVertex { x, y, z }
    }

    pub fn coord(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_h_vertex_new() {
        let vertex = Adaptor3dHVertex::new(1.0, 2.0, 3.0);
        let (x, y, z) = vertex.coord();
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 2.0).abs() < PRECISION);
        assert!((z - 3.0).abs() < PRECISION);
    }

    #[test]
    fn test_h_vertex_accessors() {
        let vertex = Adaptor3dHVertex::new(5.0, 6.0, 7.0);
        assert!((vertex.x() - 5.0).abs() < PRECISION);
        assert!((vertex.y() - 6.0).abs() < PRECISION);
        assert!((vertex.z() - 7.0).abs() < PRECISION);
    }
}
