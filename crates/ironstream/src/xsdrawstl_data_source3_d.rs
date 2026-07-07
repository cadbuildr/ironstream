// FILE: xsdrawstl_data_source3_d.rs
// occt: XSDRAWSTL_DataSource3D
//
// Faithful port of OCCT XSDRAWSTL_DataSource3D
// (Draw/TKXSDRAWSTL/XSDRAWSTL_DataSource3D.cxx/.hxx), the sample volumic
// MeshVS data source with a hard-coded mesh of 16 nodes and 5 volume
// elements (tetra, hexa, prism, hexa, tetra). The constructor tables,
// GetGeom, Get3DGeom (per-face local connectivity), GetGeomType, GetAddr,
// GetNodesByElement, GetAllNodes, GetAllElements and GetNormal (always
// false) reproduce the .cxx exactly.
//
// The packed integer map is modelled by a BTreeSet; the mesh payload is
// real and tested.

use std::collections::BTreeSet;

/// Local model of MeshVS_EntityType (values used by this data source).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stl3dEntityType {
    Node,
    Volume,
}

/// Port of XSDRAWSTL_DataSource3D.
#[derive(Debug)]
pub struct XsdrawstlDataSource3D {
    my_nodes: BTreeSet<i32>,
    my_elements: BTreeSet<i32>,
    my_elem_nb_nodes: [i32; 5],
    my_node_coords: [[f64; 3]; 16],
    /// (5 x 8) node indices; unused tail slots are 0.
    my_elem_nodes: [[i32; 8]; 5],
}

impl Default for XsdrawstlDataSource3D {
    fn default() -> Self {
        Self::new()
    }
}

impl XsdrawstlDataSource3D {
    /// Mirrors the XSDRAWSTL_DataSource3D() constructor tables.
    pub fn new() -> Self {
        let mut my_nodes = BTreeSet::new();
        for a_node_id in 1..=16 {
            my_nodes.insert(a_node_id);
        }
        let mut my_elements = BTreeSet::new();
        for an_elem_id in 1..=5 {
            my_elements.insert(an_elem_id);
        }
        let my_node_coords: [[f64; 3]; 16] = [
            [5.0, 5.0, 20.0],
            [0.0, 10.0, 10.0],
            [10.0, 0.0, 10.0],
            [0.0, 0.0, 10.0],
            [-10.0, 0.0, 10.0],
            [-10.0, 10.0, 10.0],
            [-10.0, 10.0, 0.0],
            [-10.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 10.0, 0.0],
            [0.0, -10.0, 10.0],
            [10.0, -10.0, 10.0],
            [10.0, -10.0, 0.0],
            [0.0, -10.0, 0.0],
            [10.0, 0.0, 0.0],
            [5.0, 5.0, -10.0],
        ];
        let my_elem_nb_nodes = [4, 8, 6, 8, 4];
        let my_elem_nodes: [[i32; 8]; 5] = [
            [1, 2, 3, 4, 0, 0, 0, 0],
            [2, 4, 5, 6, 7, 8, 9, 10],
            [2, 3, 4, 10, 15, 9, 0, 0],
            [4, 3, 12, 11, 14, 13, 15, 9],
            [16, 15, 10, 9, 0, 0, 0, 0],
        ];
        XsdrawstlDataSource3D {
            my_nodes,
            my_elements,
            my_elem_nb_nodes,
            my_node_coords,
            my_elem_nodes,
        }
    }

    /// Mirrors GetGeom: for an element returns 3*NbNodes coordinates in
    /// connectivity order with type Volume; for a node its 3 coordinates
    /// with NbNodes=1 and type Node.
    pub fn get_geom(
        &self,
        the_id: i32,
        the_is_element: bool,
        the_coords: &mut [f64],
    ) -> Option<(i32, Stl3dEntityType)> {
        if the_is_element {
            if the_id >= 1 && the_id <= self.my_elements.len() as i32 {
                let nb_nodes = self.my_elem_nb_nodes[(the_id - 1) as usize];
                let mut glob = 0usize;
                for a_node_i in 0..nb_nodes as usize {
                    let an_idx_node = self.my_elem_nodes[(the_id - 1) as usize][a_node_i];
                    for a_coord_i in 0..3usize {
                        the_coords[glob] = self.my_node_coords[(an_idx_node - 1) as usize][a_coord_i];
                        glob += 1;
                    }
                }
                Some((nb_nodes, Stl3dEntityType::Volume))
            } else {
                None
            }
        } else if the_id >= 1 && the_id <= self.my_nodes.len() as i32 {
            let c = self.my_node_coords[(the_id - 1) as usize];
            the_coords[0] = c[0];
            the_coords[1] = c[1];
            the_coords[2] = c[2];
            Some((1, Stl3dEntityType::Node))
        } else {
            None
        }
    }

