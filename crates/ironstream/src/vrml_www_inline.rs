// FILE: vrml_www_inline.rs
// occt: Vrml_WWWInline
//
// Faithful port of OCCT Vrml_WWWInline (DataExchange/TKDEVRML/Vrml/
// Vrml_WWWInline.hxx/.cxx): the VRML 1.0 `WWWInline` node.
// References external VRML files to be inlined into the scene.

use std::collections::VecDeque;

/// Port of Vrml_WWWInline.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlWwwInline {
    name: String,
    urls: VecDeque<String>,
    bounding_box_x: f64,
    bounding_box_y: f64,
    bounding_box_z: f64,
    bounding_box_size_x: f64,
    bounding_box_size_y: f64,
    bounding_box_size_z: f64,
}

impl VrmlWwwInline {
    /// Vrml_WWWInline with default values.
    pub fn new() -> Self {
        VrmlWwwInline {
            name: String::new(),
            urls: VecDeque::new(),
            bounding_box_x: 0.0,
            bounding_box_y: 0.0,
            bounding_box_z: 0.0,
            bounding_box_size_x: 0.0,
            bounding_box_size_y: 0.0,
            bounding_box_size_z: 0.0,
        }
    }

    /// Vrml_WWWInline(aName).
    pub fn with_name(a_name: &str) -> Self {
        VrmlWwwInline {
            name: a_name.to_string(),
            urls: VecDeque::new(),
            bounding_box_x: 0.0,
            bounding_box_y: 0.0,
            bounding_box_z: 0.0,
            bounding_box_size_x: 0.0,
            bounding_box_size_y: 0.0,
            bounding_box_size_z: 0.0,
        }
    }

    pub fn set_name(&mut self, a_name: &str) {
        self.name = a_name.to_string();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_url(&mut self, url: &str) {
        self.urls.push_back(url.to_string());
    }

    pub fn url(&self, index: usize) -> Option<&str> {
        self.urls.get(index).map(|u| u.as_str())
    }

    pub fn urls(&self) -> Vec<&str> {
        self.urls.iter().map(|u| u.as_str()).collect()
    }

    pub fn num_urls(&self) -> usize {
        self.urls.len()
    }

    pub fn clear_urls(&mut self) {
        self.urls.clear();
    }

    pub fn set_bounding_box(&mut self, x: f64, y: f64, z: f64, size_x: f64, size_y: f64, size_z: f64) {
        self.bounding_box_x = x;
        self.bounding_box_y = y;
        self.bounding_box_z = z;
        self.bounding_box_size_x = size_x;
        self.bounding_box_size_y = size_y;
        self.bounding_box_size_z = size_z;
    }

    pub fn bounding_box_center(&self) -> (f64, f64, f64) {
        (self.bounding_box_x, self.bounding_box_y, self.bounding_box_z)
    }

    pub fn bounding_box_size(&self) -> (f64, f64, f64) {
        (self.bounding_box_size_x, self.bounding_box_size_y, self.bounding_box_size_z)
    }
}

impl Default for VrmlWwwInline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_inline() {
        let inline = VrmlWwwInline::new();
        assert_eq!(inline.name(), "");
        assert_eq!(inline.num_urls(), 0);
        assert_eq!(inline.bounding_box_center(), (0.0, 0.0, 0.0));
        assert_eq!(inline.bounding_box_size(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn with_name() {
        let inline = VrmlWwwInline::with_name("scene.wrl");
        assert_eq!(inline.name(), "scene.wrl");
    }

    #[test]
    fn add_url() {
        let mut inline = VrmlWwwInline::new();
        inline.add_url("http://example.com/scene.wrl");
        inline.add_url("local/fallback.wrl");
        assert_eq!(inline.num_urls(), 2);
        assert_eq!(inline.url(0), Some("http://example.com/scene.wrl"));
        assert_eq!(inline.url(1), Some("local/fallback.wrl"));
    }

    #[test]
    fn urls() {
        let mut inline = VrmlWwwInline::new();
        inline.add_url("url1");
        inline.add_url("url2");
        let url_list = inline.urls();
        assert_eq!(url_list.len(), 2);
        assert_eq!(url_list[0], "url1");
        assert_eq!(url_list[1], "url2");
    }

    #[test]
    fn clear_urls() {
        let mut inline = VrmlWwwInline::new();
        inline.add_url("url1");
        inline.add_url("url2");
        assert_eq!(inline.num_urls(), 2);
        inline.clear_urls();
        assert_eq!(inline.num_urls(), 0);
    }

    #[test]
    fn set_bounding_box() {
        let mut inline = VrmlWwwInline::new();
        inline.set_bounding_box(1.0, 2.0, 3.0, 10.0, 20.0, 30.0);
        assert_eq!(inline.bounding_box_center(), (1.0, 2.0, 3.0));
        assert_eq!(inline.bounding_box_size(), (10.0, 20.0, 30.0));
    }

    #[test]
    fn combined_setup() {
        let mut inline = VrmlWwwInline::with_name("models/part.wrl");
        inline.add_url("http://cdn.example.com/part.wrl");
        inline.add_url("local/part.wrl");
        inline.set_bounding_box(0.0, 0.0, 0.0, 5.0, 5.0, 5.0);
        assert_eq!(inline.name(), "models/part.wrl");
        assert_eq!(inline.num_urls(), 2);
        assert_eq!(inline.bounding_box_size(), (5.0, 5.0, 5.0));
    }

    #[test]
    fn url_not_found() {
        let inline = VrmlWwwInline::new();
        assert_eq!(inline.url(0), None);
    }
}
