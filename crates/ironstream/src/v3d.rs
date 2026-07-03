// FILE: v3d.rs

// occt: V3d_TypeOfOrientation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V3dTypeOfOrientation {
    XposYpos,
    XposYneg,
    XnegYpos,
    XnegYneg,
    XposZpos,
    XposZneg,
    XnegZpos,
    XnegZneg,
    YposZpos,
    YposZneg,
    YnegZpos,
    YnegZneg,
    XposYposZpos,
}

// occt: V3d_TypeOfProjection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V3dTypeOfProjection {
    Perspective,
    Orthographic,
}

// occt: V3d_View
pub struct V3dView {
    projection: V3dTypeOfProjection,
    width: u32,
    height: u32,
    scale: f64,
    eye: [f64; 3],
    at: [f64; 3],
    up: [f64; 3],
}

impl V3dView {
    pub fn new() -> Self {
        V3dView {
            projection: V3dTypeOfProjection::Orthographic,
            width: 800,
            height: 600,
            scale: 1.0,
            eye: [0.0, 0.0, 1.0],
            at: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
        }
    }

    pub fn set_projection(&mut self, p: V3dTypeOfProjection) {
        self.projection = p;
    }

    pub fn projection(&self) -> V3dTypeOfProjection {
        self.projection
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn set_eye(&mut self, x: f64, y: f64, z: f64) {
        self.eye = [x, y, z];
    }

    pub fn eye(&self) -> [f64; 3] {
        self.eye
    }

    pub fn set_at(&mut self, x: f64, y: f64, z: f64) {
        self.at = [x, y, z];
    }

    pub fn at(&self) -> [f64; 3] {
        self.at
    }

    pub fn set_scale(&mut self, s: f64) {
        self.scale = s;
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn fit_all(&mut self) {}

    pub fn update(&self) {}
}

impl Default for V3dView {
    fn default() -> Self {
        Self::new()
    }
}

// occt: V3d_Viewer
pub struct V3dViewer {
    views: Vec<V3dView>,
    background_color: [f32; 4],
}

impl V3dViewer {
    pub fn new() -> Self {
        V3dViewer {
            views: Vec::new(),
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn create_view() -> V3dView {
        V3dView::new()
    }

    pub fn add_view(&mut self, v: V3dView) {
        self.views.push(v);
    }

    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    pub fn set_background_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.background_color = [r, g, b, a];
    }

    pub fn background_color(&self) -> [f32; 4] {
        self.background_color
    }

    pub fn update(&self) {}
}

impl Default for V3dViewer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_default_projection_is_orthographic() {
        let v = V3dView::new();
        assert_eq!(v.projection(), V3dTypeOfProjection::Orthographic);
    }

    #[test]
    fn test_view_set_projection_perspective() {
        let mut v = V3dView::new();
        v.set_projection(V3dTypeOfProjection::Perspective);
        assert_eq!(v.projection(), V3dTypeOfProjection::Perspective);
    }

    #[test]
    fn test_view_default_size() {
        let v = V3dView::new();
        assert_eq!(v.size(), (800, 600));
    }

    #[test]
    fn test_view_set_size() {
        let mut v = V3dView::new();
        v.set_size(1920, 1080);
        assert_eq!(v.size(), (1920, 1080));
    }

    #[test]
    fn test_view_default_eye() {
        let v = V3dView::new();
        let eye = v.eye();
        assert_eq!(eye[0], 0.0);
        assert_eq!(eye[1], 0.0);
        assert_eq!(eye[2], 1.0);
    }

    #[test]
    fn test_view_set_eye() {
        let mut v = V3dView::new();
        v.set_eye(1.0, 2.0, 3.0);
        assert_eq!(v.eye(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_view_default_at_is_origin() {
        let v = V3dView::new();
        assert_eq!(v.at(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_view_set_at() {
        let mut v = V3dView::new();
        v.set_at(5.0, 6.0, 7.0);
        assert_eq!(v.at(), [5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_view_default_scale() {
        let v = V3dView::new();
        assert_eq!(v.scale(), 1.0);
    }

    #[test]
    fn test_view_set_scale() {
        let mut v = V3dView::new();
        v.set_scale(2.5);
        assert_eq!(v.scale(), 2.5);
    }

    #[test]
    fn test_view_fit_all_and_update_are_noops() {
        let mut v = V3dView::new();
        v.set_scale(3.0);
        v.fit_all();
        v.update();
        assert_eq!(v.scale(), 3.0);
    }

    #[test]
    fn test_viewer_starts_empty() {
        let viewer = V3dViewer::new();
        assert_eq!(viewer.view_count(), 0);
    }

    #[test]
    fn test_viewer_add_view_increments_count() {
        let mut viewer = V3dViewer::new();
        viewer.add_view(V3dViewer::create_view());
        assert_eq!(viewer.view_count(), 1);
        viewer.add_view(V3dViewer::create_view());
        assert_eq!(viewer.view_count(), 2);
    }

    #[test]
    fn test_viewer_default_background_color() {
        let viewer = V3dViewer::new();
        let bg = viewer.background_color();
        assert_eq!(bg, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_viewer_set_background_color() {
        let mut viewer = V3dViewer::new();
        viewer.set_background_color(0.2, 0.4, 0.6, 1.0);
        let bg = viewer.background_color();
        assert_eq!(bg[0], 0.2f32);
        assert_eq!(bg[1], 0.4f32);
        assert_eq!(bg[2], 0.6f32);
        assert_eq!(bg[3], 1.0f32);
    }

    #[test]
    fn test_viewer_update_is_noop() {
        let viewer = V3dViewer::new();
        viewer.update();
        assert_eq!(viewer.view_count(), 0);
    }

    #[test]
    fn test_orientation_variants_are_distinct() {
        let variants = [
            V3dTypeOfOrientation::XposYpos,
            V3dTypeOfOrientation::XposYneg,
            V3dTypeOfOrientation::XnegYpos,
            V3dTypeOfOrientation::XnegYneg,
            V3dTypeOfOrientation::XposZpos,
            V3dTypeOfOrientation::XposZneg,
            V3dTypeOfOrientation::XnegZpos,
            V3dTypeOfOrientation::XnegZneg,
            V3dTypeOfOrientation::YposZpos,
            V3dTypeOfOrientation::YposZneg,
            V3dTypeOfOrientation::YnegZpos,
            V3dTypeOfOrientation::YnegZneg,
            V3dTypeOfOrientation::XposYposZpos,
        ];
        assert_eq!(variants.len(), 13);
        for i in 0..variants.len() {
            for j in 0..variants.len() {
                if i == j {
                    assert_eq!(variants[i], variants[j]);
                } else {
                    assert_ne!(variants[i], variants[j]);
                }
            }
        }
    }

    #[test]
    fn test_projection_variants_distinct() {
        assert_ne!(
            V3dTypeOfProjection::Perspective,
            V3dTypeOfProjection::Orthographic
        );
    }

    #[test]
    fn test_enums_are_copy() {
        let o = V3dTypeOfOrientation::XposYposZpos;
        let o2 = o;
        assert_eq!(o, o2);

        let p = V3dTypeOfProjection::Perspective;
        let p2 = p;
        assert_eq!(p, p2);
    }
}
