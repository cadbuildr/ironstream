// FILE: step_shape_csg_primitive.rs
// occt: StepShape_CsgPrimitive

//! Representation of STEP CSG Primitive SelectType

#[derive(Clone, Debug)]
pub enum CsgPrimitive {
    /// Sphere
    Sphere(String),
    /// Block
    Block(String),
    /// RightAngularWedge
    RightAngularWedge(String),
    /// Torus
    Torus(String),
    /// RightCircularCone
    RightCircularCone(String),
    /// RightCircularCylinder
    RightCircularCylinder(String),
}

impl CsgPrimitive {
    /// Returns a CsgPrimitive SelectType
    pub fn new() -> Option<Self> {
        None
    }

    /// Recognizes a CsgPrimitive Kind Entity that is:
    /// 1 -> Sphere
    /// 2 -> Block
    /// 3 -> RightAngularWedge
    /// 4 -> Torus
    /// 5 -> RightCircularCone
    /// 6 -> RightCircularCylinder
    /// 0 else
    pub fn case_num(entity_type: &str) -> i32 {
        match entity_type {
            "Sphere" => 1,
            "Block" => 2,
            "RightAngularWedge" => 3,
            "Torus" => 4,
            "RightCircularCone" => 5,
            "RightCircularCylinder" => 6,
            _ => 0,
        }
    }

    /// Returns value as Sphere (None if another type)
    pub fn sphere(&self) -> Option<&str> {
        if let CsgPrimitive::Sphere(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Returns value as Block (None if another type)
    pub fn block(&self) -> Option<&str> {
        if let CsgPrimitive::Block(b) = self {
            Some(b)
        } else {
            None
        }
    }

    /// Returns value as RightAngularWedge (None if another type)
    pub fn right_angular_wedge(&self) -> Option<&str> {
        if let CsgPrimitive::RightAngularWedge(w) = self {
            Some(w)
        } else {
            None
        }
    }

    /// Returns value as Torus (None if another type)
    pub fn torus(&self) -> Option<&str> {
        if let CsgPrimitive::Torus(t) = self {
            Some(t)
        } else {
            None
        }
    }

    /// Returns value as RightCircularCone (None if another type)
    pub fn right_circular_cone(&self) -> Option<&str> {
        if let CsgPrimitive::RightCircularCone(c) = self {
            Some(c)
        } else {
            None
        }
    }

    /// Returns value as RightCircularCylinder (None if another type)
    pub fn right_circular_cylinder(&self) -> Option<&str> {
        if let CsgPrimitive::RightCircularCylinder(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

impl Default for CsgPrimitive {
    fn default() -> Self {
        CsgPrimitive::Sphere(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        assert_eq!(CsgPrimitive::case_num("Sphere"), 1);
        assert_eq!(CsgPrimitive::case_num("Block"), 2);
        assert_eq!(CsgPrimitive::case_num("RightAngularWedge"), 3);
        assert_eq!(CsgPrimitive::case_num("Torus"), 4);
        assert_eq!(CsgPrimitive::case_num("RightCircularCone"), 5);
        assert_eq!(CsgPrimitive::case_num("RightCircularCylinder"), 6);
        assert_eq!(CsgPrimitive::case_num("Unknown"), 0);
    }

    #[test]
    fn test_sphere() {
        let prim = CsgPrimitive::Sphere("sphere1".to_string());
        assert_eq!(prim.sphere(), Some("sphere1"));
        assert!(prim.block().is_none());
    }

    #[test]
    fn test_block() {
        let prim = CsgPrimitive::Block("block1".to_string());
        assert_eq!(prim.block(), Some("block1"));
        assert!(prim.sphere().is_none());
    }

    #[test]
    fn test_torus() {
        let prim = CsgPrimitive::Torus("torus1".to_string());
        assert_eq!(prim.torus(), Some("torus1"));
        assert!(prim.right_circular_cone().is_none());
    }
}
