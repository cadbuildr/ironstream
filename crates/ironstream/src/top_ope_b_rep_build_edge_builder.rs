// FILE: top_ope_b_rep_build_edge_builder.rs
// occt: TopOpeBRepBuild_EdgeBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildEdgeBuilder {
    vertices: Vec<i32>,
    parameters: Vec<f64>,
    current_edge_index: usize,
    current_vertex_index: usize,
}

impl TopOpeBRepBuildEdgeBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildEdgeBuilder {
            vertices: Vec::new(),
            parameters: Vec::new(),
            current_edge_index: 0,
            current_vertex_index: 0,
        }
    }

    pub fn add_vertex(&mut self, vertex_id: i32, parameter: f64) {
        self.vertices.push(vertex_id);
        self.parameters.push(parameter);
    }

    pub fn init_edge(&mut self) {
        self.current_edge_index = 0;
    }

    pub fn more_edge(&self) -> bool {
        self.current_edge_index < 1
    }

    pub fn next_edge(&mut self) {
        self.current_edge_index += 1;
    }

    pub fn init_vertex(&mut self) {
        self.current_vertex_index = 0;
    }

    pub fn more_vertex(&self) -> bool {
        self.current_vertex_index < self.vertices.len()
    }

    pub fn next_vertex(&mut self) {
        self.current_vertex_index += 1;
    }

    pub fn vertex(&self) -> Option<i32> {
        if self.current_vertex_index < self.vertices.len() {
            Some(self.vertices[self.current_vertex_index])
        } else {
            None
        }
    }

    pub fn parameter(&self) -> f64 {
        if self.current_vertex_index < self.parameters.len() {
            self.parameters[self.current_vertex_index]
        } else {
            0.0
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
}

impl Default for TopOpeBRepBuildEdgeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_builder_new() {
        let eb = TopOpeBRepBuildEdgeBuilder::new();
        assert_eq!(eb.num_vertices(), 0);
    }

    #[test]
    fn test_edge_builder_add_vertex() {
        let mut eb = TopOpeBRepBuildEdgeBuilder::new();
        eb.add_vertex(10, 0.5);
        eb.add_vertex(20, 0.8);
        assert_eq!(eb.num_vertices(), 2);
    }

    #[test]
    fn test_edge_builder_vertex_iteration() {
        let mut eb = TopOpeBRepBuildEdgeBuilder::new();
        eb.add_vertex(5, 0.3);
        eb.add_vertex(15, 0.7);

        eb.init_vertex();
        assert!(eb.more_vertex());
        assert_eq!(eb.vertex(), Some(5));
        assert_eq!(eb.parameter(), 0.3);

        eb.next_vertex();
        assert!(eb.more_vertex());
        assert_eq!(eb.vertex(), Some(15));
        assert_eq!(eb.parameter(), 0.7);

        eb.next_vertex();
        assert!(!eb.more_vertex());
    }

    #[test]
    fn test_edge_builder_edge_iteration() {
        let eb = TopOpeBRepBuildEdgeBuilder::new();
        assert!(eb.more_edge());
    }
}
