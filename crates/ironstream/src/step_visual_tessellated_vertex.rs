// FILE: step_visual_tessellated_vertex.rs
// occt: StepVisual_TessellatedVertex

/// Represents a STEP TessellatedVertex entity.
pub struct TessellatedVertex {
    name: String,
    coordinates: Option<CoordinatesList>,
    topological_link: Option<VertexPoint>,
    point_index: i32,
    has_topological_link: bool,
}

/// Placeholder for CoordinatesList
pub struct CoordinatesList;

/// Placeholder for VertexPoint
pub struct VertexPoint;

impl TessellatedVertex {
    /// Creates a new tessellated vertex.
    pub fn new() -> Self {
        TessellatedVertex {
            name: String::new(),
            coordinates: None,
            topological_link: None,
            point_index: 0,
            has_topological_link: false,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        name: String,
        coordinates: Option<CoordinatesList>,
        has_topological_link: bool,
        topological_link: Option<VertexPoint>,
        point_index: i32,
    ) {
        self.name = name;
        self.coordinates = coordinates;
        self.has_topological_link = has_topological_link;
        self.topological_link = topological_link;
        self.point_index = point_index;
    }

    /// Returns the coordinates.
    pub fn coordinates(&self) -> Option<&CoordinatesList> {
        self.coordinates.as_ref()
    }

    /// Sets the coordinates.
    pub fn set_coordinates(&mut self, coordinates: CoordinatesList) {
        self.coordinates = Some(coordinates);
    }

    /// Returns the topological link.
    pub fn topological_link(&self) -> Option<&VertexPoint> {
        self.topological_link.as_ref()
    }

    /// Sets the topological link.
    pub fn set_topological_link(&mut self, link: VertexPoint) {
        self.topological_link = Some(link);
        self.has_topological_link = true;
    }

    /// Returns true if topological link is defined.
    pub fn has_topological_link(&self) -> bool {
        self.has_topological_link
    }

    /// Returns the point index.
    pub fn point_index(&self) -> i32 {
        self.point_index
    }

    /// Sets the point index.
    pub fn set_point_index(&mut self, idx: i32) {
        self.point_index = idx;
    }
}

impl Default for TessellatedVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tv = TessellatedVertex::new();
        assert_eq!(tv.point_index(), 0);
        assert!(!tv.has_topological_link());
    }

    #[test]
    fn test_point_index() {
        let mut tv = TessellatedVertex::new();
        tv.set_point_index(5);
        assert_eq!(tv.point_index(), 5);
    }

    #[test]
    fn test_topological_link() {
        let mut tv = TessellatedVertex::new();
        let vp = VertexPoint;
        tv.set_topological_link(vp);
        assert!(tv.has_topological_link());
        assert!(tv.topological_link().is_some());
    }
}
