// FILE: vrml_material.rs
// occt: Vrml_Material
//
// Faithful port of OCCT Vrml_Material (DataExchange/TKDEVRML/Vrml/
// Vrml_Material.hxx): the VRML 1.0 `Material` node, specifying surface
// properties (ambient color, diffuse color, specular color, emissive color,
// shininess, transparency). Defaults per VRML 1.0 spec. Print emits only
// non-default fields.

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_material_real(v: f64) -> String {
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

/// Local RGB color type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlMaterialColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl VrmlMaterialColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        VrmlMaterialColor { r, g, b }
    }
}

/// Port of Vrml_Material.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlMaterial {
    my_ambient_color: VrmlMaterialColor,
    my_diffuse_color: VrmlMaterialColor,
    my_specular_color: VrmlMaterialColor,
    my_emissive_color: VrmlMaterialColor,
    my_shininess: f64,
    my_transparency: f64,
}

impl VrmlMaterial {
    /// Vrml_Material(): all defaults per VRML 1.0 spec.
    /// Default ambient: (0.2 0.2 0.2)
    /// Default diffuse: (0.8 0.8 0.8)
    /// Default specular: (0 0 0)
    /// Default emissive: (0 0 0)
    /// Default shininess: 0.2
    /// Default transparency: 0
    pub fn new() -> Self {
        VrmlMaterial {
            my_ambient_color: VrmlMaterialColor::new(0.2, 0.2, 0.2),
            my_diffuse_color: VrmlMaterialColor::new(0.8, 0.8, 0.8),
            my_specular_color: VrmlMaterialColor::new(0.0, 0.0, 0.0),
            my_emissive_color: VrmlMaterialColor::new(0.0, 0.0, 0.0),
            my_shininess: 0.2,
            my_transparency: 0.0,
        }
    }

    pub fn set_ambient_color(&mut self, a_color: VrmlMaterialColor) {
        self.my_ambient_color = a_color;
    }

    pub fn ambient_color(&self) -> VrmlMaterialColor {
        self.my_ambient_color
    }

    pub fn set_diffuse_color(&mut self, a_color: VrmlMaterialColor) {
        self.my_diffuse_color = a_color;
    }

    pub fn diffuse_color(&self) -> VrmlMaterialColor {
        self.my_diffuse_color
    }

    pub fn set_specular_color(&mut self, a_color: VrmlMaterialColor) {
        self.my_specular_color = a_color;
    }

    pub fn specular_color(&self) -> VrmlMaterialColor {
        self.my_specular_color
    }

    pub fn set_emissive_color(&mut self, a_color: VrmlMaterialColor) {
        self.my_emissive_color = a_color;
    }

    pub fn emissive_color(&self) -> VrmlMaterialColor {
        self.my_emissive_color
    }

    pub fn set_shininess(&mut self, a_shininess: f64) {
        self.my_shininess = a_shininess;
    }

    pub fn shininess(&self) -> f64 {
        self.my_shininess
    }

    pub fn set_transparency(&mut self, a_transparency: f64) {
        self.my_transparency = a_transparency;
    }

    pub fn transparency(&self) -> f64 {
        self.my_transparency
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("Material {\n");

        // ambientColor (default 0.2 0.2 0.2)
        if (self.my_ambient_color.r - 0.2).abs() > 0.0001
            || (self.my_ambient_color.g - 0.2).abs() > 0.0001
            || (self.my_ambient_color.b - 0.2).abs() > 0.0001
        {
            an_ostream.push_str("    ambientColor\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_material_real(self.my_ambient_color.r),
                vrml_material_real(self.my_ambient_color.g),
                vrml_material_real(self.my_ambient_color.b)
            ));
        }

