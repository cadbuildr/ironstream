// FILE: xsdrawstl_data_source.rs
// occt: XSDRAWSTL_DataSource
//
// Faithful port of OCCT XSDRAWSTL_DataSource
// (Draw/TKXSDRAWSTL/XSDRAWSTL_DataSource.cxx/.hxx), the sample
// MeshVS_DataSource built on a Poly_Triangulation:
//   - the constructor caches node coordinates, element connectivity and
//     per-triangle normals (cross product of the two edge vectors,
//     normalized unless degenerate below Precision::SquareConfusion, in
//     which case it is zeroed);
//   - GetGeom / GetGeomType / GetNodesByElement / GetAllNodes /
//     GetAllElements / GetNormal reproduce the .cxx logic exactly.
//
// Poly_Triangulation and TColStd_PackedMapOfInteger are modelled by small
// local types; the data-source payload behaviour is real and tested.

use std::collections::BTreeSet;

/// Local model of MeshVS_EntityType (values used by this data source).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StlSourceEntityType {
    Node,
    Face,
}

/// Local model of Poly_Triangulation: 1-based nodes and triangles.
#[derive(Debug, Clone, Default)]
pub struct StlSourceTriangulation {
    nodes: Vec<[f64; 3]>,
    /// 1-based node indices per triangle.
    triangles: Vec<[i32; 3]>,
}

impl StlSourceTriangulation {
    pub fn new(nodes: Vec<[f64; 3]>, triangles: Vec<[i32; 3]>) -> Self {
        StlSourceTriangulation { nodes, triangles }
    }

    pub fn nb_nodes(&self) -> i32 {
        self.nodes.len() as i32
    }

    pub fn nb_triangles(&self) -> i32 {
        self.triangles.len() as i32
    }

    /// 1-based node accessor (Poly_Triangulation::Node).
    pub fn node(&self, index: i32) -> [f64; 3] {
        self.nodes[(index - 1) as usize]
    }

    /// 1-based triangle accessor (Poly_Triangulation::Triangle).
    pub fn triangle(&self, index: i32) -> [i32; 3] {
        self.triangles[(index - 1) as usize]
    }
}

/// Precision::SquareConfusion() = Confusion()^2 = (1e-7)^2.
const STL_SOURCE_SQUARE_CONFUSION: f64 = 1.0e-14;

/// Port of XSDRAWSTL_DataSource.
#[derive(Debug)]
pub struct XsdrawstlDataSource {
    my_mesh: Option<StlSourceTriangulation>,
    my_nodes: BTreeSet<i32>,
    my_elements: BTreeSet<i32>,
    /// (nbTris x 3) node indices.
    my_elem_nodes: Vec<[i32; 3]>,
    /// (nbNodes x 3) coordinates.
    my_node_coords: Vec<[f64; 3]>,
    /// (nbTris x 3) normals.
    my_elem_normals: Vec<[f64; 3]>,
}

impl XsdrawstlDataSource {
    /// Mirrors XSDRAWSTL_DataSource::XSDRAWSTL_DataSource(mesh).
    pub fn new(a_mesh: Option<StlSourceTriangulation>) -> Self {
        let mut ds = XsdrawstlDataSource {
            my_mesh: a_mesh,
            my_nodes: BTreeSet::new(),
            my_elements: BTreeSet::new(),
            my_elem_nodes: Vec::new(),
            my_node_coords: Vec::new(),
            my_elem_normals: Vec::new(),
        };
        if let Some(mesh) = ds.my_mesh.clone() {
            let a_nb_nodes = mesh.nb_nodes();
            for i in 1..=a_nb_nodes {
                ds.my_nodes.insert(i);
                let xyz = mesh.node(i);
                ds.my_node_coords.push(xyz);
            }
            let a_nb_tris = mesh.nb_triangles();
            for i in 1..=a_nb_tris {
                ds.my_elements.insert(i);
                let v = mesh.triangle(i);
                let a_p1 = mesh.node(v[0]);
                let a_p2 = mesh.node(v[1]);
                let a_p3 = mesh.node(v[2]);
                // aV1 = P1->P2, aV2 = P2->P3, aN = aV1 x aV2
                let a_v1 = [a_p2[0] - a_p1[0], a_p2[1] - a_p1[1], a_p2[2] - a_p1[2]];
                let a_v2 = [a_p3[0] - a_p2[0], a_p3[1] - a_p2[1], a_p3[2] - a_p2[2]];
                let mut a_n = [
                    a_v1[1] * a_v2[2] - a_v1[2] * a_v2[1],
                    a_v1[2] * a_v2[0] - a_v1[0] * a_v2[2],
                    a_v1[0] * a_v2[1] - a_v1[1] * a_v2[0],
                ];
                let sq_mag = a_n[0] * a_n[0] + a_n[1] * a_n[1] + a_n[2] * a_n[2];
                if sq_mag > STL_SOURCE_SQUARE_CONFUSION {
                    let mag = sq_mag.sqrt();
                    a_n = [a_n[0] / mag, a_n[1] / mag, a_n[2] / mag];
                } else {
                    a_n = [0.0, 0.0, 0.0];
                }
                ds.my_elem_nodes.push(v);
                ds.my_elem_normals.push(a_n);
            }
        }
        ds
    }

