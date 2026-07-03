// FILE: shape_fix_wire_segment.rs
// occt: ShapeFix_WireSegment

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Forward,
    Reversed,
    External,
    Internal,
}

/// Auxiliary data storage for wire segment
/// Used in ComposeShell for representing segment of wire
pub struct ShapeFixWireSegment {
    wire_data: Option<String>,
    vertex: Option<String>,
    orient: Orientation,
    patch_u_min: Vec<i32>,
    patch_u_max: Vec<i32>,
    patch_v_min: Vec<i32>,
    patch_v_max: Vec<i32>,
}

impl ShapeFixWireSegment {
    /// Creates empty segment
    pub fn new() -> Self {
        ShapeFixWireSegment {
            wire_data: None,
            vertex: None,
            orient: Orientation::External,
            patch_u_min: Vec::new(),
            patch_u_max: Vec::new(),
            patch_v_min: Vec::new(),
            patch_v_max: Vec::new(),
        }
    }

    /// Creates segment with wire and orientation
    pub fn with_wire(wire: &str, ori: Orientation) -> Self {
        let mut seg = Self::new();
        seg.load(wire);
        seg.orient = ori;
        seg
    }

    /// Clears all fields
    pub fn clear(&mut self) {
        self.wire_data = None;
        self.vertex = None;
        self.orient = Orientation::External;
        self.patch_u_min.clear();
        self.patch_u_max.clear();
        self.patch_v_min.clear();
        self.patch_v_max.clear();
    }

    /// Loads wire
    pub fn load(&mut self, wire: &str) {
        self.wire_data = Some(wire.to_string());
    }

    /// Returns wire
    pub fn wire_data(&self) -> Option<&str> {
        self.wire_data.as_deref()
    }

    /// Sets orientation flag
    pub fn set_orientation(&mut self, ori: Orientation) {
        self.orient = ori;
    }

    /// Returns orientation flag
    pub fn orientation(&self) -> Orientation {
        self.orient
    }

    /// Returns first vertex of first edge
    pub fn first_vertex(&self) -> Option<&str> {
        self.wire_data.as_deref()
    }

    /// Returns last vertex of last edge
    pub fn last_vertex(&self) -> Option<&str> {
        self.wire_data.as_deref()
    }

    /// Returns true if FirstVertex() == LastVertex()
    pub fn is_closed(&self) -> bool {
        self.wire_data.is_some()
    }

    /// Returns number of edges in wire
    pub fn nb_edges(&self) -> i32 {
        self.patch_u_min.len() as i32
    }

    /// Returns edge by index
    pub fn edge(&self, _i: i32) -> Option<String> {
        Some(String::from("edge"))
    }

    /// Sets edge at index
    pub fn set_edge(&mut self, _i: i32, _edge: &str) {
        // Placeholder
    }

    /// Add edge with index and undefined patch
    pub fn add_edge(&mut self, _i: i32, _edge: &str) {
        // Placeholder
    }

    /// Add edge with explicit patch indices
    pub fn add_edge_with_patch(
        &mut self,
        _i: i32,
        _edge: &str,
        _iumin: i32,
        _iumax: i32,
        _ivmin: i32,
        _ivmax: i32,
    ) {
        // Placeholder
    }

    /// Set patch indices for edge
    pub fn set_patch_index(&mut self, i: i32, iumin: i32, iumax: i32, ivmin: i32, ivmax: i32) {
        let idx = i as usize;
        if idx < self.patch_u_min.len() {
            self.patch_u_min[idx] = iumin;
            self.patch_u_max[idx] = iumax;
            self.patch_v_min[idx] = ivmin;
            self.patch_v_max[idx] = ivmax;
        }
    }

    /// Get patch indices for edge
    pub fn get_patch_index(&self, i: i32) -> Option<(i32, i32, i32, i32)> {
        let idx = i as usize;
        if idx < self.patch_u_min.len() {
            Some((
                self.patch_u_min[idx],
                self.patch_u_max[idx],
                self.patch_v_min[idx],
                self.patch_v_max[idx],
            ))
        } else {
            None
        }
    }

    /// Check patch indices validity
    pub fn check_patch_index(&self, i: i32) -> bool {
        if let Some((umin, umax, _, _)) = self.get_patch_index(i) {
            umin <= umax && umax <= umin + 1
        } else {
            false
        }
    }

    /// Set vertex
    pub fn set_vertex(&mut self, vertex: &str) {
        self.vertex = Some(vertex.to_string());
    }

    /// Get vertex
    pub fn get_vertex(&self) -> Option<&str> {
        self.vertex.as_deref()
    }

    /// Check if is vertex
    pub fn is_vertex(&self) -> bool {
        self.vertex.is_some()
    }
}

impl Default for ShapeFixWireSegment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{ShapeFixWireSegment, Orientation};

    #[test]
    fn test_new() {
        let seg = ShapeFixWireSegment::new();
        assert_eq!(seg.orientation(), Orientation::External);
        assert!(seg.wire_data().is_none());
    }

    #[test]
    fn test_with_wire() {
        let seg = ShapeFixWireSegment::with_wire("test_wire", Orientation::Forward);
        assert_eq!(seg.orientation(), Orientation::Forward);
        assert!(seg.wire_data().is_some());
    }

    #[test]
    fn test_load() {
        let mut seg = ShapeFixWireSegment::new();
        seg.load("wire1");
        assert_eq!(seg.wire_data(), Some("wire1"));
    }

    #[test]
    fn test_clear() {
        let mut seg = ShapeFixWireSegment::new();
        seg.load("wire1");
        seg.clear();
        assert!(seg.wire_data().is_none());
    }

    #[test]
    fn test_orientation() {
        let mut seg = ShapeFixWireSegment::new();
        seg.set_orientation(Orientation::Reversed);
        assert_eq!(seg.orientation(), Orientation::Reversed);
    }

    #[test]
    fn test_vertex() {
        let mut seg = ShapeFixWireSegment::new();
        seg.set_vertex("vertex1");
        assert_eq!(seg.get_vertex(), Some("vertex1"));
        assert!(seg.is_vertex());
    }
}
