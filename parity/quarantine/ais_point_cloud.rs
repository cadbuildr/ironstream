// FILE: ais_point_cloud.rs
// occt: AIS_PointCloud

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, repr(u32))]
pub enum AisPointCloudDisplayMode {
    #[default]
    Points = 0,
    BndBox = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, repr(u32))]
pub enum AisPointCloudSelectionMode {
    #[default]
    Points = 0,
    SubsetOfPoints = 1,
    BndBox = 2,
}

#[derive(Clone, Debug)]
pub struct AisPointCloud {
    pub object_id: u32,
    pub point_count: usize,
    pub is_visible: bool,
}

impl AisPointCloud {
    pub fn new(object_id: u32, point_count: usize) -> Self {
        Self { object_id, point_count, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for AisPointCloud {
    fn default() -> Self { Self::new(0, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let pc = AisPointCloud::new(1, 100);
        assert_eq!(pc.object_id, 1);
        assert_eq!(pc.point_count, 100);
    }
}