    /// Mirrors GetGeom: for an element returns the 9 coordinates of its
    /// three nodes (X,Y,Z per node in wire order) with NbNodes=3 and type
    /// Face; for a node its 3 coordinates with NbNodes=1 and type Node.
    pub fn get_geom(
        &self,
        id: i32,
        is_element: bool,
        coords: &mut [f64],
    ) -> Option<(i32, StlSourceEntityType)> {
        self.my_mesh.as_ref()?;
        if is_element {
            if id >= 1 && id <= self.my_elements.len() as i32 {
                let mut k = 0usize;
                for i in 0..3usize {
                    let idx_node = self.my_elem_nodes[(id - 1) as usize][i];
                    for j in 0..3usize {
                        coords[k] = self.my_node_coords[(idx_node - 1) as usize][j];
                        k += 1;
                    }
                }
                Some((3, StlSourceEntityType::Face))
            } else {
                None
            }
        } else if id >= 1 && id <= self.my_nodes.len() as i32 {
            let c = self.my_node_coords[(id - 1) as usize];
            coords[0] = c[0];
            coords[1] = c[1];
            coords[2] = c[2];
            Some((1, StlSourceEntityType::Node))
        } else {
            None
        }
    }

    /// Mirrors GetGeomType: elements are faces, nodes are nodes.
    pub fn get_geom_type(&self, _id: i32, is_element: bool) -> StlSourceEntityType {
        if is_element {
            StlSourceEntityType::Face
        } else {
            StlSourceEntityType::Node
        }
    }

    /// Mirrors GetAddr: always null.
    pub fn get_addr(&self, _id: i32, _is_element: bool) -> Option<usize> {
        None
    }

    /// Mirrors GetNodesByElement: fills the first three slots of
    /// `node_ids` with the element connectivity.
    pub fn get_nodes_by_element(&self, id: i32, node_ids: &mut [i32]) -> bool {
        if self.my_mesh.is_none() {
            return false;
        }
        if id >= 1 && id <= self.my_elements.len() as i32 && node_ids.len() >= 3 {
            let v = self.my_elem_nodes[(id - 1) as usize];
            node_ids[0] = v[0];
            node_ids[1] = v[1];
            node_ids[2] = v[2];
            true
        } else {
            false
        }
    }

    /// Mirrors GetAllNodes.
    pub fn get_all_nodes(&self) -> &BTreeSet<i32> {
        &self.my_nodes
    }

    /// Mirrors GetAllElements.
    pub fn get_all_elements(&self) -> &BTreeSet<i32> {
        &self.my_elements
    }

