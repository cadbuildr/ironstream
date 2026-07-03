// FILE: top_ope_b_rep_ds_geometry_data.rs
// occt: TopOpeBRepDS_GeometryData

/// Mother-class of SurfaceData, CurveData, PointData
/// Contains a list of interferences
#[derive(Debug, Clone)]
pub struct GeometryData {
    /// List of interference indices (stored as usize)
    interferences: Vec<usize>,
}

impl GeometryData {
    /// Create new GeometryData
    pub fn new() -> Self {
        GeometryData {
            interferences: Vec::new(),
        }
    }

    /// Assign from another GeometryData
    pub fn assign(&mut self, other: &GeometryData) {
        self.interferences = other.interferences.clone();
    }

    /// Get interferences (read-only)
    pub fn interferences(&self) -> &[usize] {
        &self.interferences
    }

    /// Get mutable interferences
    pub fn interferences_mut(&mut self) -> &mut Vec<usize> {
        &mut self.interferences
    }

    /// Add an interference
    pub fn add_interference(&mut self, i: usize) {
        self.interferences.push(i);
    }

    /// Clear all interferences
    pub fn clear(&mut self) {
        self.interferences.clear();
    }
}

impl Default for GeometryData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_data_new() {
        let gd = GeometryData::new();
        assert_eq!(gd.interferences().len(), 0);
    }

    #[test]
    fn test_add_interference() {
        let mut gd = GeometryData::new();
        gd.add_interference(1);
        gd.add_interference(2);
        gd.add_interference(3);
        assert_eq!(gd.interferences().len(), 3);
        assert_eq!(gd.interferences()[0], 1);
        assert_eq!(gd.interferences()[1], 2);
        assert_eq!(gd.interferences()[2], 3);
    }

    #[test]
    fn test_assign() {
        let mut gd1 = GeometryData::new();
        gd1.add_interference(10);
        gd1.add_interference(20);

        let mut gd2 = GeometryData::new();
        gd2.assign(&gd1);
        assert_eq!(gd2.interferences().len(), 2);
        assert_eq!(gd2.interferences()[0], 10);
    }

    #[test]
    fn test_clear() {
        let mut gd = GeometryData::new();
        gd.add_interference(1);
        gd.add_interference(2);
        assert_eq!(gd.interferences().len(), 2);
        gd.clear();
        assert_eq!(gd.interferences().len(), 0);
    }

    #[test]
    fn test_interferences_mut() {
        let mut gd = GeometryData::new();
        {
            let vec = gd.interferences_mut();
            vec.push(5);
            vec.push(6);
        }
        assert_eq!(gd.interferences().len(), 2);
    }
}
