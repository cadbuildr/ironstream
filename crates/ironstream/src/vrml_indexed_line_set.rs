// FILE: vrml_indexed_line_set.rs
// occt: Vrml_IndexedLineSet
//
// Faithful port of OCCT Vrml_IndexedLineSet (DataExchange/TKDEVRML/Vrml/
// Vrml_IndexedLineSet.hxx): the VRML 1.0 `IndexedLineSet` geometry node,
// storing indexed line data with vertex and color indices.
// Uses positive indices for vertex/color data and negative/zero terminators.
// Default has empty index arrays. Print emits field arrays in VRML syntax.

/// Port of Vrml_IndexedLineSet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlIndexedLineSet {
    my_coord_index: Vec<i32>,
    my_color_index: Vec<i32>,
}

impl VrmlIndexedLineSet {
    /// Vrml_IndexedLineSet(): all index arrays empty.
    pub fn new() -> Self {
        VrmlIndexedLineSet {
            my_coord_index: Vec::new(),
            my_color_index: Vec::new(),
        }
    }

    /// Constructor with explicit index arrays.
    pub fn with_indices(a_coord_index: Vec<i32>, a_color_index: Vec<i32>) -> Self {
        VrmlIndexedLineSet {
            my_coord_index: a_coord_index,
            my_color_index: a_color_index,
        }
    }

    pub fn set_coord_index(&mut self, a_coord_index: Vec<i32>) {
        self.my_coord_index = a_coord_index;
    }

    pub fn coord_index(&self) -> &[i32] {
        &self.my_coord_index
    }

    pub fn set_color_index(&mut self, a_color_index: Vec<i32>) {
        self.my_color_index = a_color_index;
    }

    pub fn color_index(&self) -> &[i32] {
        &self.my_color_index
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("IndexedLineSet {\n");

        // coordIndex field
        if !self.my_coord_index.is_empty() {
            an_ostream.push_str("    coordIndex\t[\n");
            for (i, idx) in self.my_coord_index.iter().enumerate() {
                if i > 0 {
                    if i % 8 == 0 {
                        // line break every 8 indices for readability
                        an_ostream.push('\n');
                    } else {
                        an_ostream.push(' ');
                    }
                }
                an_ostream.push_str(&idx.to_string());
            }
            an_ostream.push_str("\n    ]\n");
        }

        // colorIndex field
        if !self.my_color_index.is_empty() {
            an_ostream.push_str("    colorIndex\t[\n");
            for (i, idx) in self.my_color_index.iter().enumerate() {
                if i > 0 {
                    if i % 8 == 0 {
                        an_ostream.push('\n');
                    } else {
                        an_ostream.push(' ');
                    }
                }
                an_ostream.push_str(&idx.to_string());
            }
            an_ostream.push_str("\n    ]\n");
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlIndexedLineSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty_indices() {
        let ils = VrmlIndexedLineSet::new();
        assert_eq!(ils.coord_index().len(), 0);
        assert_eq!(ils.color_index().len(), 0);
    }

    #[test]
    fn default_prints_empty_node() {
        let ils = VrmlIndexedLineSet::new();
        let mut out = String::new();
        ils.print(&mut out);
        assert_eq!(out, "IndexedLineSet {\n}\n");
    }

    #[test]
    fn coord_index_only() {
        let ils = VrmlIndexedLineSet::with_indices(vec![0, 1, -1, 1, 2, -1], vec![]);
        let mut out = String::new();
        ils.print(&mut out);
        assert!(out.contains("coordIndex"));
        assert!(out.contains("0 1 -1 1 2 -1"));
    }

    #[test]
    fn with_color_index() {
        let ils = VrmlIndexedLineSet::with_indices(vec![0, 1, -1], vec![0, 1, -1]);
        let mut out = String::new();
        ils.print(&mut out);
        assert!(out.contains("coordIndex"));
        assert!(out.contains("colorIndex"));
    }

    #[test]
    fn large_index_array() {
        let mut indices = Vec::new();
        for i in 0..16 {
            indices.push(i);
        }
        indices.push(-1);
        let ils = VrmlIndexedLineSet::with_indices(indices, vec![]);
        let mut out = String::new();
        ils.print(&mut out);
        assert!(out.contains("coordIndex"));
        assert!(out.contains("0"));
        assert!(out.contains("15"));
    }

    #[test]
    fn setter() {
        let mut ils = VrmlIndexedLineSet::new();
        ils.set_coord_index(vec![0, 1, -1]);
        ils.set_color_index(vec![0, 1, -1]);
        assert_eq!(ils.coord_index(), &[0, 1, -1]);
        assert_eq!(ils.color_index(), &[0, 1, -1]);
    }
}