    /// Mirrors GetNormal: returns the cached per-triangle normal when
    /// Id is a valid element and Max >= 3.
    pub fn get_normal(&self, id: i32, max: i32) -> Option<(f64, f64, f64)> {
        self.my_mesh.as_ref()?;
        if id >= 1 && id <= self.my_elements.len() as i32 && max >= 3 {
            let n = self.my_elem_normals[(id - 1) as usize];
            Some((n[0], n[1], n[2]))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_triangle_mesh() -> StlSourceTriangulation {
        // Unit square in XY split into two triangles, plus a degenerate one.
        StlSourceTriangulation::new(
            vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            vec![[1, 2, 3], [1, 3, 4]],
        )
    }

    #[test]
    fn constructor_populates_maps_and_normals() {
        let ds = XsdrawstlDataSource::new(Some(two_triangle_mesh()));
        assert_eq!(ds.get_all_nodes().len(), 4);
        assert_eq!(ds.get_all_elements().len(), 2);
        assert!(ds.get_all_nodes().contains(&1) && ds.get_all_nodes().contains(&4));
        // planar CCW triangles in XY: normal +Z
        let (nx, ny, nz) = ds.get_normal(1, 3).unwrap();
        assert!((nx, ny).0.abs() < 1e-15 && ny.abs() < 1e-15);
        assert!((nz - 1.0).abs() < 1e-15);
    }

    #[test]
    fn degenerate_triangle_gets_zero_normal() {
        let ds = XsdrawstlDataSource::new(Some(StlSourceTriangulation::new(
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
            vec![[1, 2, 3]],
        )));
        assert_eq!(ds.get_normal(1, 3), Some((0.0, 0.0, 0.0)));
    }

    #[test]
    fn get_geom_element_returns_nine_coords_in_wire_order() {
        let ds = XsdrawstlDataSource::new(Some(two_triangle_mesh()));
        let mut coords = [0.0f64; 9];
        let (nb, ty) = ds.get_geom(2, true, &mut coords).unwrap();
        assert_eq!(nb, 3);
        assert_eq!(ty, StlSourceEntityType::Face);
        // triangle 2 = nodes 1,3,4
        assert_eq!(
            coords,
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0]
        );
        // out of range element
        assert!(ds.get_geom(3, true, &mut coords).is_none());
    }

    #[test]
    fn get_geom_node_returns_three_coords() {
        let ds = XsdrawstlDataSource::new(Some(two_triangle_mesh()));
        let mut coords = [0.0f64; 3];
        let (nb, ty) = ds.get_geom(3, false, &mut coords).unwrap();
        assert_eq!(nb, 1);
        assert_eq!(ty, StlSourceEntityType::Node);
        assert_eq!(coords, [1.0, 1.0, 0.0]);
        assert!(ds.get_geom(0, false, &mut coords).is_none());
        assert!(ds.get_geom(5, false, &mut coords).is_none());
    }

    #[test]
    fn get_nodes_by_element_and_geom_type() {
        let ds = XsdrawstlDataSource::new(Some(two_triangle_mesh()));
        let mut ids = [0i32; 3];
        assert!(ds.get_nodes_by_element(1, &mut ids));
        assert_eq!(ids, [1, 2, 3]);
        let mut small = [0i32; 2];
        assert!(!ds.get_nodes_by_element(1, &mut small));
        assert!(!ds.get_nodes_by_element(9, &mut ids));
        assert_eq!(ds.get_geom_type(1, true), StlSourceEntityType::Face);
        assert_eq!(ds.get_geom_type(1, false), StlSourceEntityType::Node);
        assert_eq!(ds.get_addr(1, true), None);
    }

    #[test]
    fn null_mesh_behaves_like_cxx() {
        let ds = XsdrawstlDataSource::new(None);
        let mut coords = [0.0f64; 9];
        assert!(ds.get_geom(1, true, &mut coords).is_none());
        let mut ids = [0i32; 3];
        assert!(!ds.get_nodes_by_element(1, &mut ids));
        assert!(ds.get_normal(1, 3).is_none());
        assert!(ds.get_all_nodes().is_empty());
        assert!(ds.get_all_elements().is_empty());
    }

    #[test]
    fn get_normal_requires_max_at_least_three() {
        let ds = XsdrawstlDataSource::new(Some(two_triangle_mesh()));
        assert!(ds.get_normal(1, 2).is_none());
        assert!(ds.get_normal(1, 3).is_some());
    }
}
