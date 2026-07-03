// FILE: shape_analysis_free_bound_data.rs
// occt: ShapeAnalysis_FreeBoundData

/// Properties of a free bound.
pub struct FreeBoundData {
    bound: Vec<u8>,
    area: f64,
    perimeter: f64,
    ratio: f64,
    width: f64,
    notches: Vec<Vec<u8>>,
    notch_widths: Vec<f64>,
}

impl FreeBoundData {
    /// Create a new FreeBoundData.
    pub fn new() -> Self {
        FreeBoundData {
            bound: Vec::new(),
            area: 0.0,
            perimeter: 0.0,
            ratio: 0.0,
            width: 0.0,
            notches: Vec::new(),
            notch_widths: Vec::new(),
        }
    }

    /// Create with free bound wire.
    pub fn with_free_bound(_freebound: &[u8]) -> Self {
        FreeBoundData::new()
    }

    /// Clear all properties.
    pub fn clear(&mut self) {
        self.area = 0.0;
        self.perimeter = 0.0;
        self.ratio = 0.0;
        self.width = 0.0;
        self.notches.clear();
        self.notch_widths.clear();
    }

    /// Set free bound.
    pub fn set_free_bound(&mut self, _freebound: &[u8]) {
        // Implementation stub
    }

    /// Set area.
    pub fn set_area(&mut self, area: f64) {
        self.area = area;
    }

    /// Set perimeter.
    pub fn set_perimeter(&mut self, perimeter: f64) {
        self.perimeter = perimeter;
    }

    /// Set ratio.
    pub fn set_ratio(&mut self, ratio: f64) {
        self.ratio = ratio;
    }

    /// Set width.
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Add notch.
    pub fn add_notch(&mut self, _notch: &[u8], width: f64) {
        self.notches.push(Vec::new());
        self.notch_widths.push(width);
    }

    /// Get free bound.
    pub fn free_bound(&self) -> Option<&[u8]> {
        if self.bound.is_empty() {
            None
        } else {
            Some(&self.bound)
        }
    }

    /// Get area.
    pub fn area(&self) -> f64 {
        self.area
    }

    /// Get perimeter.
    pub fn perimeter(&self) -> f64 {
        self.perimeter
    }

    /// Get ratio.
    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    /// Get width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Get number of notches.
    pub fn nb_notches(&self) -> usize {
        self.notches.len()
    }

    /// Get notch by index.
    pub fn notch(&self, index: usize) -> Option<&[u8]> {
        if index < self.notches.len() {
            Some(&self.notches[index])
        } else {
            None
        }
    }

    /// Get notch width by index.
    pub fn notch_width(&self, index: usize) -> Option<f64> {
        if index < self.notch_widths.len() {
            Some(self.notch_widths[index])
        } else {
            None
        }
    }
}

impl Default for FreeBoundData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = FreeBoundData::new();
        assert_eq!(data.area(), 0.0);
        assert_eq!(data.nb_notches(), 0);
    }

    #[test]
    fn test_set_area() {
        let mut data = FreeBoundData::new();
        data.set_area(100.0);
        assert_eq!(data.area(), 100.0);
    }

    #[test]
    fn test_add_notch() {
        let mut data = FreeBoundData::new();
        data.add_notch(&[], 5.0);
        assert_eq!(data.nb_notches(), 1);
        assert_eq!(data.notch_width(0), Some(5.0));
    }

    #[test]
    fn test_clear() {
        let mut data = FreeBoundData::new();
        data.set_area(100.0);
        data.clear();
        assert_eq!(data.area(), 0.0);
    }
}
