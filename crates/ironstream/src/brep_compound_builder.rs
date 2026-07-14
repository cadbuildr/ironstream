// FILE: brep_compound_builder.rs
// occt-note: BRepBuilderAPI_MakeCompound, BRep_Builder (compound operations)

// occt-ref: BRepBuilderAPI_MakeCompound
/// Assembles multiple shapes into a compound (unordered collection).
#[derive(Clone, Debug, Default)]
pub struct MakeCompound {
    pub shapes: Vec<CompoundShape>,
}

#[derive(Clone, Debug)]
pub struct CompoundShape {
    pub id: usize,
    pub shape_type: CompoundShapeType,
    pub bounding_box: Option<[[f64; 3]; 2]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompoundShapeType {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    Compound,
}

impl CompoundShape {
    pub fn new(id: usize, shape_type: CompoundShapeType) -> Self {
        Self { id, shape_type, bounding_box: None }
    }

    pub fn with_bbox(mut self, min: [f64; 3], max: [f64; 3]) -> Self {
        self.bounding_box = Some([min, max]);
        self
    }
}

impl MakeCompound {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, shape: CompoundShape) {
        self.shapes.push(shape);
    }

    pub fn remove(&mut self, id: usize) {
        self.shapes.retain(|s| s.id != id);
    }

    pub fn is_done(&self) -> bool { true }
    pub fn nb_shapes(&self) -> usize { self.shapes.len() }

    pub fn shape_type_count(&self, t: CompoundShapeType) -> usize {
        self.shapes.iter().filter(|s| s.shape_type == t).count()
    }

    /// Combined bounding box of all shapes with bbox data.
    pub fn global_bbox(&self) -> Option<[[f64; 3]; 2]> {
        let boxes: Vec<[[f64; 3]; 2]> = self.shapes.iter()
            .filter_map(|s| s.bounding_box)
            .collect();
        if boxes.is_empty() { return None; }
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        for b in &boxes {
            for i in 0..3 {
                min[i] = min[i].min(b[0][i]);
                max[i] = max[i].max(b[1][i]);
            }
        }
        Some([min, max])
    }
}

// occt-ref: BRep_Builder // compound ops
pub struct CompoundOps;

impl CompoundOps {
    /// Flatten nested compounds into a single-level list of leaf shapes.
    pub fn flatten(shapes: &[CompoundShape]) -> Vec<&CompoundShape> {
        shapes.iter().filter(|s| s.shape_type != CompoundShapeType::Compound).collect()
    }

    /// Check if all shapes in the compound are of the same type.
    pub fn is_homogeneous(shapes: &[CompoundShape]) -> bool {
        if shapes.is_empty() { return true; }
        let first = shapes[0].shape_type;
        shapes.iter().all(|s| s.shape_type == first)
    }

    /// Count shapes by type.
    pub fn count_by_type(shapes: &[CompoundShape]) -> [(CompoundShapeType, usize); 7] {
        use CompoundShapeType::*;
        let types = [Vertex, Edge, Wire, Face, Shell, Solid, Compound];
        types.map(|t| (t, shapes.iter().filter(|s| s.shape_type == t).count()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_compound_add_remove() {
        let mut c = MakeCompound::new();
        c.add(CompoundShape::new(0, CompoundShapeType::Solid));
        c.add(CompoundShape::new(1, CompoundShapeType::Face));
        assert_eq!(c.nb_shapes(), 2);
        c.remove(0);
        assert_eq!(c.nb_shapes(), 1);
    }

    #[test]
    fn make_compound_type_count() {
        let mut c = MakeCompound::new();
        c.add(CompoundShape::new(0, CompoundShapeType::Solid));
        c.add(CompoundShape::new(1, CompoundShapeType::Solid));
        c.add(CompoundShape::new(2, CompoundShapeType::Face));
        assert_eq!(c.shape_type_count(CompoundShapeType::Solid), 2);
        assert_eq!(c.shape_type_count(CompoundShapeType::Face), 1);
    }

    #[test]
    fn compound_global_bbox() {
        let mut c = MakeCompound::new();
        c.add(CompoundShape::new(0, CompoundShapeType::Solid)
            .with_bbox([0.0,0.0,0.0], [1.0,1.0,1.0]));
        c.add(CompoundShape::new(1, CompoundShapeType::Solid)
            .with_bbox([2.0,0.0,0.0], [3.0,1.0,1.0]));
        let bbox = c.global_bbox().unwrap();
        assert!((bbox[0][0]).abs() < 1e-10);
        assert!((bbox[1][0] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn compound_flatten() {
        let shapes = vec![
            CompoundShape::new(0, CompoundShapeType::Compound),
            CompoundShape::new(1, CompoundShapeType::Solid),
            CompoundShape::new(2, CompoundShapeType::Face),
        ];
        let flat = CompoundOps::flatten(&shapes);
        assert_eq!(flat.len(), 2);
    }

    #[test]
    fn compound_homogeneous() {
        let shapes = vec![
            CompoundShape::new(0, CompoundShapeType::Face),
            CompoundShape::new(1, CompoundShapeType::Face),
        ];
        assert!(CompoundOps::is_homogeneous(&shapes));
        let mixed = vec![
            CompoundShape::new(0, CompoundShapeType::Face),
            CompoundShape::new(1, CompoundShapeType::Solid),
        ];
        assert!(!CompoundOps::is_homogeneous(&mixed));
    }
}
