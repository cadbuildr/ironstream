// FILE: iges_solid_shell.rs
// occt: IGESSolid_Shell

/// Shell entity (Type 514, Form 1 for Closed or Form 2 for Open) in IGESSolid.
/// A shell is a connected 2D entity which divides R^3 into two arcwise connected open subsets.
/// One subset is finite (the inside).
#[derive(Debug, Clone)]
pub struct IGESSolidShell {
    /// Faces comprising the shell (simplified: stored as strings for Rust port)
    faces: Vec<String>,
    /// Orientation flags for each face (true = same direction, false = opposite)
    orientations: Vec<bool>,
    /// Is shell closed (form 1) or open (form 2)
    is_closed: bool,
}

impl IGESSolidShell {
    /// Creates a new Shell with default values.
    pub fn new() -> Self {
        Self {
            faces: Vec::new(),
            orientations: Vec::new(),
            is_closed: true, // Default to closed (Form 1)
        }
    }

    /// Initializes the Shell with faces and orientation flags.
    /// - faces: array of face entities
    /// - orientations: orientation flags (true/false for each face)
    /// Panics if lengths don't match.
    pub fn init(&mut self, faces: Vec<String>, orientations: Vec<bool>) {
        assert_eq!(
            faces.len(),
            orientations.len(),
            "faces and orientations must have the same length"
        );
        self.faces = faces;
        self.orientations = orientations;
    }

    /// Returns true if the Shell is Closed (FormNumber = 1).
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Sets the Closed status. true -> FormNumber 1, false -> FormNumber 2.
    pub fn set_closed(&mut self, closed: bool) {
        self.is_closed = closed;
    }

    /// Returns the FormNumber: 1 for closed, 2 for open.
    pub fn form_number(&self) -> i32 {
        if self.is_closed { 1 } else { 2 }
    }

    /// Returns the number of face entities in the shell.
    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    /// Returns the Index-th face (0-indexed in Rust, but presented as 1-indexed API).
    pub fn face(&self, index: usize) -> Option<&str> {
        if index > 0 && index <= self.faces.len() {
            Some(&self.faces[index - 1])
        } else {
            None
        }
    }

    /// Returns the orientation of the Index-th face (1-indexed).
    /// true = same direction as underlying surface, false = opposite.
    pub fn orientation(&self, index: usize) -> Option<bool> {
        if index > 0 && index <= self.orientations.len() {
            Some(self.orientations[index - 1])
        } else {
            None
        }
    }

    /// Returns all faces.
    pub fn faces(&self) -> &[String] {
        &self.faces
    }

    /// Returns all orientation flags.
    pub fn orientations(&self) -> &[bool] {
        &self.orientations
    }
}

impl Default for IGESSolidShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creation() {
        let shell = IGESSolidShell::new();
        assert!(shell.is_closed());
        assert_eq!(shell.nb_faces(), 0);
        assert_eq!(shell.form_number(), 1);
    }

    #[test]
    fn test_set_closed_open() {
        let mut shell = IGESSolidShell::new();
        shell.set_closed(false);
        assert!(!shell.is_closed());
        assert_eq!(shell.form_number(), 2);
    }

    #[test]
    fn test_init_faces_and_orientations() {
        let mut shell = IGESSolidShell::new();
        shell.init(
            vec![
                "Face_1".to_string(),
                "Face_2".to_string(),
                "Face_3".to_string(),
            ],
            vec![true, false, true],
        );

        assert_eq!(shell.nb_faces(), 3);
        assert_eq!(shell.face(1), Some("Face_1"));
        assert_eq!(shell.face(2), Some("Face_2"));
        assert_eq!(shell.face(3), Some("Face_3"));
        assert_eq!(shell.orientation(1), Some(true));
        assert_eq!(shell.orientation(2), Some(false));
        assert_eq!(shell.orientation(3), Some(true));
    }

    #[test]
    fn test_face_out_of_bounds() {
        let mut shell = IGESSolidShell::new();
        shell.init(vec!["Face_1".to_string()], vec![true]);

        assert_eq!(shell.face(0), None);
        assert_eq!(shell.face(1), Some("Face_1"));
        assert_eq!(shell.face(2), None);
    }

    #[test]
    fn test_orientation_out_of_bounds() {
        let mut shell = IGESSolidShell::new();
        shell.init(vec!["Face_1".to_string()], vec![true]);

        assert_eq!(shell.orientation(0), None);
        assert_eq!(shell.orientation(1), Some(true));
        assert_eq!(shell.orientation(2), None);
    }

    #[test]
    #[should_panic(expected = "must have the same length")]
    fn test_init_mismatched_lengths() {
        let mut shell = IGESSolidShell::new();
        shell.init(
            vec!["Face_1".to_string(), "Face_2".to_string()],
            vec![true],
        );
    }
}
