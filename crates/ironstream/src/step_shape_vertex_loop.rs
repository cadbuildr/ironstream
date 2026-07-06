// FILE: step_shape_vertex_loop.rs
// occt: StepShape_VertexLoop

use std::sync::Arc;

/// Placeholder for StepShape_Vertex
pub struct Vertex {
    id: usize,
}

impl Vertex {
    pub fn new(id: usize) -> Self {
        Vertex { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Placeholder for StepShape_Loop base class
pub struct Loop {
    name: Arc<str>,
}

impl Loop {
    pub fn new(name: Arc<str>) -> Self {
        Loop { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Represents a vertex loop in STEP format.
/// Inherits from StepShape_Loop.
pub struct VertexLoop {
    name: Arc<str>,
    loop_vertex: Option<Arc<Vertex>>,
}

impl VertexLoop {
    /// Create a new VertexLoop
    pub fn new() -> Self {
        VertexLoop {
            name: Arc::from(""),
            loop_vertex: None,
        }
    }

    /// Initialize with name and loop vertex
    pub fn init(&mut self, name: Arc<str>, loop_vertex: Arc<Vertex>) {
        self.name = name;
        self.loop_vertex = Some(loop_vertex);
    }

    /// Set the loop vertex
    pub fn set_loop_vertex(&mut self, loop_vertex: Arc<Vertex>) {
        self.loop_vertex = Some(loop_vertex);
    }

    /// Get the loop vertex
    pub fn loop_vertex(&self) -> Option<&Arc<Vertex>> {
        self.loop_vertex.as_ref()
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for VertexLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_loop_creation() {
        let vl = VertexLoop::new();
        assert_eq!(vl.name(), "");
        assert!(vl.loop_vertex().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut vl = VertexLoop::new();
        let vertex = Arc::new(Vertex::new(1));
        let name: Arc<str> = Arc::from("vertex_loop_1");

        vl.init(name.clone(), vertex.clone());

        assert_eq!(vl.name(), "vertex_loop_1");
        assert!(vl.loop_vertex().is_some());
        assert_eq!(vl.loop_vertex().unwrap().id(), 1);
    }

    #[test]
    fn test_set_loop_vertex() {
        let mut vl = VertexLoop::new();
        let vertex = Arc::new(Vertex::new(42));

        vl.set_loop_vertex(vertex);

        assert!(vl.loop_vertex().is_some());
        assert_eq!(vl.loop_vertex().unwrap().id(), 42);
    }

    #[test]
    fn test_set_name() {
        let mut vl = VertexLoop::new();
        vl.set_name(Arc::from("my_loop"));

        assert_eq!(vl.name(), "my_loop");
    }

    #[test]
    fn test_full_initialization() {
        let mut vl = VertexLoop::new();
        let vertex = Arc::new(Vertex::new(10));
        let name: Arc<str> = Arc::from("full_loop");

        vl.init(name.clone(), vertex);

        assert_eq!(vl.name(), "full_loop");
        assert!(vl.loop_vertex().is_some());
        assert_eq!(vl.loop_vertex().unwrap().id(), 10);
    }
}
