// FILE: prs3d_datum_aspect.rs
// occt: Prs3d_DatumAspect

#[derive(Clone, Debug)]
pub struct Prs3dDatumAspect {
    pub aspect_id: u32,
    pub axis_length: f64,
    pub number_of_facettes: u32,
}

impl Prs3dDatumAspect {
    pub fn new(aspect_id: u32) -> Self {
        Self { aspect_id, axis_length: 10.0, number_of_facettes: 20 }
    }
    pub fn set_axis_length(&mut self, len: f64) { self.axis_length = len.max(0.01); }
}

impl Default for Prs3dDatumAspect {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let d = Prs3dDatumAspect::new(1);
        assert_eq!(d.axis_length, 10.0);
    }
}
