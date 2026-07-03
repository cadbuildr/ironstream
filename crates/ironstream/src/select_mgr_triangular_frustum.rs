// FILE: select_mgr_triangular_frustum.rs
// occt: SelectMgr_TriangularFrustum

/// Triangular frustum for pyramid-shaped selection volumes.
#[derive(Clone, Debug)]
pub struct TriangularFrustum {
    pub frustum_id: u32,
    pub apex: [f64; 3],
    pub p1: [f64; 3],
    pub p2: [f64; 3],
    pub p3: [f64; 3],
}

impl TriangularFrustum {
    pub fn new(frustum_id: u32, apex: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> Self {
        Self {
            frustum_id,
            apex,
            p1,
            p2,
            p3,
        }
    }

    pub fn set_base_triangle(&mut self, p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) {
        self.p1 = p1;
        self.p2 = p2;
        self.p3 = p3;
    }

    pub fn set_apex(&mut self, apex: [f64; 3]) {
        self.apex = apex;
    }

    pub fn volume(&self) -> f64 {
        let v1 = [self.p1[0] - self.apex[0], self.p1[1] - self.apex[1], self.p1[2] - self.apex[2]];
        let v2 = [self.p2[0] - self.apex[0], self.p2[1] - self.apex[1], self.p2[2] - self.apex[2]];
        let v3 = [self.p3[0] - self.apex[0], self.p3[1] - self.apex[1], self.p3[2] - self.apex[2]];

        let cross = [
            v1[1] * v2[2] - v1[2] * v2[1],
            v1[2] * v2[0] - v1[0] * v2[2],
            v1[0] * v2[1] - v1[1] * v2[0],
        ];

        let det = (cross[0] * v3[0] + cross[1] * v3[1] + cross[2] * v3[2]).abs();
        det / 6.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_frustum() {
        let frustum = TriangularFrustum::new(
            1,
            [0.0, 0.0, -1.0],
            [1.0, 0.0, 0.0],
            [-0.5, 1.0, 0.0],
            [-0.5, -1.0, 0.0],
        );
        assert_eq!(frustum.frustum_id, 1);
    }

    #[test]
    fn set_base_triangle() {
        let mut frustum = TriangularFrustum::new(
            1,
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        );
        frustum.set_base_triangle([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        assert_eq!(frustum.p1, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn volume() {
        let frustum = TriangularFrustum::new(
            1,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        );
        let v = frustum.volume();
        assert!((v - 1.0 / 6.0).abs() < 1e-10);
    }
}
