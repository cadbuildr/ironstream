// FILE: de_shape.rs

// occt-ref: TopAbs_ShapeEnum // (mirror as DE_Shape type)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeShapeType {
    Vertex = 0,
    Edge = 1,
    Wire = 2,
    Face = 3,
    Shell = 4,
    Solid = 5,
    CompSolid = 6,
    Compound = 7,
    Shape = 8,
}

impl DeShapeType {
    pub fn rank(&self) -> u8 {
        *self as u8
    }

    pub fn name(&self) -> &'static str {
        match self {
            DeShapeType::Vertex => "VERTEX",
            DeShapeType::Edge => "EDGE",
            DeShapeType::Wire => "WIRE",
            DeShapeType::Face => "FACE",
            DeShapeType::Shell => "SHELL",
            DeShapeType::Solid => "SOLID",
            DeShapeType::CompSolid => "COMPSOLID",
            DeShapeType::Compound => "COMPOUND",
            DeShapeType::Shape => "SHAPE",
        }
    }
}

// occt-note: statistics for DataExchange
pub struct DeShapeStats {
    pub nb_vertices: u32,
    pub nb_edges: u32,
    pub nb_wires: u32,
    pub nb_faces: u32,
    pub nb_shells: u32,
    pub nb_solids: u32,
}

impl DeShapeStats {
    pub fn new() -> Self {
        DeShapeStats {
            nb_vertices: 0,
            nb_edges: 0,
            nb_wires: 0,
            nb_faces: 0,
            nb_shells: 0,
            nb_solids: 0,
        }
    }

    pub fn total(&self) -> u32 {
        self.nb_vertices
            + self.nb_edges
            + self.nb_wires
            + self.nb_faces
            + self.nb_shells
            + self.nb_solids
    }

    pub fn add(&mut self, other: &DeShapeStats) {
        self.nb_vertices += other.nb_vertices;
        self.nb_edges += other.nb_edges;
        self.nb_wires += other.nb_wires;
        self.nb_faces += other.nb_faces;
        self.nb_shells += other.nb_shells;
        self.nb_solids += other.nb_solids;
    }
}

impl Default for DeShapeStats {
    fn default() -> Self {
        Self::new()
    }
}

// occt-note: BRep bounding box
pub struct DeShapeBoundingBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub is_void: bool,
}

impl DeShapeBoundingBox {
    pub fn new_void() -> Self {
        DeShapeBoundingBox {
            xmin: 0.0,
            xmax: 0.0,
            ymin: 0.0,
            ymax: 0.0,
            zmin: 0.0,
            zmax: 0.0,
            is_void: true,
        }
    }

