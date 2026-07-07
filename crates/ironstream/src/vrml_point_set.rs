// FILE: vrml_point_set.rs
// occt: Vrml_PointSet
//
// Faithful port of OCCT Vrml_PointSet (DataExchange/TKDEVRML/Vrml/
// Vrml_PointSet.hxx): the VRML 1.0 `PointSet` node, a simple geometry
// with a start index and count of points (uses Coordinate node for actual coordinates).
// Default startIndex is 0, default numPoints is -1 (all points).
// Print emits non-default fields.

/// Port of Vrml_PointSet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlPointSet {
    my_start_index: i32,
    my_num_points: i32,
}

impl VrmlPointSet {
    /// Vrml_PointSet(): startIndex=0, numPoints=-1 (all).
    pub fn new() -> Self {
        VrmlPointSet {
            my_start_index: 0,
            my_num_points: -1,
        }
    }

    /// Vrml_PointSet(const Standard_Integer aStartIndex, const Standard_Integer aNumPoints).
    pub fn with_indices(a_start_index: i32, a_num_points: i32) -> Self {
        VrmlPointSet {
            my_start_index: a_start_index,
            my_num_points: a_num_points,
        }
    }

    pub fn set_start_index(&mut self, a_start_index: i32) {
        self.my_start_index = a_start_index;
    }

    pub fn start_index(&self) -> i32 {
        self.my_start_index
    }

    pub fn set_num_points(&mut self, a_num_points: i32) {
        self.my_num_points = a_num_points;
    }

    pub fn num_points(&self) -> i32 {
        self.my_num_points
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("PointSet {\n");

        // startIndex (default 0)
        if self.my_start_index != 0 {
            an_ostream.push_str("    startIndex\t");
            an_ostream.push_str(&self.my_start_index.to_string());
            an_ostream.push('\n');
        }

        // numPoints (default -1)
        if self.my_num_points != -1 {
            an_ostream.push_str("    numPoints\t");
            an_ostream.push_str(&self.my_num_points.to_string());
            an_ostream.push('\n');
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlPointSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_point_set() {
        let ps = VrmlPointSet::new();
        assert_eq!(ps.start_index(), 0);
        assert_eq!(ps.num_points(), -1);
    }

    #[test]
    fn default_prints_empty_node() {
        let ps = VrmlPointSet::new();
        let mut out = String::new();
        ps.print(&mut out);
        assert_eq!(out, "PointSet {\n}\n");
    }

    #[test]
    fn custom_start_index() {
        let ps = VrmlPointSet::with_indices(5, -1);
        let mut out = String::new();
        ps.print(&mut out);
        assert_eq!(out, "PointSet {\n    startIndex\t5\n}\n");
    }

    #[test]
    fn custom_num_points() {
        let ps = VrmlPointSet::with_indices(0, 10);
        let mut out = String::new();
        ps.print(&mut out);
        assert_eq!(out, "PointSet {\n    numPoints\t10\n}\n");
    }

    #[test]
    fn both_custom() {
        let ps = VrmlPointSet::with_indices(5, 10);
        let mut out = String::new();
        ps.print(&mut out);
        assert!(out.contains("startIndex\t5"));
        assert!(out.contains("numPoints\t10"));
    }

    #[test]
    fn setters() {
        let mut ps = VrmlPointSet::new();
        ps.set_start_index(3);
        ps.set_num_points(7);
        assert_eq!(ps.start_index(), 3);
        assert_eq!(ps.num_points(), 7);
    }
}
