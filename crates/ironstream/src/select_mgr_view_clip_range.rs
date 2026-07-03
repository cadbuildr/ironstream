// FILE: select_mgr_view_clip_range.rs
// occt: SelectMgr_ViewClipRange

/// Manages depth clipping ranges for view-based selection.
#[derive(Clone, Debug)]
pub struct ViewClipRange {
    pub unclip_range: (f64, f64),
    pub clip_ranges: Vec<(f64, f64)>,
}

impl ViewClipRange {
    pub fn new() -> Self {
        Self {
            unclip_range: (f64::MAX, -f64::MAX),
            clip_ranges: Vec::new(),
        }
    }

    pub fn set_void(&mut self) {
        self.unclip_range = (f64::MAX, -f64::MAX);
        self.clip_ranges.clear();
    }

    pub fn is_void(&self) -> bool {
        self.unclip_range.0 > self.unclip_range.1
    }

    pub fn is_clipped(&self, depth: f64) -> bool {
        if !self.is_void() && (depth < self.unclip_range.0 || depth > self.unclip_range.1) {
            return true;
        }
        for &(min, max) in &self.clip_ranges {
            if depth >= min && depth <= max {
                return true;
            }
        }
        false
    }

    pub fn add_clip_range(&mut self, min: f64, max: f64) {
        if min <= max {
            self.clip_ranges.push((min, max));
        }
    }

    pub fn get_nearest_depth(&self, range_min: f64, range_max: f64) -> Option<f64> {
        if !self.is_void() && (range_max < self.unclip_range.0 || range_min > self.unclip_range.1) {
            return None;
        }

        let mut depth = range_min;
        if !self.is_void() && depth < self.unclip_range.0 {
            depth = self.unclip_range.0;
        }

        if depth <= range_max {
            Some(depth)
        } else {
            None
        }
    }
}

impl Default for ViewClipRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_range() {
        let range = ViewClipRange::new();
        assert!(range.is_void());
    }

    #[test]
    fn is_clipped() {
        let mut range = ViewClipRange::new();
        range.add_clip_range(5.0, 10.0);
        assert!(range.is_clipped(7.0));
        assert!(!range.is_clipped(12.0));
    }

    #[test]
    fn add_clip_range() {
        let mut range = ViewClipRange::new();
        range.add_clip_range(0.0, 10.0);
        range.add_clip_range(20.0, 30.0);
        assert_eq!(range.clip_ranges.len(), 2);
    }

    #[test]
    fn get_nearest_depth() {
        let mut range = ViewClipRange::new();
        range.unclip_range = (0.0, 1000.0);
        let depth = range.get_nearest_depth(100.0, 200.0);
        assert_eq!(depth, Some(100.0));
    }

    #[test]
    fn set_void() {
        let mut range = ViewClipRange::new();
        range.unclip_range = (0.0, 100.0);
        assert!(!range.is_void());
        range.set_void();
        assert!(range.is_void());
    }
}
