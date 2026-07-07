// FILE: b_rep_builder_api_cell_filter.rs
// occt: BRepBuilderAPI_CellFilter

use std::collections::HashMap;

/// Vertex inspector for spatial queries.
#[derive(Debug, Clone)]
struct VertexInspector {
    vertex_id: usize,
    x: f64,
    y: f64,
    z: f64,
}

impl VertexInspector {
    fn new(vertex_id: usize, x: f64, y: f64, z: f64) -> Self {
        VertexInspector {
            vertex_id,
            x,
            y,
            z,
        }
    }

    fn id(&self) -> usize {
        self.vertex_id
    }

    fn coords(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

/// Cell filter for spatial binning of vertices.
pub struct BrepBuilderApiCellFilter {
    cells: HashMap<(i32, i32, i32), Vec<VertexInspector>>,
    cell_size: f64,
}

impl BrepBuilderApiCellFilter {
    /// Creates a new cell filter with the given cell size.
    pub fn new(cell_size: f64) -> Self {
        BrepBuilderApiCellFilter {
            cells: HashMap::new(),
            cell_size,
        }
    }

    /// Adds a vertex to the filter.
    pub fn add_vertex(&mut self, vertex: VertexInspector) {
        let cell_key = self.get_cell_key(vertex.x, vertex.y, vertex.z);
        self.cells.entry(cell_key).or_insert_with(Vec::new).push(vertex);
    }

    /// Finds all vertices within a given distance from a point.
    pub fn find_vertices_near(&self, x: f64, y: f64, z: f64, tolerance: f64) -> Vec<usize> {
        let mut result = Vec::new();
        let cell_key = self.get_cell_key(x, y, z);

        // Check nearby cells
        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    let near_key = (
                        cell_key.0 + di,
                        cell_key.1 + dj,
                        cell_key.2 + dk,
                    );
                    if let Some(vertices) = self.cells.get(&near_key) {
                        for v in vertices {
                            let dx = v.x - x;
                            let dy = v.y - y;
                            let dz = v.z - z;
                            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                            if dist <= tolerance {
                                result.push(v.vertex_id);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    /// Clears all cells.
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Returns the number of cells.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Returns the total number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.cells.values().map(|v| v.len()).sum()
    }

    fn get_cell_key(&self, x: f64, y: f64, z: f64) -> (i32, i32, i32) {
        (
            (x / self.cell_size).floor() as i32,
            (y / self.cell_size).floor() as i32,
            (z / self.cell_size).floor() as i32,
        )
    }
}

impl Default for BrepBuilderApiCellFilter {
    fn default() -> Self {
        Self::new(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let v = VertexInspector::new(1, 0.5, 1.5, 2.5);
        assert_eq!(v.id(), 1);
        assert_eq!(v.coords(), (0.5, 1.5, 2.5));
    }

    #[test]
    fn test_filter_creation() {
        let filter = BrepBuilderApiCellFilter::new(1.0);
        assert_eq!(filter.cell_count(), 0);
        assert_eq!(filter.vertex_count(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut filter = BrepBuilderApiCellFilter::new(10.0);
        let v = VertexInspector::new(1, 5.0, 5.0, 5.0);
        filter.add_vertex(v);
        assert_eq!(filter.vertex_count(), 1);
    }

    #[test]
    fn test_find_nearby() {
        let mut filter = BrepBuilderApiCellFilter::new(10.0);
        let v1 = VertexInspector::new(1, 0.0, 0.0, 0.0);
        let v2 = VertexInspector::new(2, 0.5, 0.5, 0.5);
        let v3 = VertexInspector::new(3, 10.0, 10.0, 10.0);
        filter.add_vertex(v1);
        filter.add_vertex(v2);
        filter.add_vertex(v3);

        let nearby = filter.find_vertices_near(0.2, 0.2, 0.2, 1.0);
        assert!(nearby.contains(&1));
        assert!(nearby.contains(&2));
        assert!(!nearby.contains(&3));
    }

    #[test]
    fn test_multiple_vertices_same_cell() {
        let mut filter = BrepBuilderApiCellFilter::new(10.0);
        for i in 1..=5 {
            let v = VertexInspector::new(i, 5.0 + i as f64 * 0.1, 5.0, 5.0);
            filter.add_vertex(v);
        }
        assert_eq!(filter.vertex_count(), 5);
    }

    #[test]
    fn test_vertices_different_cells() {
        let mut filter = BrepBuilderApiCellFilter::new(10.0);
        let v1 = VertexInspector::new(1, 5.0, 5.0, 5.0);
        let v2 = VertexInspector::new(2, 25.0, 25.0, 25.0);
        filter.add_vertex(v1);
        filter.add_vertex(v2);
        assert_eq!(filter.cell_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut filter = BrepBuilderApiCellFilter::new(1.0);
        filter.add_vertex(VertexInspector::new(1, 0.0, 0.0, 0.0));
        filter.add_vertex(VertexInspector::new(2, 5.0, 5.0, 5.0));
        assert_eq!(filter.vertex_count(), 2);

        filter.clear();
        assert_eq!(filter.vertex_count(), 0);
        assert_eq!(filter.cell_count(), 0);
    }

    #[test]
    fn test_tolerance() {
        let mut filter = BrepBuilderApiCellFilter::new(5.0);
        let v1 = VertexInspector::new(1, 0.0, 0.0, 0.0);
        let v2 = VertexInspector::new(2, 1.0, 0.0, 0.0);
        filter.add_vertex(v1);
        filter.add_vertex(v2);

        let nearby_tight = filter.find_vertices_near(0.0, 0.0, 0.0, 0.5);
        assert_eq!(nearby_tight.len(), 1);

        let nearby_loose = filter.find_vertices_near(0.0, 0.0, 0.0, 2.0);
        assert_eq!(nearby_loose.len(), 2);
    }
}
