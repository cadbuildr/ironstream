// FILE: rust/ironstream/crates/ironstream/src/aspect_window.rs

// occt: Aspect_Window // type — enum NativeWindow, OffscreenBuffer, VirtualWindow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspectWindowType {
    NativeWindow,
    OffscreenBuffer,
    VirtualWindow,
}

// occt-note: window creation params — fields: width u32, height u32, x i32, y i32,
//       title String, background_color [f32; 3]
#[derive(Debug, Clone)]
pub struct AspectWindowParams {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    title: String,
    background_color: [f32; 3],
}

impl AspectWindowParams {
    /// Create params with the given dimensions; all other fields take defaults:
    /// x=0, y=0, title="", background_color=[0.5, 0.5, 0.5].
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            x: 0,
            y: 0,
            title: String::new(),
            background_color: [0.5, 0.5, 0.5],
        }
    }

    /// Set the window title.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    /// Return the window title as a string slice.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the background colour as an RGB triple (each component in [0, 1]).
    pub fn set_background_color(&mut self, color: [f32; 3]) {
        self.background_color = color;
    }

    /// Return the background colour as an RGB triple.
    pub fn background_color(&self) -> [f32; 3] {
        self.background_color
    }

    /// Set the top-left corner position of the window in screen coordinates.
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Return the top-left corner position as `(x, y)`.
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// occt: Aspect_Window // stub — fields: params AspectWindowParams,
//       window_type AspectWindowType, is_mapped bool
#[derive(Debug, Clone)]
pub struct AspectWindow {
    params: AspectWindowParams,
    window_type: AspectWindowType,
    is_mapped: bool,
}

impl AspectWindow {
    /// Create a new window descriptor with the given params and window type.
    /// The window starts in the unmapped (invisible) state.
    pub fn new(params: AspectWindowParams, window_type: AspectWindowType) -> Self {
        Self {
            params,
            window_type,
            is_mapped: false,
        }
    }

    /// Return a reference to the window creation params.
    pub fn params(&self) -> &AspectWindowParams {
        &self.params
    }

    /// Return the window type.
    pub fn window_type(&self) -> AspectWindowType {
        self.window_type
    }

    /// Return the window width in pixels.
    pub fn width(&self) -> u32 {
        self.params.width
    }

    /// Return the window height in pixels.
    pub fn height(&self) -> u32 {
        self.params.height
    }

    /// Mark the window as mapped (visible/active).
    pub fn map(&mut self) {
        self.is_mapped = true;
    }

    /// Mark the window as unmapped (hidden/inactive).
    pub fn unmap(&mut self) {
        self.is_mapped = false;
    }

    /// Return true if the window is currently mapped.
    pub fn is_mapped(&self) -> bool {
        self.is_mapped
    }

    /// Return the top-left corner position as `(x, y)`.
    pub fn position(&self) -> (i32, i32) {
        self.params.position()
    }

    /// Return the window dimensions as `(width, height)`.
    pub fn size(&self) -> (u32, u32) {
        (self.params.width, self.params.height)
    }

