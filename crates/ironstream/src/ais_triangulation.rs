// FILE: ais_triangulation.rs
// occt: AIS_Triangulation

#[derive(Clone, Debug)]
pub struct AisTriangulation {
    pub object_id: u32,
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub is_visible: bool,
}

impl AisTriangulation {
    pub fn new(object_id: u32, vertex_count: usize, triangle_count: usize) -> Self {
        Self { object_id, vertex_count, triangle_count, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for AisTriangulation {
    fn default() -> Self { Self::new(0, 0, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let tri = AisTriangulation::new(1, 100, 50);
        assert_eq!(tri.vertex_count, 100);
    }
}
