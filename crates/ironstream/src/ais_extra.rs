// FILE: crates/ironstream/src/ais_extra.rs

// occt: AIS_TypeOfAxis
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisTypeOfAxis {
    XAxis,
    YAxis,
    ZAxis,
    UnknownAxis,
}

// occt-ref: AIS_Trihedron
pub struct AisTrihedron {
    pub position: [f64; 3],
    pub size: f64,
    pub x_color: [f32; 3],
    pub y_color: [f32; 3],
    pub z_color: [f32; 3],
    pub is_displayed: bool,
}

impl AisTrihedron {
    pub fn new(position: [f64; 3], size: f64) -> Self {
        Self {
            position,
            size,
            x_color: [1.0, 0.0, 0.0],
            y_color: [0.0, 1.0, 0.0],
            z_color: [0.0, 0.0, 1.0],
            is_displayed: false,
        }
    }

    pub fn set_size(&mut self, s: f64) {
        self.size = s;
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn set_position(&mut self, p: [f64; 3]) {
        self.position = p;
    }

    pub fn set_x_color(&mut self, r: f32, g: f32, b: f32) {
        self.x_color = [r, g, b];
    }

    pub fn set_y_color(&mut self, r: f32, g: f32, b: f32) {
        self.y_color = [r, g, b];
    }

    pub fn set_z_color(&mut self, r: f32, g: f32, b: f32) {
        self.z_color = [r, g, b];
    }

    pub fn display(&mut self) {
        self.is_displayed = true;
    }

    pub fn hide(&mut self) {
        self.is_displayed = false;
    }

    pub fn is_displayed(&self) -> bool {
        self.is_displayed
    }
}

// occt-ref: AIS_Plane
pub struct AisPlane {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub size: f64,
    pub is_displayed: bool,
    pub color: [f32; 3],
}

impl AisPlane {
    pub fn new(origin: [f64; 3], normal: [f64; 3], size: f64) -> Self {
        Self {
            origin,
            normal,
            size,
            is_displayed: false,
            color: [0.8, 0.8, 0.8],
        }
    }

    pub fn set_size(&mut self, s: f64) {
        self.size = s;
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    pub fn display(&mut self) {
        self.is_displayed = true;
    }

    pub fn hide(&mut self) {
        self.is_displayed = false;
    }

    pub fn is_displayed(&self) -> bool {
        self.is_displayed
    }

    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }

    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }
}

// occt-ref: AIS_Axis
pub struct AisAxis {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
    pub axis_type: AisTypeOfAxis,
    pub is_displayed: bool,
    pub color: [f32; 3],
}

impl AisAxis {
    pub fn new(origin: [f64; 3], direction: [f64; 3], axis_type: AisTypeOfAxis) -> Self {
        Self {
            origin,
            direction,
            axis_type,
            is_displayed: false,
            color: [1.0, 1.0, 1.0],
        }
    }

    pub fn set_axis_type(&mut self, t: AisTypeOfAxis) {
        self.axis_type = t;
    }

    pub fn axis_type(&self) -> AisTypeOfAxis {
        self.axis_type
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    pub fn display(&mut self) {
        self.is_displayed = true;
    }

    pub fn hide(&mut self) {
        self.is_displayed = false;
    }

    pub fn is_displayed(&self) -> bool {
        self.is_displayed
    }
}

// occt-ref: AIS_DimensionType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AisDimensionType {
    Length,
    Angle,
    Radius,
    Diameter,
}

// occt-ref: PrsDim_Dimension
pub struct AisDimension {
    pub dim_type: AisDimensionType,
    pub value: f64,
    pub text: String,
    pub is_displayed: bool,
}

impl AisDimension {
    pub fn new(dim_type: AisDimensionType, value: f64) -> Self {
        Self {
            dim_type,
            value,
            text: String::new(),
            is_displayed: false,
        }
    }

    pub fn set_text(&mut self, t: &str) {
        self.text = t.to_string();
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn display(&mut self) {
        self.is_displayed = true;
    }

    pub fn hide(&mut self) {
        self.is_displayed = false;
    }
}
