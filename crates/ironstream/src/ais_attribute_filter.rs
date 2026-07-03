// FILE: ais_attribute_filter.rs
// occt: AIS_AttributeFilter

#[derive(Clone, Debug, Default)]
pub struct AttributeFilter {
    color: Option<[u8; 3]>,
    transparency: Option<f32>,
}

impl AttributeFilter {
    pub fn new() -> Self { Self::default() }
    pub fn color(&self) -> Option<[u8; 3]> { self.color }
    pub fn set_color(&mut self, c: Option<[u8; 3]>) { self.color = c; }
    pub fn transparency(&self) -> Option<f32> { self.transparency }
    pub fn set_transparency(&mut self, t: Option<f32>) { self.transparency = t.map(|v| v.max(0.0).min(1.0)); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let f = AttributeFilter::new();
        assert!(f.color().is_none());
    }
}
