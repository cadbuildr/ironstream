// FILE: vrml_shape_hints.rs
// occt: Vrml_ShapeHints
//
// Faithful port of OCCT Vrml_ShapeHints (DataExchange/TKDEVRML/Vrml/
// Vrml_ShapeHints.hxx/.cxx): the VRML 1.0 `ShapeHints` property node.
// Defaults: vertexOrdering UNKNOWN_ORDERING, shapeType UNKNOWN_SHAPE_TYPE,
// faceType CONVEX, creaseAngle 0.5. Print suppresses fields at their
// written defaults (UNKNOWN_ORDERING, UNKNOWN_SHAPE_TYPE, CONVEX, and a
// creaseAngle within 0.0001 of 0.5).

/// Local model of the Vrml_VertexOrdering enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeHintsVertexOrdering {
    UnknownOrdering,
    Clockwise,
    Counterclockwise,
}

/// Local model of the Vrml_ShapeType enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeHintsShapeType {
    UnknownShapeType,
    Solid,
}

/// Local model of the Vrml_FaceType enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeHintsFaceType {
    UnknownFaceType,
    Convex,
}

/// Emulates C++ default-ostream double output (printf "%g", 6 significant digits).
fn shape_hints_real(v: f64) -> String {
    let p = 6usize;
    let sci = format!("{:.*e}", p - 1, v);
    let epos = sci.find('e').expect("exponent");
    let exp: i32 = sci[epos + 1..].parse().expect("exp digits");
    if exp < -4 || exp >= p as i32 {
        let mant = sci[..epos].trim_end_matches('0').trim_end_matches('.');
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else {
        let prec = (p as i32 - 1 - exp).max(0) as usize;
        let fixed = format!("{:.*}", prec, v);
        if fixed.contains('.') {
            fixed
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            fixed
        }
    }
}

/// Port of Vrml_ShapeHints.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlShapeHints {
    my_vertex_ordering: ShapeHintsVertexOrdering,
    my_shape_type: ShapeHintsShapeType,
    my_face_type: ShapeHintsFaceType,
    my_angle: f64,
}

impl VrmlShapeHints {
    /// Vrml_ShapeHints with all default arguments
    /// (UNKNOWN_ORDERING, UNKNOWN_SHAPE_TYPE, CONVEX, 0.5).
    pub fn new() -> Self {
        Self::with_fields(
            ShapeHintsVertexOrdering::UnknownOrdering,
            ShapeHintsShapeType::UnknownShapeType,
            ShapeHintsFaceType::Convex,
            0.5,
        )
    }

    /// Vrml_ShapeHints(aVertexOrdering, aShapeType, aFaceType, aAngle).
    pub fn with_fields(
        a_vertex_ordering: ShapeHintsVertexOrdering,
        a_shape_type: ShapeHintsShapeType,
        a_face_type: ShapeHintsFaceType,
        a_angle: f64,
    ) -> Self {
        VrmlShapeHints {
            my_vertex_ordering: a_vertex_ordering,
            my_shape_type: a_shape_type,
            my_face_type: a_face_type,
            my_angle: a_angle,
        }
    }

    pub fn set_vertex_ordering(&mut self, a_vertex_ordering: ShapeHintsVertexOrdering) {
        self.my_vertex_ordering = a_vertex_ordering;
    }

    pub fn vertex_ordering(&self) -> ShapeHintsVertexOrdering {
        self.my_vertex_ordering
    }

    pub fn set_shape_type(&mut self, a_shape_type: ShapeHintsShapeType) {
        self.my_shape_type = a_shape_type;
    }

    pub fn shape_type(&self) -> ShapeHintsShapeType {
        self.my_shape_type
    }

    pub fn set_face_type(&mut self, a_face_type: ShapeHintsFaceType) {
        self.my_face_type = a_face_type;
    }

    pub fn face_type(&self) -> ShapeHintsFaceType {
        self.my_face_type
    }

    pub fn set_angle(&mut self, a_angle: f64) {
        self.my_angle = a_angle;
    }

    pub fn angle(&self) -> f64 {
        self.my_angle
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("ShapeHints {\n");

        match self.my_vertex_ordering {
            ShapeHintsVertexOrdering::UnknownOrdering => {}
            ShapeHintsVertexOrdering::Clockwise => {
                an_ostream.push_str("    vertexOrdering\tCLOCKWISE\n");
            }
            ShapeHintsVertexOrdering::Counterclockwise => {
                an_ostream.push_str("    vertexOrdering\tCOUNTERCLOCKWISE\n");
            }
        }

        match self.my_shape_type {
            ShapeHintsShapeType::UnknownShapeType => {}
            ShapeHintsShapeType::Solid => {
                an_ostream.push_str("    shapeType\t\tSOLID\n");
            }
        }

        match self.my_face_type {
            ShapeHintsFaceType::UnknownFaceType => {
                an_ostream.push_str("    faceType\t\tUNKNOWN_FACE_TYPE\n");
            }
            ShapeHintsFaceType::Convex => {}
        }

        if (self.my_angle - 0.5).abs() > 0.0001 {
            an_ostream.push_str(&format!(
                "    creaseAngle\t\t{}\n",
                shape_hints_real(self.my_angle)
            ));
        }
        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlShapeHints {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prints_empty_node() {
        let sh = VrmlShapeHints::new();
        assert_eq!(sh.angle(), 0.5);
        let mut out = String::new();
        sh.print(&mut out);
        assert_eq!(out, "ShapeHints {\n}\n");
    }

    #[test]
    fn all_non_default_fields() {
        let sh = VrmlShapeHints::with_fields(
            ShapeHintsVertexOrdering::Counterclockwise,
            ShapeHintsShapeType::Solid,
            ShapeHintsFaceType::UnknownFaceType,
            0.9,
        );
        let mut out = String::new();
        sh.print(&mut out);
        assert_eq!(
            out,
            "ShapeHints {\n    vertexOrdering\tCOUNTERCLOCKWISE\n    shapeType\t\tSOLID\n    faceType\t\tUNKNOWN_FACE_TYPE\n    creaseAngle\t\t0.9\n}\n"
        );
    }

    #[test]
    fn clockwise_and_angle_tolerance() {
        let mut sh = VrmlShapeHints::new();
        sh.set_vertex_ordering(ShapeHintsVertexOrdering::Clockwise);
        sh.set_angle(0.50005); // within 0.0001 of default -> suppressed
        let mut out = String::new();
        sh.print(&mut out);
        assert_eq!(out, "ShapeHints {\n    vertexOrdering\tCLOCKWISE\n}\n");
    }

    #[test]
    fn setters_and_getters() {
        let mut sh = VrmlShapeHints::new();
        sh.set_shape_type(ShapeHintsShapeType::Solid);
        sh.set_face_type(ShapeHintsFaceType::UnknownFaceType);
        sh.set_angle(1.25);
        assert_eq!(sh.shape_type(), ShapeHintsShapeType::Solid);
        assert_eq!(sh.face_type(), ShapeHintsFaceType::UnknownFaceType);
        assert_eq!(sh.angle(), 1.25);
        let mut out = String::new();
        sh.print(&mut out);
        assert!(out.contains("    creaseAngle\t\t1.25\n"));
    }
}
