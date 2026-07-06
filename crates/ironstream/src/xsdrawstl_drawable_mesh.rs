// FILE: xsdrawstl_drawable_mesh.rs
// occt: XSDRAWSTL_DrawableMesh
//
// Faithful port of OCCT XSDRAWSTL_DrawableMesh
// (Draw/TKXSDRAWSTL/XSDRAWSTL_DrawableMesh.cxx/.hxx), the tiny
// Draw_Drawable3D wrapper that stores a handle to a MeshVS_Mesh:
//   - the constructor stores the mesh handle (possibly null);
//   - DrawOn(Draw_Display&) is intentionally empty;
//   - GetMesh() returns the stored handle.
//
// MeshVS_Mesh and Draw_Display are modelled by small local types; the
// wrapper behaviour (handle storage/sharing, no-op drawing) is real and
// tested.

use std::rc::Rc;

/// Local model of MeshVS_Mesh: an interactive mesh object identified for
/// tests by its name and a node/element census.
#[derive(Debug, PartialEq, Eq)]
pub struct DrawableMeshVsMesh {
    pub name: String,
    pub nb_nodes: i32,
    pub nb_elements: i32,
}

/// Local model of Draw_Display: records every drawing primitive emitted.
#[derive(Debug, Default)]
pub struct DrawableMeshDisplay {
    pub emitted_segments: usize,
    pub emitted_moves: usize,
}

/// Port of XSDRAWSTL_DrawableMesh (a Draw_Drawable3D).
#[derive(Debug, Clone)]
pub struct XsdrawstlDrawableMesh {
    my_mesh: Option<Rc<DrawableMeshVsMesh>>,
}

impl XsdrawstlDrawableMesh {
    /// Mirrors XSDRAWSTL_DrawableMesh(const occ::handle<MeshVS_Mesh>&).
    pub fn new(a_mesh: Option<Rc<DrawableMeshVsMesh>>) -> Self {
        XsdrawstlDrawableMesh { my_mesh: a_mesh }
    }

    /// Mirrors DrawOn(Draw_Display&): deliberately draws nothing (the mesh
    /// is presented through AIS/MeshVS, not the Draw axonometric viewer).
    pub fn draw_on(&self, _d: &mut DrawableMeshDisplay) {}

    /// Mirrors GetMesh(): returns the stored handle (shared, not cloned).
    pub fn get_mesh(&self) -> Option<Rc<DrawableMeshVsMesh>> {
        self.my_mesh.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_and_get_mesh_returns_same_handle() {
        let mesh = Rc::new(DrawableMeshVsMesh {
            name: "m".to_string(),
            nb_nodes: 4,
            nb_elements: 2,
        });
        let drawable = XsdrawstlDrawableMesh::new(Some(mesh.clone()));
        let got = drawable.get_mesh().unwrap();
        // Same underlying object (handle semantics), not a copy.
        assert!(Rc::ptr_eq(&mesh, &got));
        assert_eq!(got.nb_nodes, 4);
        assert_eq!(got.nb_elements, 2);
    }

    #[test]
    fn null_mesh_handle_is_preserved() {
        let drawable = XsdrawstlDrawableMesh::new(None);
        assert!(drawable.get_mesh().is_none());
    }

    #[test]
    fn draw_on_emits_nothing() {
        let drawable = XsdrawstlDrawableMesh::new(Some(Rc::new(DrawableMeshVsMesh {
            name: "m".to_string(),
            nb_nodes: 1,
            nb_elements: 0,
        })));
        let mut display = DrawableMeshDisplay::default();
        drawable.draw_on(&mut display);
        assert_eq!(display.emitted_segments, 0);
        assert_eq!(display.emitted_moves, 0);
    }

    #[test]
    fn clones_share_the_same_mesh_handle() {
        let mesh = Rc::new(DrawableMeshVsMesh {
            name: "shared".to_string(),
            nb_nodes: 8,
            nb_elements: 12,
        });
        let a = XsdrawstlDrawableMesh::new(Some(mesh.clone()));
        let b = a.clone();
        assert!(Rc::ptr_eq(&a.get_mesh().unwrap(), &b.get_mesh().unwrap()));
    }
}