    pub fn from_min_max(
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        zmin: f64,
        zmax: f64,
    ) -> Self {
        DeShapeBoundingBox {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
            is_void: false,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64, z: f64) {
        if self.is_void {
            self.xmin = x;
            self.xmax = x;
            self.ymin = y;
            self.ymax = y;
            self.zmin = z;
            self.zmax = z;
            self.is_void = false;
        } else {
            if x < self.xmin {
                self.xmin = x;
            }
            if x > self.xmax {
                self.xmax = x;
            }
            if y < self.ymin {
                self.ymin = y;
            }
            if y > self.ymax {
                self.ymax = y;
            }
            if z < self.zmin {
                self.zmin = z;
            }
            if z > self.zmax {
                self.zmax = z;
            }
        }
    }

    pub fn diagonal(&self) -> f64 {
        if self.is_void {
            return 0.0;
        }
        let dx = self.xmax - self.xmin;
        let dy = self.ymax - self.ymin;
        let dz = self.zmax - self.zmin;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn center(&self) -> [f64; 3] {
        [
            0.5 * (self.xmin + self.xmax),
            0.5 * (self.ymin + self.ymax),
            0.5 * (self.zmin + self.zmax),
        ]
    }

    pub fn volume(&self) -> f64 {
        if self.is_void {
            return 0.0;
        }
        (self.xmax - self.xmin) * (self.ymax - self.ymin) * (self.zmax - self.zmin)
    }

    pub fn contains_point(&self, x: f64, y: f64, z: f64) -> bool {
        if self.is_void {
            return false;
        }
        x >= self.xmin
            && x <= self.xmax
            && y >= self.ymin
            && y <= self.ymax
            && z >= self.zmin
            && z <= self.zmax
    }

    pub fn is_void(&self) -> bool {
        self.is_void
    }

    pub fn merge(&mut self, other: &DeShapeBoundingBox) {
        if other.is_void {
            return;
        }
        if self.is_void {
            *self = DeShapeBoundingBox {
                xmin: other.xmin,
                xmax: other.xmax,
                ymin: other.ymin,
                ymax: other.ymax,
                zmin: other.zmin,
                zmax: other.zmax,
                is_void: false,
            };
            return;
        }
        if other.xmin < self.xmin {
            self.xmin = other.xmin;
        }
        if other.xmax > self.xmax {
            self.xmax = other.xmax;
        }
        if other.ymin < self.ymin {
            self.ymin = other.ymin;
        }
        if other.ymax > self.ymax {
            self.ymax = other.ymax;
        }
        if other.zmin < self.zmin {
            self.zmin = other.zmin;
        }
        if other.zmax > self.zmax {
            self.zmax = other.zmax;
        }
    }
}

// occt-note: shape type utilities
pub struct DeShapeClassifier;

impl DeShapeClassifier {
    pub fn classify(type_name: &str) -> DeShapeType {
        match type_name {
            "VERTEX" => DeShapeType::Vertex,
            "EDGE" => DeShapeType::Edge,
            "WIRE" => DeShapeType::Wire,
            "FACE" => DeShapeType::Face,
            "SHELL" => DeShapeType::Shell,
            "SOLID" => DeShapeType::Solid,
            "COMPSOLID" => DeShapeType::CompSolid,
            "COMPOUND" => DeShapeType::Compound,
            _ => DeShapeType::Shape,
        }
    }

    pub fn is_manifold(st: DeShapeType) -> bool {
        matches!(
            st,
            DeShapeType::Solid
                | DeShapeType::Shell
                | DeShapeType::Face
                | DeShapeType::Wire
                | DeShapeType::Edge
                | DeShapeType::Vertex
        )
    }

    pub fn is_closed(st: DeShapeType) -> bool {
        matches!(
            st,
            DeShapeType::Shell | DeShapeType::Solid | DeShapeType::Wire | DeShapeType::Edge
        )
    }

    pub fn dimension(st: DeShapeType) -> u8 {
        match st {
            DeShapeType::Vertex => 0,
            DeShapeType::Edge => 1,
            DeShapeType::Wire => 1,
            DeShapeType::Face => 2,
            DeShapeType::Shell => 2,
            DeShapeType::Solid => 3,
            DeShapeType::CompSolid => 3,
            DeShapeType::Compound => 3,
            DeShapeType::Shape => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_type_ranks_match_discriminants() {
        assert_eq!(DeShapeType::Vertex.rank(), 0);
        assert_eq!(DeShapeType::Edge.rank(), 1);
        assert_eq!(DeShapeType::Wire.rank(), 2);
        assert_eq!(DeShapeType::Face.rank(), 3);
        assert_eq!(DeShapeType::Shell.rank(), 4);
        assert_eq!(DeShapeType::Solid.rank(), 5);
        assert_eq!(DeShapeType::CompSolid.rank(), 6);
        assert_eq!(DeShapeType::Compound.rank(), 7);
        assert_eq!(DeShapeType::Shape.rank(), 8);
    }

    #[test]
    fn shape_type_names_correct() {
        assert_eq!(DeShapeType::Vertex.name(), "VERTEX");
        assert_eq!(DeShapeType::Edge.name(), "EDGE");
        assert_eq!(DeShapeType::Solid.name(), "SOLID");
        assert_eq!(DeShapeType::CompSolid.name(), "COMPSOLID");
        assert_eq!(DeShapeType::Shape.name(), "SHAPE");
    }

    #[test]
    fn shape_type_ordering() {
        assert!(DeShapeType::Vertex < DeShapeType::Edge);
        assert!(DeShapeType::Edge < DeShapeType::Wire);
        assert!(DeShapeType::Wire < DeShapeType::Face);
        assert!(DeShapeType::Face < DeShapeType::Shell);
        assert!(DeShapeType::Shell < DeShapeType::Solid);
        assert!(DeShapeType::Solid < DeShapeType::CompSolid);
        assert!(DeShapeType::CompSolid < DeShapeType::Compound);
        assert!(DeShapeType::Compound < DeShapeType::Shape);
    }

    #[test]
    fn stats_new_all_zero() {
        let s = DeShapeStats::new();
        assert_eq!(s.total(), 0);
        assert_eq!(s.nb_vertices, 0);
        assert_eq!(s.nb_solids, 0);
    }

    #[test]
    fn stats_total_and_add() {
        let mut a = DeShapeStats::new();
        a.nb_vertices = 3;
        a.nb_faces = 2;

        let mut b = DeShapeStats::new();
        b.nb_vertices = 1;
        b.nb_solids = 4;

        a.add(&b);
        assert_eq!(a.nb_vertices, 4);
        assert_eq!(a.nb_faces, 2);
        assert_eq!(a.nb_solids, 4);
        assert_eq!(a.total(), 10);
    }

    #[test]
    fn bbox_void_state() {
        let bb = DeShapeBoundingBox::new_void();
        assert!(bb.is_void());
        assert_eq!(bb.diagonal(), 0.0);
        assert_eq!(bb.volume(), 0.0);
        assert!(!bb.contains_point(0.0, 0.0, 0.0));
    }

    #[test]
    fn bbox_add_point_transitions_from_void() {
        let mut bb = DeShapeBoundingBox::new_void();
        bb.add_point(1.0, 2.0, 3.0);
        assert!(!bb.is_void());
        assert_eq!(bb.xmin, 1.0);
        assert_eq!(bb.xmax, 1.0);
        assert_eq!(bb.ymin, 2.0);
        assert_eq!(bb.zmax, 3.0);
    }

    #[test]
    fn bbox_diagonal_unit_cube() {
        let bb = DeShapeBoundingBox::from_min_max(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        let d = bb.diagonal();
        assert!((d - 3.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn bbox_merge_void_into_nonempty() {
        let mut bb = DeShapeBoundingBox::from_min_max(0.0, 2.0, 0.0, 2.0, 0.0, 2.0);
        let empty = DeShapeBoundingBox::new_void();
        bb.merge(&empty);
        assert_eq!(bb.xmax, 2.0);
    }

    #[test]
    fn bbox_merge_expands_bounds() {
        let mut a = DeShapeBoundingBox::from_min_max(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        let b = DeShapeBoundingBox::from_min_max(-1.0, 3.0, 0.5, 2.0, -0.5, 1.5);
        a.merge(&b);
        assert_eq!(a.xmin, -1.0);
        assert_eq!(a.xmax, 3.0);
        assert_eq!(a.ymin, 0.0);
        assert_eq!(a.ymax, 2.0);
        assert_eq!(a.zmin, -0.5);
        assert_eq!(a.zmax, 1.5);
    }

    #[test]
    fn classifier_classify_all_known() {
        assert_eq!(DeShapeClassifier::classify("VERTEX"), DeShapeType::Vertex);
        assert_eq!(DeShapeClassifier::classify("EDGE"), DeShapeType::Edge);
        assert_eq!(DeShapeClassifier::classify("WIRE"), DeShapeType::Wire);
        assert_eq!(DeShapeClassifier::classify("FACE"), DeShapeType::Face);
        assert_eq!(DeShapeClassifier::classify("SHELL"), DeShapeType::Shell);
        assert_eq!(DeShapeClassifier::classify("SOLID"), DeShapeType::Solid);
        assert_eq!(
            DeShapeClassifier::classify("COMPSOLID"),
            DeShapeType::CompSolid
        );
        assert_eq!(
            DeShapeClassifier::classify("COMPOUND"),
            DeShapeType::Compound
        );
        assert_eq!(DeShapeClassifier::classify("SHAPE"), DeShapeType::Shape);
    }

    #[test]
    fn classifier_classify_unknown_falls_back_to_shape() {
        assert_eq!(DeShapeClassifier::classify("UNKNOWN"), DeShapeType::Shape);
        assert_eq!(DeShapeClassifier::classify(""), DeShapeType::Shape);
    }

    #[test]
    fn classifier_is_manifold() {
        assert!(DeShapeClassifier::is_manifold(DeShapeType::Solid));
        assert!(DeShapeClassifier::is_manifold(DeShapeType::Shell));
        assert!(DeShapeClassifier::is_manifold(DeShapeType::Face));
        assert!(DeShapeClassifier::is_manifold(DeShapeType::Wire));
        assert!(DeShapeClassifier::is_manifold(DeShapeType::Edge));
        assert!(DeShapeClassifier::is_manifold(DeShapeType::Vertex));
        assert!(!DeShapeClassifier::is_manifold(DeShapeType::CompSolid));
        assert!(!DeShapeClassifier::is_manifold(DeShapeType::Compound));
        assert!(!DeShapeClassifier::is_manifold(DeShapeType::Shape));
    }

    #[test]
    fn classifier_is_closed() {
        assert!(DeShapeClassifier::is_closed(DeShapeType::Shell));
        assert!(DeShapeClassifier::is_closed(DeShapeType::Solid));
        assert!(DeShapeClassifier::is_closed(DeShapeType::Wire));
        assert!(DeShapeClassifier::is_closed(DeShapeType::Edge));
        assert!(!DeShapeClassifier::is_closed(DeShapeType::Vertex));
        assert!(!DeShapeClassifier::is_closed(DeShapeType::Face));
        assert!(!DeShapeClassifier::is_closed(DeShapeType::Compound));
    }

    #[test]
    fn classifier_dimension() {
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Vertex), 0);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Edge), 1);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Wire), 1);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Face), 2);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Shell), 2);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Solid), 3);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::CompSolid), 3);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Compound), 3);
        assert_eq!(DeShapeClassifier::dimension(DeShapeType::Shape), 3);
    }
}
