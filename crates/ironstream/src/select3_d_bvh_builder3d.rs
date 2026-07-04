// FILE: select3_d_bvh_builder3d.rs
// occt: Select3D_BVHBuilder3d

/// Stub for Select3D_BVHBuilder3d from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_BVHBuilder3d {}

impl Select3D_BVHBuilder3d {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_BVHBuilder3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_bvh_builder3d_creation() {
        let _obj = Select3D_BVHBuilder3d::new();
        let _def = Select3D_BVHBuilder3d::default();
    }
}
