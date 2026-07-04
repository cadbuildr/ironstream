// FILE: iges_draw_views_visible.rs
// occt: IGESDraw_ViewsVisible

/// Views visible entity
pub struct IgesDrawViewsVisible {
    views: Vec<Box<dyn std::any::Any>>,
}

impl IgesDrawViewsVisible {
    pub fn new() -> Self {
        IgesDrawViewsVisible {
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

impl Default for IgesDrawViewsVisible {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vv = IgesDrawViewsVisible::new();
        assert_eq!(vv.nb_views(), 0);
    }

    #[test]
    fn test_init() {
        let mut vv = IgesDrawViewsVisible::new();
        vv.init(vec![]);
        assert_eq!(vv.nb_views(), 0);
    }
}
