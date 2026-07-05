// FILE: db_rep.rs
// occt: DBRep

//! DBRep: Display and manage BRep (Boundary Representation) objects in Draw.
//! Provides methods to display shapes and manage a directory of named shapes.

use std::collections::HashMap;
use std::sync::Mutex;

/// Shape type enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Shape,
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    CompSolid,
    Compound,
}

/// A simplified representation of a TopoDS_Shape.
#[derive(Clone, Debug, PartialEq)]
pub struct TopodsShape {
    pub shape_type: ShapeType,
    pub name: String,
    pub data: Vec<u8>,
}

impl TopodsShape {
    pub fn new(shape_type: ShapeType, name: &str) -> Self {
        TopodsShape {
            shape_type,
            name: name.to_string(),
            data: Vec::new(),
        }
    }

    pub fn null() -> Self {
        TopodsShape {
            shape_type: ShapeType::Shape,
            name: String::new(),
            data: Vec::new(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.name.is_empty()
    }
}

/// DBRep parameters for display control.
#[derive(Clone, Debug)]
pub struct DbrepParams {
    pub with_hlr: bool,
    pub with_rg1: bool,
    pub with_rgn: bool,
    pub with_hid: bool,
    pub hlr_angle: f64,
    pub nb_isos: i32,
    pub discretization: i32,
}

impl Default for DbrepParams {
    fn default() -> Self {
        DbrepParams {
            with_hlr: false,
            with_rg1: true,
            with_rgn: true,
            with_hid: false,
            hlr_angle: 0.57,
            nb_isos: 10,
            discretization: 30,
        }
    }
}

/// Global shape storage for DBRep.
static SHAPE_STORAGE: Mutex<HashMap<String, TopodsShape>> = Mutex::new(HashMap::new());
static PARAMS: Mutex<DbrepParams> = Mutex::new(DbrepParams {
    with_hlr: false,
    with_rg1: true,
    with_rgn: true,
    with_hid: false,
    hlr_angle: 0.57,
    nb_isos: 10,
    discretization: 30,
});

/// DBRep: Display and manage BRep objects in the Draw interpreter.
pub struct Dbrep;

impl Dbrep {
    /// Set a shape in the variable with the given name.
    pub fn set(name: &str, shape: TopodsShape) {
        let mut storage = SHAPE_STORAGE.lock().unwrap();
        storage.insert(name.to_string(), shape);
    }

    /// Get a shape from the variable with the given name.
    /// Returns NULL shape if not found or type doesn't match.
    pub fn get(name: &str, shape_type: ShapeType, complain: bool) -> TopodsShape {
        let storage = SHAPE_STORAGE.lock().unwrap();

        match storage.get(name) {
            Some(shape) => {
                if shape_type == ShapeType::Shape || shape.shape_type == shape_type {
                    shape.clone()
                } else {
                    if complain {
                        eprintln!("Shape type mismatch for '{}'", name);
                    }
                    TopodsShape::null()
                }
            }
            None => {
                if complain {
                    eprintln!("Shape '{}' not found", name);
                }
                TopodsShape::null()
            }
        }
    }

    /// Get an existing shape without interactive picking.
    pub fn get_existing(name: &str, shape_type: ShapeType, complain: bool) -> TopodsShape {
        if name == "." {
            return TopodsShape::null();
        }
        Self::get(name, shape_type, complain)
    }

    /// Register basic Draw commands.
    pub fn basic_commands(_interpreter: &str) {
        // In a real implementation, this would register commands like:
        // - shape: display a shape
        // - dump: dump shape data
        // - rename: rename a shape
        // etc.
    }

    /// Get or initialize global parameters.
    pub fn parameters() -> DbrepParams {
        PARAMS.lock().unwrap().clone()
    }

    /// Set global parameters.
    pub fn set_parameters(params: DbrepParams) {
        *PARAMS.lock().unwrap() = params;
    }

    /// Check if HLR mode is enabled.
    pub fn hlr_mode() -> bool {
        PARAMS.lock().unwrap().with_hlr
    }

    /// Check if Rg1 lines are displayed.
    pub fn rg1_mode() -> bool {
        PARAMS.lock().unwrap().with_rg1
    }

    /// Check if RgN lines are displayed.
    pub fn rgn_mode() -> bool {
        PARAMS.lock().unwrap().with_rgn
    }

    /// Check if hidden lines are displayed.
    pub fn hid_mode() -> bool {
        PARAMS.lock().unwrap().with_hid
    }

    /// Get HLR discretisation angle.
    pub fn hlr_angle() -> f64 {
        PARAMS.lock().unwrap().hlr_angle
    }

    /// Get number of isos in U and V.
    pub fn nb_isos() -> i32 {
        PARAMS.lock().unwrap().nb_isos
    }

    /// Get discretization number of points for curves.
    pub fn discretisation() -> i32 {
        PARAMS.lock().unwrap().discretization
    }

    /// Clear all stored shapes.
    pub fn clear() {
        SHAPE_STORAGE.lock().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_shape() {
        Dbrep::clear();
        let shape = TopodsShape::new(ShapeType::Face, "test_face");
        Dbrep::set("myface", shape.clone());

        let retrieved = Dbrep::get("myface", ShapeType::Face, false);
        assert_eq!(retrieved, shape);
    }

    #[test]
    fn test_get_nonexistent_shape() {
        Dbrep::clear();
        let shape = Dbrep::get("nonexistent", ShapeType::Shape, false);
        assert!(shape.is_null());
    }

    #[test]
    fn test_shape_type_filter() {
        Dbrep::clear();
        let shape = TopodsShape::new(ShapeType::Edge, "myedge");
        Dbrep::set("myedge", shape);

        let retrieved = Dbrep::get("myedge", ShapeType::Edge, false);
        assert!(!retrieved.is_null());

        let mismatched = Dbrep::get("myedge", ShapeType::Face, false);
        assert!(mismatched.is_null());
    }

    #[test]
    fn test_get_existing() {
        Dbrep::clear();
        let shape = TopodsShape::new(ShapeType::Solid, "mysolid");
        Dbrep::set("mysolid", shape.clone());

        let retrieved = Dbrep::get_existing("mysolid", ShapeType::Solid, false);
        assert_eq!(retrieved, shape);

        let interactive = Dbrep::get_existing(".", ShapeType::Solid, false);
        assert!(interactive.is_null());
    }

    #[test]
    fn test_parameters() {
        let params = Dbrep::parameters();
        assert_eq!(params.nb_isos, 10);
        assert_eq!(params.discretization, 30);
    }

    #[test]
    fn test_hlr_mode() {
        let params = Dbrep::parameters();
        assert_eq!(Dbrep::hlr_mode(), params.with_hlr);
    }

    #[test]
    fn test_rg1_mode() {
        let params = Dbrep::parameters();
        assert_eq!(Dbrep::rg1_mode(), params.with_rg1);
    }

    #[test]
    fn test_rgn_mode() {
        let params = Dbrep::parameters();
        assert_eq!(Dbrep::rgn_mode(), params.with_rgn);
    }

    #[test]
    fn test_hid_mode() {
        let params = Dbrep::parameters();
        assert_eq!(Dbrep::hid_mode(), params.with_hid);
    }

    #[test]
    fn test_hlr_angle() {
        let angle = Dbrep::hlr_angle();
        assert!(angle > 0.5 && angle < 0.6);
    }

    #[test]
    fn test_discretisation() {
        let disc = Dbrep::discretisation();
        assert_eq!(disc, 30);
    }

    #[test]
    fn test_set_parameters() {
        let mut params = DbrepParams::default();
        params.nb_isos = 20;
        params.discretization = 50;

        Dbrep::set_parameters(params.clone());

        let retrieved = Dbrep::parameters();
        assert_eq!(retrieved.nb_isos, 20);
        assert_eq!(retrieved.discretization, 50);
    }

    #[test]
    fn test_clear() {
        Dbrep::clear();
        Dbrep::set("shape1", TopodsShape::new(ShapeType::Face, "f1"));
        Dbrep::set("shape2", TopodsShape::new(ShapeType::Edge, "e1"));

        assert!(!Dbrep::get("shape1", ShapeType::Face, false).is_null());
        Dbrep::clear();
        assert!(Dbrep::get("shape1", ShapeType::Face, false).is_null());
    }
}