    /// Return the aspect ratio (width / height).
    ///
    /// Returns 1.0 if the height is zero to avoid division by zero.
    pub fn aspect_ratio(&self) -> f32 {
        if self.params.height == 0 {
            1.0
        } else {
            self.params.width as f32 / self.params.height as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_new_defaults() {
        let p = AspectWindowParams::new(800, 600);
        assert_eq!(p.width, 800);
        assert_eq!(p.height, 600);
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
        assert_eq!(p.title(), "");
        let bg = p.background_color();
        assert!((bg[0] - 0.5).abs() < f32::EPSILON);
        assert!((bg[1] - 0.5).abs() < f32::EPSILON);
        assert!((bg[2] - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn params_set_title() {
        let mut p = AspectWindowParams::new(100, 100);
        p.set_title("My Window");
        assert_eq!(p.title(), "My Window");
    }

    #[test]
    fn params_set_background_color() {
        let mut p = AspectWindowParams::new(100, 100);
        p.set_background_color([0.1, 0.2, 0.3]);
        let bg = p.background_color();
        assert!((bg[0] - 0.1).abs() < f32::EPSILON);
        assert!((bg[1] - 0.2).abs() < f32::EPSILON);
        assert!((bg[2] - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn params_set_position() {
        let mut p = AspectWindowParams::new(100, 100);
        p.set_position(50, -20);
        assert_eq!(p.position(), (50, -20));
    }

    #[test]
    fn window_new_unmapped() {
        let p = AspectWindowParams::new(1920, 1080);
        let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
        assert!(!w.is_mapped());
    }

    #[test]
    fn window_map_unmap() {
        let p = AspectWindowParams::new(640, 480);
        let mut w = AspectWindow::new(p, AspectWindowType::VirtualWindow);
        assert!(!w.is_mapped());
        w.map();
        assert!(w.is_mapped());
        w.unmap();
        assert!(!w.is_mapped());
    }

    #[test]
    fn window_width_height() {
        let p = AspectWindowParams::new(1024, 768);
        let w = AspectWindow::new(p, AspectWindowType::OffscreenBuffer);
        assert_eq!(w.width(), 1024);
        assert_eq!(w.height(), 768);
    }

    #[test]
    fn window_size() {
        let p = AspectWindowParams::new(320, 240);
        let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
        assert_eq!(w.size(), (320, 240));
    }

    #[test]
    fn window_position() {
        let mut p = AspectWindowParams::new(100, 100);
        p.set_position(10, 20);
        let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
        assert_eq!(w.position(), (10, 20));
    }

    #[test]
    fn window_aspect_ratio() {
        let p = AspectWindowParams::new(1920, 1080);
        let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
        let ratio = w.aspect_ratio();
        assert!((ratio - 16.0 / 9.0).abs() < 1e-5);
    }

    #[test]
    fn window_aspect_ratio_square() {
        let p = AspectWindowParams::new(512, 512);
        let w = AspectWindow::new(p, AspectWindowType::NativeWindow);
        assert!((w.aspect_ratio() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn window_aspect_ratio_zero_height() {
        let p = AspectWindowParams::new(100, 0);
        let w = AspectWindow::new(p, AspectWindowType::OffscreenBuffer);
        assert!((w.aspect_ratio() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn window_type_returned_correctly() {
        let p = AspectWindowParams::new(100, 100);
        let w = AspectWindow::new(p, AspectWindowType::OffscreenBuffer);
        assert_eq!(w.window_type(), AspectWindowType::OffscreenBuffer);
    }

    #[test]
    fn window_params_ref() {
        let mut p = AspectWindowParams::new(200, 150);
        p.set_title("test");
        let w = AspectWindow::new(p, AspectWindowType::VirtualWindow);
        assert_eq!(w.params().title(), "test");
        assert_eq!(w.params().width, 200);
        assert_eq!(w.params().height, 150);
    }

    #[test]
    fn window_type_all_variants_distinct() {
        let variants = [
            AspectWindowType::NativeWindow,
            AspectWindowType::OffscreenBuffer,
            AspectWindowType::VirtualWindow,
        ];
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
    fn window_type_is_copy() {
        let t = AspectWindowType::NativeWindow;
        let t2 = t;
        assert_eq!(t, t2);
    }

    #[test]
    fn params_clone() {
        let mut p = AspectWindowParams::new(400, 300);
        p.set_title("clone test");
        p.set_position(5, 10);
        let p2 = p.clone();
        assert_eq!(p2.title(), "clone test");
        assert_eq!(p2.position(), (5, 10));
        assert_eq!(p2.width, 400);
        assert_eq!(p2.height, 300);
    }

    #[test]
    fn window_clone() {
        let p = AspectWindowParams::new(800, 600);
        let mut w = AspectWindow::new(p, AspectWindowType::NativeWindow);
        w.map();
        let w2 = w.clone();
        assert!(w2.is_mapped());
        assert_eq!(w2.width(), 800);
        assert_eq!(w2.height(), 600);
        assert_eq!(w2.window_type(), AspectWindowType::NativeWindow);
    }

    #[test]
    fn window_type_debug() {
        assert_eq!(format!("{:?}", AspectWindowType::NativeWindow), "NativeWindow");
        assert_eq!(format!("{:?}", AspectWindowType::OffscreenBuffer), "OffscreenBuffer");
        assert_eq!(format!("{:?}", AspectWindowType::VirtualWindow), "VirtualWindow");
    }
}
