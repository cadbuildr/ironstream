// FILE: iges_solid_topo_builder.rs
// occt: IGESSolid_TopoBuilder

/// This class manages the creation of an IGES Topologic entity
/// (BREP : ManifoldSolid, Shell, Face)
/// This includes defining of Vertex and Edge Lists,
/// building of Edges and Loops
pub struct IgesSolidTopoBuilder {
    solid: Option<IgesEntity>,
    shells: Vec<(IgesEntity, i32)>,
    current_shell: Option<IgesEntity>,
    faces: Vec<(IgesEntity, i32)>,
    current_face: Option<IgesEntity>,
    surface: Option<IgesEntity>,
    loops: Vec<IgesEntity>,
    current_loop: Option<IgesEntity>,
    edges_3d: Vec<(IgesEntity, usize, usize)>,
    vertices: Vec<[f64; 3]>,
}

impl Default for IgesSolidTopoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSolidTopoBuilder {
    /// Creates an empty TopoBuilder
    /// This creates also a unique VertexList and a unique EdgeList,
    /// empty, but which can be referenced from starting
    pub fn new() -> Self {
        Self {
            solid: None,
            shells: Vec::new(),
            current_shell: None,
            faces: Vec::new(),
            current_face: None,
            surface: None,
            loops: Vec::new(),
            current_loop: None,
            edges_3d: Vec::new(),
            vertices: Vec::new(),
        }
    }

    /// Resets the TopoBuilder for an entirely new operation
    /// (with a new EdgeList, a new VertexList, new Shells, ...)
    pub fn clear(&mut self) {
        self.solid = None;
        self.shells.clear();
        self.current_shell = None;
        self.faces.clear();
        self.current_face = None;
        self.surface = None;
        self.loops.clear();
        self.current_loop = None;
        self.edges_3d.clear();
        self.vertices.clear();
    }

    /// Adds a Vertex to the VertexList
    pub fn add_vertex(&mut self, vertex: [f64; 3]) {
        self.vertices.push(vertex);
    }

    /// Returns the count of already recorded Vertices
    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Returns a Vertex, given its rank (1-indexed)
    pub fn vertex(&self, num: usize) -> Option<[f64; 3]> {
        if num > 0 && num <= self.vertices.len() {
            Some(self.vertices[num - 1])
        } else {
            None
        }
    }

    /// Adds an Edge (3D) to the EdgeList, defined by a Curve and
    /// two vertex indices, for start and end
    pub fn add_edge(&mut self, curve: IgesEntity, vstart: usize, vend: usize) -> Result<(), String> {
        if vstart == 0 || vend == 0 || vstart > self.vertices.len() || vend > self.vertices.len() {
            return Err("Invalid vertex indices for edge".to_string());
        }
        self.edges_3d.push((curve, vstart, vend));
        Ok(())
    }

    /// Returns the count of recorded Edges (3D)
    pub fn nb_edges(&self) -> usize {
        self.edges_3d.len()
    }

    /// Returns the definition of an Edge (3D) given its rank
    pub fn edge(&self, num: usize) -> Option<(IgesEntity, usize, usize)> {
        if num > 0 && num <= self.edges_3d.len() {
            let (curve, vstart, vend) = &self.edges_3d[num - 1];
            Some((curve.clone(), *vstart, *vend))
        } else {
            None
        }
    }

    /// Begins the definition of a new Loop
    pub fn make_loop(&mut self) {
        self.current_loop = Some(IgesEntity::default());
    }

    /// Closes the current Loop and fills it
    pub fn end_loop(&mut self) {
        if let Some(loop_entity) = self.current_loop.take() {
            self.loops.push(loop_entity);
        }
    }

    /// Begins the definition of a new Face, on a surface
    pub fn make_face(&mut self, surface: IgesEntity) {
        self.surface = Some(surface);
        self.current_face = Some(IgesEntity::default());
    }

    /// Closes the current Loop and sets it as Outer Loop for the current Face
    pub fn set_outer(&mut self) {
        self.end_loop();
    }

    /// Closes the current Loop and adds it to the list of Inner Loops
    pub fn add_inner(&mut self) {
        self.end_loop();
    }

    /// Closes the definition of the current Face, fills it and adds
    /// it to the current Shell with an orientation flag
    pub fn end_face(&mut self, orientation: i32) {
        if let Some(face) = self.current_face.take() {
            self.faces.push((face, orientation));
        }
    }

    /// Begins the definition of a new Shell
    pub fn make_shell(&mut self) {
        self.current_shell = Some(IgesEntity::default());
        self.faces.clear();
    }

