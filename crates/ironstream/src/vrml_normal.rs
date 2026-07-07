// FILE: vrml_normal.rs
// occt: Vrml_Normal
//
// Faithful port of OCCT Vrml_Normal (DataExchange/TKDEVRML/Vrml/Vrml_Normal.hxx):
// the VRML 1.0 `Normal` node, containing a list of 3D normal vectors.
// Default list is empty. Print emits all vectors in array syntax.

/// Local model of gp_Vec (3D normal vector).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlNormalVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlNormalVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlNormalVec { x, y, z }
    }
}

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_normal_real(v: f64) -> String {
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

/// Port of Vrml_Normal.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlNormal {
    my_vectors: Vec<VrmlNormalVec>,
}

impl VrmlNormal {
    /// Vrml_Normal(): empty vector list.
    pub fn new() -> Self {
        VrmlNormal {
            my_vectors: Vec::new(),
        }
    }

    /// Vrml_Normal(const Handle(TColStd_HArray1OfVec)& aVectors).
    pub fn with_vectors(a_vectors: Vec<VrmlNormalVec>) -> Self {
        VrmlNormal {
            my_vectors: a_vectors,
        }
    }

    pub fn set_vectors(&mut self, a_vectors: Vec<VrmlNormalVec>) {
        self.my_vectors = a_vectors;
    }

    pub fn vectors(&self) -> &[VrmlNormalVec] {
        &self.my_vectors
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("Normal {\n");

        if !self.my_vectors.is_empty() {
            an_ostream.push_str("    vector\t[\n");
            for (i, vec) in self.my_vectors.iter().enumerate() {
                if i > 0 {
                    an_ostream.push('\n');
                }
                an_ostream.push_str("\t    ");
                an_ostream.push_str(&format!(
                    "{} {} {}",
                    vrml_normal_real(vec.x),
                    vrml_normal_real(vec.y),
                    vrml_normal_real(vec.z)
                ));
            }
            an_ostream.push_str("\n    ]\n");
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlNormal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty_vector_list() {
        let norm = VrmlNormal::new();
        assert_eq!(norm.vectors().len(), 0);
        let mut out = String::new();
        norm.print(&mut out);
        assert_eq!(out, "Normal {\n}\n");
    }

    #[test]
    fn single_vector() {
        let norm = VrmlNormal::with_vectors(vec![VrmlNormalVec::new(0.0, 0.0, 1.0)]);
        let mut out = String::new();
        norm.print(&mut out);
        assert_eq!(out, "Normal {\n    vector\t[\n\t    0 0 1\n    ]\n}\n");
    }

    #[test]
    fn multiple_vectors() {
        let norm = VrmlNormal::with_vectors(vec![
            VrmlNormalVec::new(1.0, 0.0, 0.0),
            VrmlNormalVec::new(0.0, 1.0, 0.0),
            VrmlNormalVec::new(0.0, 0.0, 1.0),
        ]);
        let mut out = String::new();
        norm.print(&mut out);
        assert!(out.contains("1 0 0"));
        assert!(out.contains("0 1 0"));
        assert!(out.contains("0 0 1"));
        assert!(out.contains("vector"));
    }

    #[test]
    fn setter() {
        let mut norm = VrmlNormal::new();
        norm.set_vectors(vec![VrmlNormalVec::new(0.707, 0.707, 0.0)]);
        assert_eq!(norm.vectors().len(), 1);
        let mut out = String::new();
        norm.print(&mut out);
        assert!(out.contains("0.707"));
    }
}
