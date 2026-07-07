// FILE: vrml_texture_coordinate2.rs
// occt: Vrml_TextureCoordinate2
//
// Faithful port of OCCT Vrml_TextureCoordinate2 (DataExchange/TKDEVRML/Vrml/
// Vrml_TextureCoordinate2.hxx/.cxx): the VRML 1.0 `TextureCoordinate2` node.
// Stores an array of 2D texture coordinates.

/// Port of Vrml_TextureCoordinate2.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlTextureCoordinate2 {
    points: Vec<(f64, f64)>,
}

impl VrmlTextureCoordinate2 {
    /// Vrml_TextureCoordinate2 with empty coordinate list.
    pub fn new() -> Self {
        VrmlTextureCoordinate2 {
            points: Vec::new(),
        }
    }

    /// Vrml_TextureCoordinate2(aPoints).
    pub fn with_points(a_points: Vec<(f64, f64)>) -> Self {
        VrmlTextureCoordinate2 { points: a_points }
    }

    pub fn set_points(&mut self, a_points: Vec<(f64, f64)>) {
        self.points = a_points;
    }

    pub fn points(&self) -> &[(f64, f64)] {
        &self.points
    }

    pub fn point(&self, index: usize) -> Option<(f64, f64)> {
        self.points.get(index).copied()
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push((x, y));
    }

    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for VrmlTextureCoordinate2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty() {
        let tc = VrmlTextureCoordinate2::new();
        assert_eq!(tc.num_points(), 0);
        assert_eq!(tc.points().len(), 0);
    }

    #[test]
    fn with_points() {
        let points = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let tc = VrmlTextureCoordinate2::with_points(points.clone());
        assert_eq!(tc.num_points(), 4);
        assert_eq!(tc.points(), points.as_slice());
    }

    #[test]
    fn add_point() {
        let mut tc = VrmlTextureCoordinate2::new();
        tc.add_point(0.5, 0.5);
        tc.add_point(1.0, 1.0);
        assert_eq!(tc.num_points(), 2);
        assert_eq!(tc.point(0), Some((0.5, 0.5)));
        assert_eq!(tc.point(1), Some((1.0, 1.0)));
    }

    #[test]
    fn point_access() {
        let mut tc = VrmlTextureCoordinate2::new();
        tc.add_point(0.25, 0.75);
        assert_eq!(tc.point(0), Some((0.25, 0.75)));
        assert_eq!(tc.point(1), None);
    }

    #[test]
    fn set_points() {
        let mut tc = VrmlTextureCoordinate2::new();
        tc.add_point(0.0, 0.0);
        let new_points = vec![(0.5, 0.5), (1.0, 1.0)];
        tc.set_points(new_points.clone());
        assert_eq!(tc.points(), new_points.as_slice());
    }

    #[test]
    fn clear() {
        let mut tc = VrmlTextureCoordinate2::new();
        tc.add_point(1.0, 1.0);
        assert_eq!(tc.num_points(), 1);
        tc.clear();
        assert_eq!(tc.num_points(), 0);
    }

    #[test]
    fn equality() {
        let points1 = vec![(0.0, 0.0), (1.0, 1.0)];
        let points2 = vec![(0.0, 0.0), (1.0, 1.0)];
        let tc1 = VrmlTextureCoordinate2::with_points(points1);
        let tc2 = VrmlTextureCoordinate2::with_points(points2);
        assert_eq!(tc1, tc2);
    }
}
