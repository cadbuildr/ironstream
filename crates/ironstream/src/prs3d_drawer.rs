// FILE: prs3d_drawer.rs

// occt: Prs3d_TypeOfHLR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prs3dTypeOfHLR {
    NotSet,
    PolyAlgo,
    Algo,
}

// occt-ref: Prs3d_LineAspect
#[derive(Debug, Clone, PartialEq)]
pub struct Prs3dLineAspect {
    color: [f32; 4],
    line_width: f32,
    line_type: u8,
}

impl Prs3dLineAspect {
    pub fn new(color: [f32; 4], width: f32, line_type: u8) -> Self {
        Self { color, line_width: width, line_type }
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn width(&self) -> f32 {
        self.line_width
    }

    pub fn set_width(&mut self, width: f32) {
        self.line_width = width;
    }

    pub fn line_type(&self) -> u8 {
        self.line_type
    }

    pub fn set_line_type(&mut self, line_type: u8) {
        self.line_type = line_type;
    }
}

// occt-ref: Prs3d_ShadingAspect
#[derive(Debug, Clone, PartialEq)]
pub struct Prs3dShadingAspect {
    color: [f32; 4],
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    transparency: f32,
}

impl Prs3dShadingAspect {
    pub fn new() -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0],
            ambient: 0.1,
            diffuse: 0.8,
            specular: 0.5,
            shininess: 64.0,
            transparency: 0.0,
        }
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn ambient(&self) -> f32 {
        self.ambient
    }

    pub fn set_ambient(&mut self, ambient: f32) {
        self.ambient = ambient;
    }

    pub fn diffuse(&self) -> f32 {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: f32) {
        self.diffuse = diffuse;
    }

    pub fn specular(&self) -> f32 {
        self.specular
    }

    pub fn set_specular(&mut self, specular: f32) {
        self.specular = specular;
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    pub fn set_shininess(&mut self, shininess: f32) {
        self.shininess = shininess;
    }

    pub fn transparency(&self) -> f32 {
        self.transparency
    }

    pub fn set_transparency(&mut self, transparency: f32) {
        self.transparency = transparency;
    }
}

impl Default for Prs3dShadingAspect {
    fn default() -> Self {
        Self::new()
    }
}

// occt: Prs3d_Drawer
#[derive(Debug, Clone)]
pub struct Prs3dDrawer {
    wire_aspect: Prs3dLineAspect,
    free_wire_aspect: Prs3dLineAspect,
    shading_aspect: Prs3dShadingAspect,
    hlr_type: Prs3dTypeOfHLR,
    nb_points: u32,
    max_flyflyangle: f64,
}

impl Prs3dDrawer {
    pub fn new() -> Self {
        Self {
            wire_aspect: Prs3dLineAspect::new([1.0, 1.0, 0.0, 1.0], 1.0, 0),
            free_wire_aspect: Prs3dLineAspect::new([0.0, 1.0, 0.0, 1.0], 1.0, 0),
            shading_aspect: Prs3dShadingAspect::new(),
            hlr_type: Prs3dTypeOfHLR::NotSet,
            nb_points: 30,
            max_flyflyangle: 20.0,
        }
    }

    pub fn wire_aspect(&self) -> &Prs3dLineAspect {
        &self.wire_aspect
    }

    pub fn wire_aspect_mut(&mut self) -> &mut Prs3dLineAspect {
        &mut self.wire_aspect
    }

    pub fn free_wire_aspect(&self) -> &Prs3dLineAspect {
        &self.free_wire_aspect
    }

    pub fn free_wire_aspect_mut(&mut self) -> &mut Prs3dLineAspect {
        &mut self.free_wire_aspect
    }

    pub fn shading_aspect(&self) -> &Prs3dShadingAspect {
        &self.shading_aspect
    }

    pub fn shading_aspect_mut(&mut self) -> &mut Prs3dShadingAspect {
        &mut self.shading_aspect
    }

    pub fn hlr_type(&self) -> Prs3dTypeOfHLR {
        self.hlr_type
    }

    pub fn set_hlr_type(&mut self, hlr_type: Prs3dTypeOfHLR) {
        self.hlr_type = hlr_type;
    }

    pub fn nb_points(&self) -> u32 {
        self.nb_points
    }

    pub fn set_nb_points(&mut self, nb_points: u32) {
        self.nb_points = nb_points;
    }

    pub fn max_flyflyangle(&self) -> f64 {
        self.max_flyflyangle
    }

    pub fn set_max_flyflyangle(&mut self, angle: f64) {
        self.max_flyflyangle = angle;
    }
}

impl Default for Prs3dDrawer {
    fn default() -> Self {
        Self::new()
    }
}
