// FILE: t_data_std_note_book.rs
// occt: TDataStd_NoteBook

/// A NoteBook attribute for managing numeric values and expressions.
/// Used to store real and integer values as child attributes.
#[derive(Clone, Debug, Default)]
pub struct TDataStd_NoteBook {
    id: [u8; 16],
    reals: Vec<f64>,
    integers: Vec<i32>,
    exported_reals: Vec<bool>,
    exported_integers: Vec<bool>,
}

impl TDataStd_NoteBook {
    /// Create a new NoteBook attribute.
    pub fn new() -> Self {
        Self {
            id: Self::get_id(),
            reals: Vec::new(),
            integers: Vec::new(),
            exported_reals: Vec::new(),
            exported_integers: Vec::new(),
        }
    }

    /// Get the standard GUID for NoteBook attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_NoteBook
        [
            0x9D, 0xF2, 0x1A, 0x88, 0x0F, 0x4C, 0x44, 0x6D, 0x89, 0x7B, 0x56, 0x2F, 0x33, 0x22,
            0x22, 0x22,
        ]
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Append a real value to the notebook.
    pub fn append_real(&mut self, value: f64, is_exported: bool) -> usize {
        self.reals.push(value);
        self.exported_reals.push(is_exported);
        self.reals.len() - 1
    }

    /// Append an integer value to the notebook.
    pub fn append_integer(&mut self, value: i32, is_exported: bool) -> usize {
        self.integers.push(value);
        self.exported_integers.push(is_exported);
        self.integers.len() - 1
    }

    /// Get all real values.
    pub fn reals(&self) -> &[f64] {
        &self.reals
    }

    /// Get all integer values.
    pub fn integers(&self) -> &[i32] {
        &self.integers
    }

    /// Check if a real value is exported.
    pub fn is_real_exported(&self, index: usize) -> bool {
        self.exported_reals.get(index).copied().unwrap_or(false)
    }

    /// Check if an integer value is exported.
    pub fn is_integer_exported(&self, index: usize) -> bool {
        self.exported_integers.get(index).copied().unwrap_or(false)
    }

    /// Get the number of real values.
    pub fn real_count(&self) -> usize {
        self.reals.len()
    }

    /// Get the number of integer values.
    pub fn integer_count(&self) -> usize {
        self.integers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_notebook() {
        let notebook = TDataStd_NoteBook::new();
        assert_eq!(notebook.real_count(), 0);
        assert_eq!(notebook.integer_count(), 0);
    }

    #[test]
    fn test_append_real() {
        let mut notebook = TDataStd_NoteBook::new();
        let idx1 = notebook.append_real(3.14, false);
        let idx2 = notebook.append_real(2.71, true);
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(notebook.real_count(), 2);
    }

    #[test]
    fn test_append_integer() {
        let mut notebook = TDataStd_NoteBook::new();
        let idx1 = notebook.append_integer(42, false);
        let idx2 = notebook.append_integer(100, true);
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(notebook.integer_count(), 2);
    }

    #[test]
    fn test_export_flags() {
        let mut notebook = TDataStd_NoteBook::new();
        notebook.append_real(1.0, true);
        notebook.append_real(2.0, false);
        assert!(notebook.is_real_exported(0));
        assert!(!notebook.is_real_exported(1));

        notebook.append_integer(10, false);
        notebook.append_integer(20, true);
        assert!(!notebook.is_integer_exported(0));
        assert!(notebook.is_integer_exported(1));
    }

    #[test]
    fn test_get_values() {
        let mut notebook = TDataStd_NoteBook::new();
        notebook.append_real(1.5, false);
        notebook.append_real(2.5, false);
        notebook.append_integer(10, false);
        notebook.append_integer(20, false);

        assert_eq!(notebook.reals(), &[1.5, 2.5]);
        assert_eq!(notebook.integers(), &[10, 20]);
    }

    #[test]
    fn test_default() {
        let notebook = TDataStd_NoteBook::default();
        assert_eq!(notebook.real_count(), 0);
    }
}
