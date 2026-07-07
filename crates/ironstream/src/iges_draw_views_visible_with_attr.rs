// FILE: iges_draw_views_visible_with_attr.rs
// occt: IGESDraw_ViewsVisibleWithAttr

/// Views visible with attributes entity
pub struct IgesDrawViewsVisibleWithAttr {
    views: Vec<Box<dyn std::any::Any>>,
    attributes: Vec<String>,
}

impl IgesDrawViewsVisibleWithAttr {
    pub fn new() -> Self {
        IgesDrawViewsVisibleWithAttr {
            views: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn init(&mut self, views: Vec<Box<dyn std::any::Any>>, attributes: Vec<String>) {
        self.views = views;
        self.attributes = attributes;
    }

    pub fn nb_views(&self) -> i32 {
        self.views.len() as i32
    }

    pub fn nb_attributes(&self) -> i32 {
        self.attributes.len() as i32
    }
}

impl Default for IgesDrawViewsVisibleWithAttr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vva = IgesDrawViewsVisibleWithAttr::new();
        assert_eq!(vva.nb_views(), 0);
        assert_eq!(vva.nb_attributes(), 0);
    }

    #[test]
    fn test_init() {
        let mut vva = IgesDrawViewsVisibleWithAttr::new();
        vva.init(vec![], vec![]);
        assert_eq!(vva.nb_views(), 0);
        assert_eq!(vva.nb_attributes(), 0);
    }
}
