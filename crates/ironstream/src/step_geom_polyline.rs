// FILE: step_geom_polyline.rs
// occt: StepGeom_Polyline

/// Represents a polyline (multiple connected line segments)
pub struct StepGeomPolyline {
    name: String,
    /// Vertex IDs defining the polyline
    vertices: Vec<i32>,
}

impl StepGeomPolyline {
    pub fn new(name: String) -> Self {
        StepGeomPolyline {
            name,
            vertices: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_vertex(&mut self, vertex_id: i32) {
        self.vertices.push(vertex_id);
    }

    pub fn vertices(&self) -> &[i32] {
        &self.vertices
    }

    pub fn nb_vertices(&self) -> i32 {
        self.vertices.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_polyline() {
        let polyline = StepGeomPolyline::new("Polyline1".to_string());
        assert_eq!(polyline.name(), "Polyline1");
        assert_eq!(polyline.nb_vertices(), 0);
    }

    #[test]
    fn test_add_vertices() {
        let mut polyline = StepGeomPolyline::new("Polyline1".to_string());
        polyline.add_vertex(1);
        polyline.add_vertex(2);
        polyline.add_vertex(3);
        assert_eq!(polyline.nb_vertices(), 3);
    }

    #[test]
    fn test_vertices() {
        let mut polyline = StepGeomPolyline::new("Polyline1".to_string());
        polyline.add_vertex(10);
        polyline.add_vertex(20);
        let verts = polyline.vertices();
        assert_eq!(verts[0], 10);
        assert_eq!(verts[1], 20);
    }
}
