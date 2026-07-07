// FILE: step_visual_tessellated_surface_set.rs
// occt: StepVisual_TessellatedSurfaceSet

/// Represents a STEP TessellatedSurfaceSet entity.
pub struct TessellatedSurfaceSet {
    name: String,
    coordinates: Option<CoordinatesList>,
    pnmax: i32,
    normals: Vec<Vec<f64>>,
}

/// Placeholder for CoordinatesList
pub struct CoordinatesList;

impl TessellatedSurfaceSet {
    /// Creates a new tessellated surface set.
    pub fn new() -> Self {
        TessellatedSurfaceSet {
            name: String::new(),
            coordinates: None,
            pnmax: 0,
            normals: Vec::new(),
        }
    }

    /// Initializes all fields.
    pub fn init(&mut self, name: String, coordinates: Option<CoordinatesList>, pnmax: i32, normals: Vec<Vec<f64>>) {
        self.name = name;
        self.coordinates = coordinates;
        self.pnmax = pnmax;
        self.normals = normals;
    }

    /// Returns the coordinates.
    pub fn coordinates(&self) -> Option<&CoordinatesList> {
        self.coordinates.as_ref()
    }

    /// Sets the coordinates.
    pub fn set_coordinates(&mut self, coordinates: CoordinatesList) {
        self.coordinates = Some(coordinates);
    }

    /// Returns pnmax.
    pub fn pnmax(&self) -> i32 {
        self.pnmax
    }

    /// Sets pnmax.
    pub fn set_pnmax(&mut self, pnmax: i32) {
        self.pnmax = pnmax;
    }

    /// Returns the normals.
    pub fn normals(&self) -> &[Vec<f64>] {
        &self.normals
    }

    /// Sets the normals.
    pub fn set_normals(&mut self, normals: Vec<Vec<f64>>) {
        self.normals = normals;
    }

    /// Returns the number of normals.
    pub fn nb_normals(&self) -> usize {
        self.normals.len()
    }
}

impl Default for TessellatedSurfaceSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tss = TessellatedSurfaceSet::new();
        assert_eq!(tss.pnmax(), 0);
        assert_eq!(tss.nb_normals(), 0);
    }

    #[test]
    fn test_pnmax() {
        let mut tss = TessellatedSurfaceSet::new();
        tss.set_pnmax(42);
        assert_eq!(tss.pnmax(), 42);
    }

    #[test]
    fn test_normals() {
        let mut tss = TessellatedSurfaceSet::new();
        let normals = vec![vec![0.0, 0.0, 1.0]];
        tss.set_normals(normals);
        assert_eq!(tss.nb_normals(), 1);
    }
}