        // diffuseColor (default 0.8 0.8 0.8)
        if (self.my_diffuse_color.r - 0.8).abs() > 0.0001
            || (self.my_diffuse_color.g - 0.8).abs() > 0.0001
            || (self.my_diffuse_color.b - 0.8).abs() > 0.0001
        {
            an_ostream.push_str("    diffuseColor\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_material_real(self.my_diffuse_color.r),
                vrml_material_real(self.my_diffuse_color.g),
                vrml_material_real(self.my_diffuse_color.b)
            ));
        }

        // specularColor (default 0 0 0)
        if self.my_specular_color.r > 0.0001
            || self.my_specular_color.g > 0.0001
            || self.my_specular_color.b > 0.0001
        {
            an_ostream.push_str("    specularColor\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_material_real(self.my_specular_color.r),
                vrml_material_real(self.my_specular_color.g),
                vrml_material_real(self.my_specular_color.b)
            ));
        }

        // emissiveColor (default 0 0 0)
        if self.my_emissive_color.r > 0.0001
            || self.my_emissive_color.g > 0.0001
            || self.my_emissive_color.b > 0.0001
        {
            an_ostream.push_str("    emissiveColor\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_material_real(self.my_emissive_color.r),
                vrml_material_real(self.my_emissive_color.g),
                vrml_material_real(self.my_emissive_color.b)
            ));
        }

        // shininess (default 0.2)
        if (self.my_shininess - 0.2).abs() > 0.0001 {
            an_ostream.push_str("    shininess\t");
            an_ostream.push_str(&vrml_material_real(self.my_shininess));
            an_ostream.push('\n');
        }

        // transparency (default 0)
        if self.my_transparency > 0.0001 {
            an_ostream.push_str("    transparency\t");
            an_ostream.push_str(&vrml_material_real(self.my_transparency));
            an_ostream.push('\n');
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlMaterial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prints_empty_node() {
        let mat = VrmlMaterial::new();
        let mut out = String::new();
        mat.print(&mut out);
        assert_eq!(out, "Material {\n}\n");
    }

    #[test]
    fn custom_diffuse_color() {
        let mut mat = VrmlMaterial::new();
        mat.set_diffuse_color(VrmlMaterialColor::new(1.0, 0.0, 0.0));
        let mut out = String::new();
        mat.print(&mut out);
        assert_eq!(out, "Material {\n    diffuseColor\t1 0 0\n}\n");
    }

    #[test]
    fn custom_specular_color() {
        let mut mat = VrmlMaterial::new();
        mat.set_specular_color(VrmlMaterialColor::new(0.5, 0.5, 0.5));
        let mut out = String::new();
        mat.print(&mut out);
        assert_eq!(out, "Material {\n    specularColor\t0.5 0.5 0.5\n}\n");
    }

    #[test]
    fn custom_shininess() {
        let mut mat = VrmlMaterial::new();
        mat.set_shininess(0.8);
        let mut out = String::new();
        mat.print(&mut out);
        assert_eq!(out, "Material {\n    shininess\t0.8\n}\n");
    }

    #[test]
    fn custom_transparency() {
        let mut mat = VrmlMaterial::new();
        mat.set_transparency(0.5);
        let mut out = String::new();
        mat.print(&mut out);
        assert_eq!(out, "Material {\n    transparency\t0.5\n}\n");
    }

    #[test]
    fn all_custom_fields() {
        let mut mat = VrmlMaterial::new();
        mat.set_ambient_color(VrmlMaterialColor::new(0.3, 0.3, 0.3));
        mat.set_diffuse_color(VrmlMaterialColor::new(0.6, 0.6, 0.6));
        mat.set_specular_color(VrmlMaterialColor::new(0.2, 0.2, 0.2));
        mat.set_shininess(0.5);
        mat.set_transparency(0.25);
        let mut out = String::new();
        mat.print(&mut out);
        assert!(out.contains("ambientColor"));
        assert!(out.contains("diffuseColor"));
        assert!(out.contains("specularColor"));
        assert!(out.contains("shininess"));
        assert!(out.contains("transparency"));
    }
}
