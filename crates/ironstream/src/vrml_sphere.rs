// FILE: vrml_sphere.rs
// occt: Vrml_Sphere
//
// Faithful port of OCCT Vrml_Sphere (DataExchange/TKDEVRML/Vrml/
// Vrml_Sphere.hxx/.cxx): the VRML 1.0 `Sphere` geometry node.
// Default radius is 1; Print emits the radius field only when it differs
// from 1 by more than 0.0001.

/// Emulates C++ default-ostream double output (printf "%g", 6 significant digits).
fn vrml_sphere_real(v: f64) -> String {
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

/// Port of Vrml_Sphere.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlSphere {
    my_radius: f64,
}

impl VrmlSphere {
    /// Vrml_Sphere(const double aRadius = 1).
    pub fn new(a_radius: f64) -> Self {
        VrmlSphere { my_radius: a_radius }
    }

    pub fn set_radius(&mut self, a_radius: f64) {
        self.my_radius = a_radius;
    }

    pub fn radius(&self) -> f64 {
        self.my_radius
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("Sphere {\n");

        if (self.my_radius - 1.0).abs() > 0.0001 {
            an_ostream.push_str("    radius\t");
            an_ostream.push_str(&vrml_sphere_real(self.my_radius));
            an_ostream.push('\n');
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlSphere {
    fn default() -> Self {
        Self::new(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_radius_prints_empty_node() {
        let s = VrmlSphere::default();
        assert_eq!(s.radius(), 1.0);
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Sphere {\n}\n");
    }

    #[test]
    fn custom_radius_prints_field() {
        let s = VrmlSphere::new(2.5);
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Sphere {\n    radius\t2.5\n}\n");
    }

    #[test]
    fn radius_tolerance_and_setter() {
        let mut s = VrmlSphere::new(1.00005); // within 0.0001 of 1 -> suppressed
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Sphere {\n}\n");

        s.set_radius(10.0);
        assert_eq!(s.radius(), 10.0);
        let mut out2 = String::new();
        s.print(&mut out2);
        assert_eq!(out2, "Sphere {\n    radius\t10\n}\n");
    }
}
