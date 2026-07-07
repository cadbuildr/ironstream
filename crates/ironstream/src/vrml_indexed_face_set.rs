// FILE: vrml_indexed_face_set.rs
// occt: Vrml_IndexedFaceSet
//
// Faithful port of OCCT Vrml_IndexedFaceSet (DataExchange/TKDEVRML/Vrml/
// Vrml_IndexedFaceSet.hxx): the VRML 1.0 `IndexedFaceSet` geometry node,
// storing indexed face data with vertex, normal, and color indices.
// Uses positive indices for vertex/normal/color data and negative/zero terminators.
// Default has empty index arrays. Print emits field arrays in VRML syntax.

/// Port of Vrml_IndexedFaceSet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlIndexedFaceSet {
    my_coord_index: Vec<i32>,
    my_normal_index: Vec<i32>,
    my_color_index: Vec<i32>,
}

impl VrmlIndexedFaceSet {
    /// Vrml_IndexedFaceSet(): all index arrays empty.
    pub fn new() -> Self {
        VrmlIndexedFaceSet {
            my_coord_index: Vec::new(),
            my_normal_index: Vec::new(),
            my_color_index: Vec::new(),
        }
    }

    /// Constructor with explicit index arrays.
    pub fn with_indices(
        a_coord_index: Vec<i32>,
        a_normal_index: Vec<i32>,
        a_color_index: Vec<i32>,
    ) -> Self {
        VrmlIndexedFaceSet {
            my_coord_index: a_coord_index,
            my_normal_index: a_normal_index,
            my_color_index: a_color_index,
        }
    }

    pub fn set_coord_index(&mut self, a_coord_index: Vec<i32>) {
        self.my_coord_index = a_coord_index;
    }

    pub fn coord_index(&self) -> &[i32] {
        &self.my_coord_index
    }

    pub fn set_normal_index(&mut self, a_normal_index: Vec<i32>) {
        self.my_normal_index = a_normal_index;
    }

    pub fn normal_index(&self) -> &[i32] {
        &self.my_normal_index
    }

    pub fn set_color_index(&mut self, a_color_index: Vec<i32>) {
        self.my_color_index = a_color_index;
    }

    pub fn color_index(&self) -> &[i32] {
        &self.my_color_index
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("IndexedFaceSet {\n");

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

        // normalIndex field
        if !self.my_normal_index.is_empty() {
            an_ostream.push_str("    normalIndex\t[\n");
            for (i, idx) in self.my_normal_index.iter().enumerate() {
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

impl Default for VrmlIndexedFaceSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty_indices() {
        let ifs = VrmlIndexedFaceSet::new();
        assert_eq!(ifs.coord_index().len(), 0);
        assert_eq!(ifs.normal_index().len(), 0);
        assert_eq!(ifs.color_index().len(), 0);
    }

    #[test]
    fn default_prints_empty_node() {
        let ifs = VrmlIndexedFaceSet::new();
        let mut out = String::new();
        ifs.print(&mut out);
        assert_eq!(out, "IndexedFaceSet {\n}\n");
    }

    #[test]
    fn coord_index_only() {
        let ifs = VrmlIndexedFaceSet::with_indices(vec![0, 1, 2, -1], vec![], vec![]);
        let mut out = String::new();
        ifs.print(&mut out);
        assert!(out.contains("coordIndex"));
        assert!(out.contains("0 1 2 -1"));
    }

    #[test]
    fn all_indices() {
        let ifs = VrmlIndexedFaceSet::with_indices(
            vec![0, 1, 2, -1],
            vec![0, 1, 2, -1],
            vec![0, 0, 0, -1],
        );
        let mut out = String::new();
        ifs.print(&mut out);
        assert!(out.contains("coordIndex"));
        assert!(out.contains("normalIndex"));
        assert!(out.contains("colorIndex"));
    }

    #[test]
    fn large_index_array() {
        let mut indices = Vec::new();
        for i in 0..20 {
            indices.push(i);
        }
        indices.push(-1);
        let ifs = VrmlIndexedFaceSet::with_indices(indices, vec![], vec![]);
        let mut out = String::new();
        ifs.print(&mut out);
        assert!(out.contains("coordIndex"));
        assert!(out.contains("0"));
        assert!(out.contains("19"));
    }

    #[test]
    fn setter() {
        let mut ifs = VrmlIndexedFaceSet::new();
        ifs.set_coord_index(vec![0, 1, 2, -1]);
        ifs.set_normal_index(vec![0, 1, 2, -1]);
        assert_eq!(ifs.coord_index(), &[0, 1, 2, -1]);
        assert_eq!(ifs.normal_index(), &[0, 1, 2, -1]);
    }
}
