// FILE: iges_draw_segmented_views_visible.rs
// occt: IGESDraw_SegmentedViewsVisible

/// Segmented views visible entity
pub struct IgesDrawSegmentedViewsVisible {
    views: Vec<Box<dyn std::any::Any>>,
}

impl IgesDrawSegmentedViewsVisible {
    pub fn new() -> Self {
        IgesDrawSegmentedViewsVisible {
            views: Vec::new(),
        }
    }

    pub fn init(&mut self, views: Vec<Box<dyn std::any::Any>>) {
        self.views = views;
    }

    pub fn nb_views(&self) -> i32 {
        self.views.len() as i32
    }
}

impl Default for IgesDrawSegmentedViewsVisible {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let svv = IgesDrawSegmentedViewsVisible::new();
        assert_eq!(svv.nb_views(), 0);
    }

    #[test]
    fn test_init() {
        let mut svv = IgesDrawSegmentedViewsVisible::new();
        svv.init(vec![]);
        assert_eq!(svv.nb_views(), 0);
    }
}
