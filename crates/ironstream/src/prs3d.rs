// FILE: prs3d.rs

// occt: Aspect_TypeOfLine
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prs3dLineType {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
}

// occt: Quantity_Color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Prs3dColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Prs3dColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn with_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
}

// occt: Prs3d_LineAspect
pub struct Prs3dLineAspect {
    pub color: Prs3dColor,
    pub line_type: Prs3dLineType,
    pub width: f64,
}

impl Prs3dLineAspect {
    pub fn new(color: Prs3dColor, line_type: Prs3dLineType, width: f64) -> Self {
        Self { color, line_type, width }
    }

    pub fn set_color(&mut self, c: Prs3dColor) {
        self.color = c;
    }

    pub fn set_width(&mut self, w: f64) {
        self.width = w;
    }

    pub fn set_type(&mut self, t: Prs3dLineType) {
        self.line_type = t;
    }
}

// occt: Prs3d_ShadingAspect
pub struct Prs3dShadingAspect {
    pub color: Prs3dColor,
    pub transparency: f64,
    pub shininess: f64,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
}

impl Prs3dShadingAspect {
    pub fn new() -> Self {
        Self {
            color: Prs3dColor::WHITE,
            transparency: 0.0,
            shininess: 0.3,
            ambient: 0.3,
            diffuse: 0.65,
            specular: 0.05,
        }
    }

    pub fn color(&self) -> Prs3dColor {
        self.color
    }