    /// Mirrors Get3DGeom: returns (NbNodes, faces) where each face is the
    /// sequence of 0-based local node indices of the .cxx tables:
    ///   ID 1/5 (tetra): 4 nodes, 4 triangular faces;
    ///   ID 2/4 (hexa): 8 nodes, 6 quad faces;
    ///   ID 3 (prism): 6 nodes, 2 triangles + 3 quads.
    pub fn get_3d_geom(&self, the_id: i32) -> Option<(i32, Vec<Vec<i32>>)> {
        if the_id == 1 || the_id == 5 {
            let mut a_mesh_data: Vec<Vec<i32>> = Vec::with_capacity(4);
            for an_elem_i in 1..=4i32 {
                a_mesh_data.push(vec![
                    (an_elem_i - 1) % 4,
                    an_elem_i % 4,
                    (an_elem_i + 1) % 4,
                ]);
            }
            return Some((4, a_mesh_data));
        }
        if the_id == 2 || the_id == 4 {
            let mut a_mesh_data: Vec<Vec<i32>> = Vec::with_capacity(6);
            let mut k = 1i32;
            for _an_elem_i in 1..=4 {
                a_mesh_data.push(vec![(k - 1) % 8, k % 8, (k + 1) % 8, (k + 2) % 8]);
                k += 2;
            }
            a_mesh_data.push(vec![0, 3, 4, 7]);
            a_mesh_data.push(vec![1, 2, 5, 6]);
            return Some((8, a_mesh_data));
        }
        if the_id == 3 {
            let mut a_mesh_data: Vec<Vec<i32>> = Vec::with_capacity(5);
            for an_elem_i in 1..=2i32 {
                a_mesh_data.push(vec![
                    (an_elem_i - 1) * 3,
                    (an_elem_i - 1) * 3 + 1,
                    (an_elem_i - 1) * 3 + 2,
                ]);
            }
            for an_elem_i in 1..=3i32 {
                a_mesh_data.push(vec![
                    (an_elem_i - 1) % 3,
                    an_elem_i % 3,
                    an_elem_i % 3 + 3,
                    (an_elem_i - 1) % 3 + 3,
                ]);
            }
            return Some((6, a_mesh_data));
        }
        None
    }

    /// Mirrors GetGeomType.
    pub fn get_geom_type(&self, the_id: i32, the_is_element: bool) -> Option<Stl3dEntityType> {
        if the_is_element {
            if the_id >= 1 && the_id <= self.my_elements.len() as i32 {
                return Some(Stl3dEntityType::Volume);
            }
        } else if the_id >= 1 && the_id <= self.my_nodes.len() as i32 {
            return Some(Stl3dEntityType::Node);
        }
        None
    }

    /// Mirrors GetAddr: always null.
    pub fn get_addr(&self, _the_id: i32, _the_is_element: bool) -> Option<usize> {
        None
    }

    /// Mirrors GetNodesByElement: fills the connectivity and returns
    /// NbNodes (4 for elements 1/5, 8 for 2/4, 6 for 3).
    pub fn get_nodes_by_element(&self, the_id: i32, the_node_ids: &mut [i32]) -> Option<i32> {
        let nb_nodes = match the_id {
            1 | 5 => 4,
            2 | 4 => 8,
            3 => 6,
            _ => return None,
        };
        for i in 0..nb_nodes as usize {
            the_node_ids[i] = self.my_elem_nodes[(the_id - 1) as usize][i];
        }
        Some(nb_nodes)
    }

    /// Mirrors GetAllNodes.
    pub fn get_all_nodes(&self) -> &BTreeSet<i32> {
        &self.my_nodes
    }

    /// Mirrors GetAllElements.
    pub fn get_all_elements(&self) -> &BTreeSet<i32> {
        &self.my_elements
    }

    /// Mirrors GetNormal: always false for volumic elements.
    pub fn get_normal(&self, _the_id: i32, _the_max: i32) -> Option<(f64, f64, f64)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_builds_sixteen_nodes_and_five_elements() {
        let ds = XsdrawstlDataSource3D::new();
        assert_eq!(ds.get_all_nodes().len(), 16);
        assert_eq!(ds.get_all_elements().len(), 5);
        assert!(ds.get_all_nodes().contains(&16));
        assert!(!ds.get_all_nodes().contains(&17));
    }

    #[test]
    fn get_geom_node_coordinates_match_tables() {
        let ds = XsdrawstlDataSource3D::new();
        let mut c = [0.0f64; 3];
        let (nb, ty) = ds.get_geom(1, false, &mut c).unwrap();
        assert_eq!((nb, ty), (1, Stl3dEntityType::Node));
        assert_eq!(c, [5.0, 5.0, 20.0]);
        ds.get_geom(11, false, &mut c).unwrap();
        assert_eq!(c, [0.0, -10.0, 10.0]);
        ds.get_geom(16, false, &mut c).unwrap();
        assert_eq!(c, [5.0, 5.0, -10.0]);
        assert!(ds.get_geom(17, false, &mut c).is_none());
    }

