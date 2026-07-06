// FILE: vrml_font_style.rs
// occt: Vrml_FontStyle
//
// Faithful port of OCCT Vrml_FontStyle (DataExchange/TKDEVRML/Vrml/
// Vrml_FontStyle.hxx): the VRML 1.0 `FontStyle` node, specifying font
// appearance properties (family, style, size). Defaults: family=SERIF,
// style=NORMAL, size=1. Print emits only non-default fields.

/// Local enums for FontStyle properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlFontStyleFamily {
    Serif,
    Sans,
    Typewriter,
}

impl VrmlFontStyleFamily {
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlFontStyleFamily::Serif => "SERIF",
            VrmlFontStyleFamily::Sans => "SANS",
            VrmlFontStyleFamily::Typewriter => "TYPEWRITER",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlFontStyleStyle {
    Normal,
    Bold,
    Italic,
    Bolditalic,
}

impl VrmlFontStyleStyle {
    pub fn vrml_keyword(self) -> &'static str {
        match self {
            VrmlFontStyleStyle::Normal => "NORMAL",
            VrmlFontStyleStyle::Bold => "BOLD",
            VrmlFontStyleStyle::Italic => "ITALIC",
            VrmlFontStyleStyle::Bolditalic => "BOLDITALIC",
        }
    }
}

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_font_style_real(v: f64) -> String {
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

/// Port of Vrml_FontStyle.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlFontStyle {
    my_family: VrmlFontStyleFamily,
    my_style: VrmlFontStyleStyle,
    my_size: f64,
}

impl VrmlFontStyle {
    /// Vrml_FontStyle(): family=SERIF, style=NORMAL, size=1.
    pub fn new() -> Self {
        VrmlFontStyle {
            my_family: VrmlFontStyleFamily::Serif,
            my_style: VrmlFontStyleStyle::Normal,
            my_size: 1.0,
        }
    }

    /// Constructor with explicit parameters.
    pub fn with_properties(
        a_family: VrmlFontStyleFamily,
        a_style: VrmlFontStyleStyle,
        a_size: f64,
    ) -> Self {
        VrmlFontStyle {
            my_family: a_family,
            my_style: a_style,
            my_size: a_size,
        }
    }

    pub fn set_family(&mut self, a_family: VrmlFontStyleFamily) {
        self.my_family = a_family;
    }

    pub fn family(&self) -> VrmlFontStyleFamily {
        self.my_family
    }

    pub fn set_style(&mut self, a_style: VrmlFontStyleStyle) {
        self.my_style = a_style;
    }

    pub fn style(&self) -> VrmlFontStyleStyle {
        self.my_style
    }

    pub fn set_size(&mut self, a_size: f64) {
        self.my_size = a_size;
    }

    pub fn size(&self) -> f64 {
        self.my_size
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("FontStyle {\n");

        // family field (print only if not SERIF)
        if self.my_family != VrmlFontStyleFamily::Serif {
            an_ostream.push_str("    family\t");
            an_ostream.push_str(self.my_family.vrml_keyword());
            an_ostream.push('\n');
        }

        // style field (print only if not NORMAL)
        if self.my_style != VrmlFontStyleStyle::Normal {
            an_ostream.push_str("    style\t");
            an_ostream.push_str(self.my_style.vrml_keyword());
            an_ostream.push('\n');
        }

        // size field (print only if not 1.0, with tolerance)
        if (self.my_size - 1.0).abs() > 0.0001 {
            an_ostream.push_str("    size\t");
            an_ostream.push_str(&vrml_font_style_real(self.my_size));
            an_ostream.push('\n');
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlFontStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prints_empty_node() {
        let fs = VrmlFontStyle::new();
        assert_eq!(fs.family(), VrmlFontStyleFamily::Serif);
        assert_eq!(fs.style(), VrmlFontStyleStyle::Normal);
        assert_eq!(fs.size(), 1.0);
        let mut out = String::new();
        fs.print(&mut out);
        assert_eq!(out, "FontStyle {\n}\n");
    }

    #[test]
    fn family_sans_prints_field() {
        let fs = VrmlFontStyle::with_properties(
            VrmlFontStyleFamily::Sans,
            VrmlFontStyleStyle::Normal,
            1.0,
        );
        let mut out = String::new();
        fs.print(&mut out);
        assert_eq!(out, "FontStyle {\n    family\tSANS\n}\n");
    }

    #[test]
    fn style_bold_prints_field() {
        let fs = VrmlFontStyle::with_properties(
            VrmlFontStyleFamily::Serif,
            VrmlFontStyleStyle::Bold,
            1.0,
        );
        let mut out = String::new();
        fs.print(&mut out);
        assert_eq!(out, "FontStyle {\n    style\tBOLD\n}\n");
    }

    #[test]
    fn size_non_default_prints_field() {
        let fs = VrmlFontStyle::with_properties(
            VrmlFontStyleFamily::Serif,
            VrmlFontStyleStyle::Normal,
            2.5,
        );
        let mut out = String::new();
        fs.print(&mut out);
        assert_eq!(out, "FontStyle {\n    size\t2.5\n}\n");
    }

    #[test]
    fn all_fields_custom() {
        let fs = VrmlFontStyle::with_properties(
            VrmlFontStyleFamily::Typewriter,
            VrmlFontStyleStyle::Italic,
            0.8,
        );
        let mut out = String::new();
        fs.print(&mut out);
        assert_eq!(
            out,
            "FontStyle {\n    family\tTYPEWRITER\n    style\tITALIC\n    size\t0.8\n}\n"
        );
    }

    #[test]
    fn size_tolerance() {
        let fs = VrmlFontStyle::with_properties(
            VrmlFontStyleFamily::Serif,
            VrmlFontStyleStyle::Normal,
            1.00005, // within 0.0001 of 1 -> suppressed
        );
        let mut out = String::new();
        fs.print(&mut out);
        assert_eq!(out, "FontStyle {\n}\n");
    }

    #[test]
    fn setters() {
        let mut fs = VrmlFontStyle::new();
        fs.set_family(VrmlFontStyleFamily::Sans);
        fs.set_style(VrmlFontStyleStyle::Bold);
        fs.set_size(1.5);
        assert_eq!(fs.family(), VrmlFontStyleFamily::Sans);
        assert_eq!(fs.style(), VrmlFontStyleStyle::Bold);
        assert_eq!(fs.size(), 1.5);
    }
}