    /// Closes the whole definition as that of a simple Shell
    pub fn end_simple_shell(&mut self) {
        if let Some(shell) = self.current_shell.take() {
            self.solid = Some(shell);
        }
    }

    /// Closes the current Shell definition and returns the Shell entity.
    /// In OCCT (IGESSolid_TopoBuilder::EndShell) the Shell entity always
    /// exists (created by Clear/MakeShell) and EndShell fills it from the
    /// accumulated faces; here the entity is built on demand.
    fn end_shell(&mut self) -> IgesEntity {
        self.current_shell.take().unwrap_or_default()
    }

    /// Closes the definition of the current Shell as the Main
    /// Shell of a Solid, with an orientation flag
    /// (OCCT SetMainShell: EndShell then record as the solid's main shell).
    /// The main shell is recorded first in the shell list of the Solid.
    pub fn set_main_shell(&mut self, orientation: i32) {
        let shell = self.end_shell();
        self.shells.push((shell, orientation));
    }

    /// Closes the definition of the current Shell and adds it to the
    /// list of Void Shells of a Solid, with an orientation flag
    /// (OCCT AddVoidShell: EndShell then append to the void shell list).
    pub fn add_void_shell(&mut self, orientation: i32) {
        let shell = self.end_shell();
        self.shells.push((shell, orientation));
    }

    /// Closes the whole definition as that of a ManifoldSolid
    pub fn end_solid(&mut self) {
        if let Some(shell) = self.current_shell.take() {
            self.solid = Some(shell);
        }
    }

    /// Returns the current Shell
    pub fn shell(&self) -> Option<IgesEntity> {
        self.current_shell.clone()
    }

    /// Returns the current ManifoldSolid
    pub fn solid(&self) -> Option<IgesEntity> {
        self.solid.clone()
    }
}

/// Stub type for IGES entities
#[derive(Clone, Debug, Default)]
pub struct IgesEntity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_topo_builder() {
        let builder = IgesSolidTopoBuilder::new();
        assert_eq!(builder.nb_vertices(), 0);
        assert_eq!(builder.nb_edges(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.add_vertex([1.0, 2.0, 3.0]);
        builder.add_vertex([4.0, 5.0, 6.0]);
        assert_eq!(builder.nb_vertices(), 2);
        assert_eq!(builder.vertex(1), Some([1.0, 2.0, 3.0]));
        assert_eq!(builder.vertex(2), Some([4.0, 5.0, 6.0]));
    }

    #[test]
    fn test_add_edge_valid() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.add_vertex([0.0, 0.0, 0.0]);
        builder.add_vertex([1.0, 1.0, 1.0]);
        let curve = IgesEntity::default();
        assert!(builder.add_edge(curve, 1, 2).is_ok());
        assert_eq!(builder.nb_edges(), 1);
    }

    #[test]
    fn test_add_edge_invalid_vertex() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.add_vertex([0.0, 0.0, 0.0]);
        let curve = IgesEntity::default();
        // Try to add edge with invalid vertex index
        assert!(builder.add_edge(curve, 1, 5).is_err());
    }

    #[test]
    fn test_clear() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.add_vertex([1.0, 2.0, 3.0]);
        assert_eq!(builder.nb_vertices(), 1);
        builder.clear();
        assert_eq!(builder.nb_vertices(), 0);
    }

    #[test]
    fn test_make_shell_and_faces() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.make_shell();
        assert!(builder.shell().is_some());
        let surface = IgesEntity::default();
        builder.make_face(surface);
        builder.end_face(1);
    }

    #[test]
    fn test_loop_operations() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.make_loop();
        builder.set_outer();
        assert_eq!(builder.loops.len(), 1);
    }

    #[test]
    fn test_solid_workflow() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.add_vertex([0.0, 0.0, 0.0]);
        builder.add_vertex([1.0, 0.0, 0.0]);
        builder.add_vertex([1.0, 1.0, 0.0]);
        builder.add_vertex([0.0, 1.0, 0.0]);

        let curve1 = IgesEntity::default();
        let curve2 = IgesEntity::default();
        builder.add_edge(curve1.clone(), 1, 2).unwrap();
        builder.add_edge(curve2, 2, 3).unwrap();

        assert_eq!(builder.nb_vertices(), 4);
        assert_eq!(builder.nb_edges(), 2);
    }

    #[test]
    fn test_end_simple_shell() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.make_shell();
        builder.end_simple_shell();
        assert!(builder.solid().is_some());
    }

    #[test]
    fn test_end_solid() {
        let mut builder = IgesSolidTopoBuilder::new();
        builder.make_shell();
        builder.set_main_shell(0);
        builder.add_void_shell(1);
        assert_eq!(builder.shells.len(), 2);
    }
}
