// FILE: select3_d_bnd_box3d.rs
// occt: Select3D_BndBox3d

/// Stub for Select3D_BndBox3d from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_BndBox3d {}

impl Select3D_BndBox3d {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_BndBox3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_bnd_box3d_creation() {
        let _obj = Select3D_BndBox3d::new();
        let _def = Select3D_BndBox3d::default();
    }
}
