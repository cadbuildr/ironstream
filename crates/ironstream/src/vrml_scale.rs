// FILE: vrml_scale.rs
// occt: Vrml_Scale
//
// Faithful port of OCCT Vrml_Scale (DataExchange/TKDEVRML/Vrml/Vrml_Scale.hxx/.cxx):
// a VRML 1.0 `Scale` transform-property node (3D scaling about the origin).
// Default scaleFactor is (1 1 1); Print emits the field only when a component
// differs from 1 by more than 0.0001.

/// Local model of gp_Vec used as the scaleFactor field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlScaleVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlScaleVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlScaleVec { x, y, z }
    }
}

/// Emulates C++ `operator<<(std::ostream&, double)` with the default stream
/// state (precision 6, defaultfloat) — i.e. printf "%g".
fn vrml_scale_real(v: f64) -> String {
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

/// Port of Vrml_Scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlScale {
    my_scale_factor: VrmlScaleVec,
}

impl VrmlScale {
    /// Vrml_Scale(): scaleFactor defaults to (1 1 1).
    pub fn new() -> Self {
        VrmlScale {
            my_scale_factor: VrmlScaleVec::new(1.0, 1.0, 1.0),
        }
    }

    /// Vrml_Scale(const gp_Vec& aScaleFactor).
    pub fn with_scale_factor(a_scale_factor: VrmlScaleVec) -> Self {
        VrmlScale {
            my_scale_factor: a_scale_factor,
        }
    }

    pub fn set_scale_factor(&mut self, a_scale_factor: VrmlScaleVec) {
        self.my_scale_factor = a_scale_factor;
    }

    pub fn scale_factor(&self) -> VrmlScaleVec {
        self.my_scale_factor
    }

    /// Standard_OStream& Print(Standard_OStream&): writes the VRML 1.0
    /// `Scale { ... }` node; the scaleFactor line is emitted only when a
    /// component differs from 1 by more than 0.0001.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("Scale {\n");

        if (self.my_scale_factor.x - 1.0).abs() > 0.0001
            || (self.my_scale_factor.y - 1.0).abs() > 0.0001
            || (self.my_scale_factor.z - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    scaleFactor\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_scale_real(self.my_scale_factor.x),
                vrml_scale_real(self.my_scale_factor.y),
                vrml_scale_real(self.my_scale_factor.z)
            ));
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlScale {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prints_empty_node() {
        let s = VrmlScale::new();
        assert_eq!(s.scale_factor(), VrmlScaleVec::new(1.0, 1.0, 1.0));
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Scale {\n}\n");
    }

    #[test]
    fn non_default_prints_scale_factor_field() {
        let s = VrmlScale::with_scale_factor(VrmlScaleVec::new(2.0, 0.5, 1.0));
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Scale {\n    scaleFactor\t2 0.5 1\n}\n");
    }

    #[test]
    fn set_scale_factor_and_tolerance() {
        let mut s = VrmlScale::new();
        // within 0.0001 of 1 on every component -> field suppressed
        s.set_scale_factor(VrmlScaleVec::new(1.00005, 1.0, 0.99995));
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Scale {\n}\n");

        s.set_scale_factor(VrmlScaleVec::new(1.5, 1.0, 1.0));
        let mut out2 = String::new();
        s.print(&mut out2);
        assert_eq!(out2, "Scale {\n    scaleFactor\t1.5 1 1\n}\n");
    }

    #[test]
    fn real_formatting_matches_cpp_defaultfloat() {
        assert_eq!(vrml_scale_real(0.0), "0");
        assert_eq!(vrml_scale_real(1.0), "1");
        assert_eq!(vrml_scale_real(0.5), "0.5");
        assert_eq!(vrml_scale_real(-2.25), "-2.25");
        assert_eq!(vrml_scale_real(1234567.0), "1.23457e+06");
        assert_eq!(vrml_scale_real(0.0001), "0.0001");
        assert_eq!(vrml_scale_real(0.00001), "1e-05");
    }
}
