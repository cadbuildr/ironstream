// FILE: select3_d_point_data.rs
// occt: Select3D_PointData

/// Stub for Select3D_PointData from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_PointData {}

impl Select3D_PointData {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_PointData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_point_data_creation() {
        let _obj = Select3D_PointData::new();
        let _def = Select3D_PointData::default();
    }
}
