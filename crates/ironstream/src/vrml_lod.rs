// FILE: vrml_lod.rs
// occt: Vrml_LOD
//
// Faithful port of OCCT Vrml_LOD (DataExchange/TKDEVRML/Vrml/Vrml_LOD.hxx):
// the VRML 1.0 `LOD` (Level of Detail) node. Contains a range (center point),
// and a list of LOD threshold distances. Default center is (0 0 0), default
// empty LOD range list. Print emits the range and all thresholds.

/// Local model of gp_Pnt (center point).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlLodPnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlLodPnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlLodPnt { x, y, z }
    }
}

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_lod_real(v: f64) -> String {
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

/// Port of Vrml_LOD.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlLod {
    my_range: VrmlLodPnt,
    my_list: Vec<f64>,
}

impl VrmlLod {
    /// Vrml_LOD(): center defaults to (0 0 0), LOD list is empty.
    pub fn new() -> Self {
        VrmlLod {
            my_range: VrmlLodPnt::new(0.0, 0.0, 0.0),
            my_list: Vec::new(),
        }
    }

    /// Vrml_LOD(const gp_Pnt& aRange, const Handle(TColStd_HArray1OfReal)& aList).
    pub fn with_range_and_list(a_range: VrmlLodPnt, a_list: Vec<f64>) -> Self {
        VrmlLod {
            my_range: a_range,
            my_list: a_list,
        }
    }

    pub fn set_range(&mut self, a_range: VrmlLodPnt) {
        self.my_range = a_range;
    }

    pub fn range(&self) -> VrmlLodPnt {
        self.my_range
    }

    pub fn set_list(&mut self, a_list: Vec<f64>) {
        self.my_list = a_list;
    }

    pub fn list(&self) -> &[f64] {
        &self.my_list
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("LOD {\n");

        // Always write range field
        an_ostream.push_str("    range\t");
        an_ostream.push_str(&format!(
            "{} {} {}\n",
            vrml_lod_real(self.my_range.x),
            vrml_lod_real(self.my_range.y),
            vrml_lod_real(self.my_range.z)
        ));

        // Write LOD thresholds
        if !self.my_list.is_empty() {
            an_ostream.push_str("    levels\t[\n");
            for (i, val) in self.my_list.iter().enumerate() {
                if i > 0 {
                    an_ostream.push('\n');
                }
                an_ostream.push_str("\t    ");
                an_ostream.push_str(&vrml_lod_real(*val));
            }
            an_ostream.push_str("\n    ]\n");
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlLod {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prints_origin() {
        let lod = VrmlLod::new();
        let mut out = String::new();
        lod.print(&mut out);
        assert_eq!(out, "LOD {\n    range\t0 0 0\n}\n");
    }

    #[test]
    fn with_range_and_empty_list() {
        let lod = VrmlLod::with_range_and_list(VrmlLodPnt::new(1.0, 2.0, 3.0), vec![]);
        let mut out = String::new();
        lod.print(&mut out);
        assert_eq!(out, "LOD {\n    range\t1 2 3\n}\n");
    }

    #[test]
    fn with_range_and_levels() {
        let lod = VrmlLod::with_range_and_list(
            VrmlLodPnt::new(0.0, 0.0, 0.0),
            vec![10.0, 50.0, 100.0],
        );
        let mut out = String::new();
        lod.print(&mut out);
        assert_eq!(
            out,
            "LOD {\n    range\t0 0 0\n    levels\t[\n\t    10\n\t    50\n\t    100\n    ]\n}\n"
        );
    }

    #[test]
    fn setter_updates_range_and_list() {
        let mut lod = VrmlLod::new();
        lod.set_range(VrmlLodPnt::new(5.0, 5.0, 5.0));
        lod.set_list(vec![25.0]);
        let mut out = String::new();
        lod.print(&mut out);
        assert_eq!(out, "LOD {\n    range\t5 5 5\n    levels\t[\n\t    25\n    ]\n}\n");
    }
}
