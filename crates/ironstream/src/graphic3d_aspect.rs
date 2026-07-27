// FILE: crates/ironstream/src/graphic3d_aspect.rs

// occt-ref: Graphic3d_TypeOfShaderObject
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dShaderType {
    Vertex,
    Fragment,
    Geometry,
    TessControl,
    TessEval,
    Compute,
}

// occt: Graphic3d_ShaderObject
pub struct Graphic3dShaderObject {
    shader_type: Graphic3dShaderType,
    source: String,
}

impl Graphic3dShaderObject {
    pub fn new(shader_type: Graphic3dShaderType, source: &str) -> Self {
        Graphic3dShaderObject {
            shader_type,
            source: source.to_string(),
        }
    }

    pub fn shader_type(&self) -> Graphic3dShaderType {
        self.shader_type
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

// occt-ref: Aspect_HatchStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dFillMethod {
    Solid,
    Hollow,
    HorizontalLine,
    VerticalLine,
    DiagonalLeft45,
    DiagonalRight45,
    DiagonalCross45,
    HorizontalVerticalCross,
}

// occt-ref: Aspect_InteriorStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dInteriorStyle {
    Empty,
    Hollow,
    Hatch,
    Filled,
    Hiddenline,
}

// occt: Graphic3d_AspectFillArea3d
pub struct Graphic3dAspectFillArea {
    front_color: [f32; 4],
    back_color: [f32; 4],
    edge_color: [f32; 4],
    interior_style: Graphic3dInteriorStyle,
    fill_method: Graphic3dFillMethod,
    edge_width: f32,
    is_back_face_culled: bool,
}

impl Graphic3dAspectFillArea {
    pub fn new() -> Self {
        Graphic3dAspectFillArea {
            front_color: [1.0, 1.0, 1.0, 1.0],
            back_color: [1.0, 1.0, 1.0, 1.0],
            edge_color: [0.0, 0.0, 0.0, 1.0],
            interior_style: Graphic3dInteriorStyle::Filled,
            fill_method: Graphic3dFillMethod::Solid,
            edge_width: 1.0,
            is_back_face_culled: false,
        }
    }

    pub fn set_front_color(&mut self, color: [f32; 4]) {
        self.front_color = color;
    }

    pub fn front_color(&self) -> [f32; 4] {
        self.front_color
    }

    pub fn set_interior_style(&mut self, style: Graphic3dInteriorStyle) {
        self.interior_style = style;
    }

    pub fn interior_style(&self) -> Graphic3dInteriorStyle {
        self.interior_style
    }

    pub fn set_edge_color(&mut self, color: [f32; 4]) {
        self.edge_color = color;
    }

    pub fn edge_color(&self) -> [f32; 4] {
        self.edge_color
    }

    pub fn set_edge_width(&mut self, width: f32) {
        self.edge_width = width;
    }

    pub fn edge_width(&self) -> f32 {
        self.edge_width
    }

    pub fn set_back_face_culled(&mut self, cull: bool) {
        self.is_back_face_culled = cull;
    }

    pub fn is_back_face_culled(&self) -> bool {
        self.is_back_face_culled
    }
}

impl Default for Graphic3dAspectFillArea {
    fn default() -> Self {
        Self::new()
    }
}