    #[test]
    fn get_geom_element_expands_connectivity() {
        let ds = XsdrawstlDataSource3D::new();
        let mut c = [0.0f64; 24];
        // element 1 = tetra of nodes 1,2,3,4
        let (nb, ty) = ds.get_geom(1, true, &mut c).unwrap();
        assert_eq!((nb, ty), (4, Stl3dEntityType::Volume));
        assert_eq!(
            &c[..12],
            &[5.0, 5.0, 20.0, 0.0, 10.0, 10.0, 10.0, 0.0, 10.0, 0.0, 0.0, 10.0]
        );
        // element 2 = hexa with 8 nodes -> 24 coordinates
        let (nb2, _) = ds.get_geom(2, true, &mut c).unwrap();
        assert_eq!(nb2, 8);
        // last node of element 2 is node 10 = (0,10,0)
        assert_eq!(&c[21..24], &[0.0, 10.0, 0.0]);
        assert!(ds.get_geom(6, true, &mut c).is_none());
    }

    #[test]
    fn get_3d_geom_tetra_faces() {
        let ds = XsdrawstlDataSource3D::new();
        let (nb, faces) = ds.get_3d_geom(1).unwrap();
        assert_eq!(nb, 4);
        assert_eq!(
            faces,
            vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 0], vec![3, 0, 1]]
        );
        assert_eq!(ds.get_3d_geom(5).unwrap(), (4, faces));
    }

    #[test]
    fn get_3d_geom_hexa_faces() {
        let ds = XsdrawstlDataSource3D::new();
        let (nb, faces) = ds.get_3d_geom(2).unwrap();
        assert_eq!(nb, 8);
        assert_eq!(faces.len(), 6);
        assert_eq!(faces[0], vec![0, 1, 2, 3]);
        assert_eq!(faces[1], vec![2, 3, 4, 5]);
        assert_eq!(faces[2], vec![4, 5, 6, 7]);
        assert_eq!(faces[3], vec![6, 7, 0, 1]);
        assert_eq!(faces[4], vec![0, 3, 4, 7]);
        assert_eq!(faces[5], vec![1, 2, 5, 6]);
        assert_eq!(ds.get_3d_geom(4).unwrap().1, faces);
    }

    #[test]
    fn get_3d_geom_prism_faces_and_invalid_id() {
        let ds = XsdrawstlDataSource3D::new();
        let (nb, faces) = ds.get_3d_geom(3).unwrap();
        assert_eq!(nb, 6);
        assert_eq!(faces.len(), 5);
        assert_eq!(faces[0], vec![0, 1, 2]);
        assert_eq!(faces[1], vec![3, 4, 5]);
        assert_eq!(faces[2], vec![0, 1, 4, 3]);
        assert_eq!(faces[3], vec![1, 2, 5, 4]);
        assert_eq!(faces[4], vec![2, 0, 3, 5]);
        assert!(ds.get_3d_geom(6).is_none());
    }

    #[test]
    fn get_nodes_by_element_matches_tables() {
        let ds = XsdrawstlDataSource3D::new();
        let mut ids = [0i32; 8];
        assert_eq!(ds.get_nodes_by_element(1, &mut ids), Some(4));
        assert_eq!(&ids[..4], &[1, 2, 3, 4]);
        assert_eq!(ds.get_nodes_by_element(2, &mut ids), Some(8));
        assert_eq!(&ids[..8], &[2, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(ds.get_nodes_by_element(3, &mut ids), Some(6));
        assert_eq!(&ids[..6], &[2, 3, 4, 10, 15, 9]);
        assert_eq!(ds.get_nodes_by_element(4, &mut ids), Some(8));
        assert_eq!(&ids[..8], &[4, 3, 12, 11, 14, 13, 15, 9]);
        assert_eq!(ds.get_nodes_by_element(5, &mut ids), Some(4));
        assert_eq!(&ids[..4], &[16, 15, 10, 9]);
        assert_eq!(ds.get_nodes_by_element(6, &mut ids), None);
    }

    #[test]
    fn geom_type_addr_and_normal() {
        let ds = XsdrawstlDataSource3D::new();
        assert_eq!(ds.get_geom_type(3, true), Some(Stl3dEntityType::Volume));
        assert_eq!(ds.get_geom_type(6, true), None);
        assert_eq!(ds.get_geom_type(16, false), Some(Stl3dEntityType::Node));
        assert_eq!(ds.get_geom_type(17, false), None);
        assert_eq!(ds.get_addr(1, true), None);
        assert_eq!(ds.get_normal(1, 3), None);
    }
}