    pub fn set_color(&mut self, c: Prs3dColor) {
        self.color = c;
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn set_transparency(&mut self, t: f64) {
        self.transparency = t;
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_shininess(&mut self, s: f64) {
        self.shininess = s;
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn set_ambient(&mut self, a: f64) {
        self.ambient = a;
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, d: f64) {
        self.diffuse = d;
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn set_specular(&mut self, s: f64) {
        self.specular = s;
    }
}

impl Default for Prs3dShadingAspect {
    fn default() -> Self {
        Self::new()
    }
}

// occt: Prs3d_Drawer
pub struct Prs3dDrawer {
    pub line_aspect: Prs3dLineAspect,
    pub shading_aspect: Prs3dShadingAspect,
    pub nb_iso_u: u32,
    pub nb_iso_v: u32,
    pub deviation_coefficient: f64,
    pub max_param_value: f64,
}

impl Prs3dDrawer {
    pub fn new() -> Self {
        Self {
            line_aspect: Prs3dLineAspect::new(Prs3dColor::WHITE, Prs3dLineType::Solid, 1.0),
            shading_aspect: Prs3dShadingAspect::new(),
            nb_iso_u: 1,
            nb_iso_v: 1,
            deviation_coefficient: 0.001,
            max_param_value: 500_000.0,
        }
    }

    pub fn line_aspect(&self) -> &Prs3dLineAspect {
        &self.line_aspect
    }

    pub fn set_line_aspect(&mut self, la: Prs3dLineAspect) {
        self.line_aspect = la;
    }

    pub fn shading_aspect(&self) -> &Prs3dShadingAspect {
        &self.shading_aspect
    }

    pub fn set_shading_aspect(&mut self, sa: Prs3dShadingAspect) {
        self.shading_aspect = sa;
    }

    pub fn nb_iso_u(&self) -> u32 {
        self.nb_iso_u
    }

    pub fn set_nb_iso_u(&mut self, n: u32) {
        self.nb_iso_u = n;
    }

    pub fn nb_iso_v(&self) -> u32 {
        self.nb_iso_v
    }

    pub fn set_nb_iso_v(&mut self, n: u32) {
        self.nb_iso_v = n;
    }

    pub fn deviation_coefficient(&self) -> f64 {
        self.deviation_coefficient
    }

    pub fn set_deviation_coefficient(&mut self, d: f64) {
        self.deviation_coefficient = d;
    }

    pub fn max_param_value(&self) -> f64 {
        self.max_param_value
    }

    pub fn set_max_param_value(&mut self, v: f64) {
        self.max_param_value = v;
    }
}

impl Default for Prs3dDrawer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_new_has_full_alpha() {
        let c = Prs3dColor::new(0.5, 0.25, 0.75);
        assert_eq!(c.r, 0.5);
        assert_eq!(c.g, 0.25);
        assert_eq!(c.b, 0.75);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn color_with_alpha_stores_alpha() {
        let c = Prs3dColor::with_alpha(0.1, 0.2, 0.3, 0.5);
        assert_eq!(c.a, 0.5);
    }

    #[test]
    fn predefined_colors_correct() {
        assert_eq!(Prs3dColor::WHITE, Prs3dColor::new(1.0, 1.0, 1.0));
        assert_eq!(Prs3dColor::BLACK, Prs3dColor::new(0.0, 0.0, 0.0));
        assert_eq!(Prs3dColor::RED, Prs3dColor::new(1.0, 0.0, 0.0));
        assert_eq!(Prs3dColor::GREEN, Prs3dColor::new(0.0, 1.0, 0.0));
        assert_eq!(Prs3dColor::BLUE, Prs3dColor::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn line_type_variants_distinct() {
        let types = [
            Prs3dLineType::Solid,
            Prs3dLineType::Dash,
            Prs3dLineType::Dot,
            Prs3dLineType::DashDot,
            Prs3dLineType::DashDotDot,
        ];
        for i in 0..types.len() {
            for j in 0..types.len() {
                if i == j {
                    assert_eq!(types[i], types[j]);
                } else {
                    assert_ne!(types[i], types[j]);
                }
            }
        }
    }

    #[test]
    fn line_aspect_new_stores_fields() {
        let la = Prs3dLineAspect::new(Prs3dColor::RED, Prs3dLineType::Dash, 2.5);
        assert_eq!(la.color, Prs3dColor::RED);
        assert_eq!(la.line_type, Prs3dLineType::Dash);
        assert_eq!(la.width, 2.5);
    }

    #[test]
    fn line_aspect_setters_mutate() {
        let mut la = Prs3dLineAspect::new(Prs3dColor::WHITE, Prs3dLineType::Solid, 1.0);
        la.set_color(Prs3dColor::BLUE);
        la.set_width(3.0);
        la.set_type(Prs3dLineType::DashDot);
        assert_eq!(la.color, Prs3dColor::BLUE);
        assert_eq!(la.width, 3.0);
        assert_eq!(la.line_type, Prs3dLineType::DashDot);
    }

    #[test]
    fn shading_aspect_defaults() {
        let sa = Prs3dShadingAspect::new();
        assert_eq!(sa.color(), Prs3dColor::WHITE);
        assert_eq!(sa.transparency(), 0.0);
        assert_eq!(sa.shininess(), 0.3);
    }

    #[test]
    fn shading_aspect_setters() {
        let mut sa = Prs3dShadingAspect::new();
        sa.set_color(Prs3dColor::RED);
        sa.set_transparency(0.5);
        sa.set_shininess(0.8);
        sa.set_ambient(0.4);
        sa.set_diffuse(0.5);
        sa.set_specular(0.1);
        assert_eq!(sa.color(), Prs3dColor::RED);
        assert_eq!(sa.transparency(), 0.5);
        assert_eq!(sa.shininess(), 0.8);
        assert_eq!(sa.ambient(), 0.4);
        assert_eq!(sa.diffuse(), 0.5);
        assert_eq!(sa.specular(), 0.1);
    }

    #[test]
    fn drawer_new_sensible_defaults() {
        let d = Prs3dDrawer::new();
        assert_eq!(d.nb_iso_u(), 1);
        assert_eq!(d.nb_iso_v(), 1);
        assert!(d.deviation_coefficient() > 0.0);
        assert!(d.max_param_value() > 0.0);
    }

    #[test]
    fn drawer_setters() {
        let mut d = Prs3dDrawer::new();
        d.set_nb_iso_u(4);
        d.set_nb_iso_v(6);
        d.set_deviation_coefficient(0.0001);
        d.set_max_param_value(1_000_000.0);
        assert_eq!(d.nb_iso_u(), 4);
        assert_eq!(d.nb_iso_v(), 6);
        assert_eq!(d.deviation_coefficient(), 0.0001);
        assert_eq!(d.max_param_value(), 1_000_000.0);
    }

    #[test]
    fn drawer_default_trait() {
        let d: Prs3dDrawer = Default::default();
        assert_eq!(d.nb_iso_u(), 1);
    }

    #[test]
    fn line_type_copy() {
        let t = Prs3dLineType::DashDotDot;
        let t2 = t;
        assert_eq!(t, t2);
    }

    #[test]
    fn color_copy() {
        let c = Prs3dColor::new(0.3, 0.4, 0.5);
        let c2 = c;
        assert_eq!(c, c2);
    }
}
