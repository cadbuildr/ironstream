// FILE: iges_solid_vertex_list.rs
// occt: IGESSolid_VertexList

/// Defines VertexList, Type <502> Form Number <1>
/// A vertex is a point in R3. A vertex is the bound of an
/// edge and can participate in the bounds of a face.
/// It contains one or more vertices.
#[derive(Clone, Debug)]
pub struct IgesSolidVertexList {
    vertices: Vec<[f64; 3]>,
}

impl IgesSolidVertexList {
    /// Creates a new VertexList
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    /// This method is used to set the fields of the class VertexList
    /// - vertices : the vertices in the list (must have at least one)
    pub fn init(&mut self, vertices: Vec<[f64; 3]>) -> Result<(), String> {
        if vertices.is_empty() {
            return Err("VertexList must contain at least one vertex".to_string());
        }
        self.vertices = vertices;
        Ok(())
    }

    /// Return the number of vertices in the list
    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the num'th vertex in the list (1-indexed)
    /// Returns None if num <= 0 or num > NbVertices()
    pub fn vertex(&self, num: usize) -> Option<[f64; 3]> {
        if num > 0 && num <= self.vertices.len() {
            Some(self.vertices[num - 1])
        } else {
            None
        }
    }

    /// Returns a reference to all vertices
    pub fn vertices_ref(&self) -> &[[f64; 3]] {
        &self.vertices
    }
}

impl Default for IgesSolidVertexList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let list = IgesSolidVertexList::new();
        assert_eq!(list.nb_vertices(), 0);
    }

    #[test]
    fn test_init_with_vertices() {
        let mut list = IgesSolidVertexList::new();
        let vertices = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        assert!(list.init(vertices).is_ok());
        assert_eq!(list.nb_vertices(), 3);
    }

    #[test]
    fn test_init_empty_fails() {
        let mut list = IgesSolidVertexList::new();
        let vertices: Vec<[f64; 3]> = vec![];
        assert!(list.init(vertices).is_err());
    }

    #[test]
    fn test_vertex_access() {
        let mut list = IgesSolidVertexList::new();
        let vertices = vec![
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        list.init(vertices).unwrap();

        assert_eq!(list.vertex(1), Some([1.0, 2.0, 3.0]));
        assert_eq!(list.vertex(2), Some([4.0, 5.0, 6.0]));
        assert_eq!(list.vertex(3), Some([7.0, 8.0, 9.0]));
    }

    #[test]
    fn test_vertex_out_of_bounds() {
        let mut list = IgesSolidVertexList::new();
        let vertices = vec![[1.0, 2.0, 3.0]];
        list.init(vertices).unwrap();

        assert_eq!(list.vertex(0), None);
        assert_eq!(list.vertex(2), None);
    }

    #[test]
    fn test_vertices_ref() {
        let mut list = IgesSolidVertexList::new();
        let vertices = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        list.init(vertices.clone()).unwrap();

        let ref_vertices = list.vertices_ref();
        assert_eq!(ref_vertices.len(), 2);
        assert_eq!(ref_vertices[0], [1.0, 2.0, 3.0]);
        assert_eq!(ref_vertices[1], [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_clone() {
        let mut list = IgesSolidVertexList::new();
        let vertices = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ];
        list.init(vertices).unwrap();

        let cloned = list.clone();
        assert_eq!(cloned.nb_vertices(), 2);
        assert_eq!(cloned.vertex(1), Some([0.0, 0.0, 0.0]));
    }

    #[test]
    fn test_single_vertex() {
        let mut list = IgesSolidVertexList::new();
        list.init(vec![[5.5, 6.5, 7.5]]).unwrap();
        assert_eq!(list.nb_vertices(), 1);
        assert_eq!(list.vertex(1), Some([5.5, 6.5, 7.5]));
    }

    #[test]
    fn test_many_vertices() {
        let mut list = IgesSolidVertexList::new();
        let mut vertices = Vec::new();
        for i in 0..100 {
            vertices.push([i as f64, (i + 1) as f64, (i + 2) as f64]);
        }
        list.init(vertices).unwrap();
        assert_eq!(list.nb_vertices(), 100);
        assert_eq!(list.vertex(50), Some([49.0, 50.0, 51.0]));
    }
}
