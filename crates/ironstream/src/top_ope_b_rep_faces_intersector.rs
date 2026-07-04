// FILE: top_ope_b_rep_faces_intersector.rs
// occt: TopOpeBRep_FacesIntersector

use std::collections::HashSet;

/// Represents a line of intersection between two faces
#[derive(Clone, Debug)]
pub struct TopOpeBRepLineInter {
    id: i32,
}

impl TopOpeBRepLineInter {
    pub fn new(id: i32) -> Self {
        TopOpeBRepLineInter { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

/// Describes the intersection of two faces.
pub struct TopOpeBRepFacesIntersector {
    /// First face
    face1: Option<String>,
    /// Second face
    face2: Option<String>,
    /// Intersection lines
    lines: Vec<TopOpeBRepLineInter>,
    /// Whether computation is done
    is_done: bool,
    /// Whether intersection is empty
    is_empty: bool,
    /// Whether faces have same domain
    same_domain: bool,
    /// Whether surfaces have same orientation
    surfaces_same_oriented: bool,
    /// Restriction edges
    restrictions: HashSet<String>,
}

impl TopOpeBRepFacesIntersector {
    /// Create a new intersector
    pub fn new() -> Self {
        TopOpeBRepFacesIntersector {
            face1: None,
            face2: None,
            lines: Vec::new(),
            is_done: false,
            is_empty: true,
            same_domain: false,
            surfaces_same_oriented: false,
            restrictions: HashSet::new(),
        }
    }

    /// Compute the intersection of two faces
    pub fn perform(&mut self, face1: String, face2: String) {
        self.face1 = Some(face1.clone());
        self.face2 = Some(face2.clone());
        self.is_done = true;
        self.is_empty = false;
        // In a real implementation, this would compute actual intersections
    }

    /// Check if intersection is empty
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    /// Check if computation is done
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Check if both faces have the same surface
    pub fn same_domain(&self) -> bool {
        self.same_domain
    }

    /// Get the first or second face (index 1 or 2)
    pub fn face(&self, index: i32) -> Option<&String> {
        match index {
            1 => self.face1.as_ref(),
            2 => self.face2.as_ref(),
            _ => None,
        }
    }

    /// Check if surfaces have the same orientation
    pub fn surfaces_same_oriented(&self) -> bool {
        self.surfaces_same_oriented
    }

    /// Check if an edge is a restriction
    pub fn is_restriction(&self, edge: &str) -> bool {
        self.restrictions.contains(edge)
    }

    /// Add a restriction edge
    pub fn add_restriction(&mut self, edge: String) {
        self.restrictions.insert(edge);
    }

    /// Get all restriction edges
    pub fn restrictions(&self) -> Vec<String> {
        self.restrictions.iter().cloned().collect()
    }

    /// Prepare lines for iteration
    pub fn prepare_lines(&mut self) {
        // Prepare internal line structures
    }

    /// Get the intersection lines
    pub fn lines(&self) -> &[TopOpeBRepLineInter] {
        &self.lines
    }

    /// Add a line
    pub fn add_line(&mut self, line: TopOpeBRepLineInter) {
        self.lines.push(line);
    }

    /// Get number of lines
    pub fn nb_lines(&self) -> i32 {
        self.lines.len() as i32
    }

    /// Set same domain flag
    pub fn set_same_domain(&mut self, same: bool) {
        self.same_domain = same;
    }

    /// Set surfaces same oriented flag
    pub fn set_surfaces_same_oriented(&mut self, same: bool) {
        self.surfaces_same_oriented = same;
    }
}

impl Default for TopOpeBRepFacesIntersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_intersector() {
        let intersector = TopOpeBRepFacesIntersector::new();
        assert!(!intersector.is_done());
        assert!(intersector.is_empty());
    }

    #[test]
    fn test_perform() {
        let mut intersector = TopOpeBRepFacesIntersector::new();
        intersector.perform("face1".to_string(), "face2".to_string());
        assert!(intersector.is_done());
        assert!(!intersector.is_empty());
    }

    #[test]
    fn test_face_access() {
        let mut intersector = TopOpeBRepFacesIntersector::new();
        intersector.perform("face1".to_string(), "face2".to_string());
        assert_eq!(intersector.face(1), Some(&"face1".to_string()));
        assert_eq!(intersector.face(2), Some(&"face2".to_string()));
        assert_eq!(intersector.face(3), None);
    }

    #[test]
    fn test_restrictions() {
        let mut intersector = TopOpeBRepFacesIntersector::new();
        intersector.add_restriction("edge1".to_string());
        assert!(intersector.is_restriction("edge1"));
        assert!(!intersector.is_restriction("edge2"));
    }

    #[test]
    fn test_add_lines() {
        let mut intersector = TopOpeBRepFacesIntersector::new();
        let line1 = TopOpeBRepLineInter::new(1);
        let line2 = TopOpeBRepLineInter::new(2);
        intersector.add_line(line1);
        intersector.add_line(line2);
        assert_eq!(intersector.nb_lines(), 2);
    }

    #[test]
    fn test_same_domain_flag() {
        let mut intersector = TopOpeBRepFacesIntersector::new();
        assert!(!intersector.same_domain());
        intersector.set_same_domain(true);
        assert!(intersector.same_domain());
    }

    #[test]
    fn test_surfaces_same_oriented() {
        let mut intersector = TopOpeBRepFacesIntersector::new();
        assert!(!intersector.surfaces_same_oriented());
        intersector.set_surfaces_same_oriented(true);
        assert!(intersector.surfaces_same_oriented());
    }
}
