// FILE: viewer_test_v3d_view.rs
// occt: ViewerTest_V3dView

#[derive(Clone, Debug)]
pub struct ViewerTestV3dView {
    name: String,
    width: i32,
    height: i32,
}

impl ViewerTestV3dView {
    pub fn new(name: &str, width: i32, height: i32) -> Self {
        ViewerTestV3dView {
            name: name.to_string(),
            width,
            height,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let v = ViewerTestV3dView::new("view1", 800, 600);
        assert_eq!(v.name(), "view1");
        assert_eq!(v.width(), 800);
        assert_eq!(v.height(), 600);
    }

    #[test]
    fn test_set_size() {
        let mut v = ViewerTestV3dView::new("view1", 800, 600);
        v.set_size(1024, 768);
        assert_eq!(v.width(), 1024);
        assert_eq!(v.height(), 768);
    }
}
