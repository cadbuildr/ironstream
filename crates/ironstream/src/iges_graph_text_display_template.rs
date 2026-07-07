// FILE: iges_graph_text_display_template.rs
// occt: IGESGraph_TextDisplayTemplate

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}

pub struct IGESGraphTextDisplayTemplate {
    box_width: f64,
    box_height: f64,
    font_code: i32,
    font_entity: Option<Box<IGESGraphTextFontDef>>,
    slant_angle: f64,
    rotation_angle: f64,
    mirror_flag: i32,
    rotate_flag: i32,
    corner: Point3D,
    form_number: i32,
}

// Placeholder for TextFontDef - would be replaced with real implementation
#[derive(Debug, Clone)]
pub struct IGESGraphTextFontDef {
    entity_id: i32,
}

impl IGESGraphTextDisplayTemplate {
    pub fn new() -> Self {
        IGESGraphTextDisplayTemplate {
            box_width: 0.0,
            box_height: 0.0,
            font_code: 0,
            font_entity: None,
            slant_angle: 0.0,
            rotation_angle: 0.0,
            mirror_flag: 0,
            rotate_flag: 0,
            corner: Point3D::new(0.0, 0.0, 0.0),
            form_number: 0,
        }
    }

    pub fn init(
        &mut self,
        a_width: f64,
        a_height: f64,
        a_font_code: i32,
        a_font_entity: Option<Box<IGESGraphTextFontDef>>,
        a_slant_angle: f64,
        a_rotation_angle: f64,
        a_mirror_flag: i32,
        a_rotation_flag: i32,
        a_corner: Point3D,
    ) {
        self.box_width = a_width;
        self.box_height = a_height;
        self.font_code = a_font_code;
        self.font_entity = a_font_entity;
        self.slant_angle = a_slant_angle;
        self.rotation_angle = a_rotation_angle;
        self.mirror_flag = a_mirror_flag;
        self.rotate_flag = a_rotation_flag;
        self.corner = a_corner;
    }

    pub fn set_incremental(&mut self, mode: bool) {
        self.form_number = if mode { 1 } else { 0 };
    }

    pub fn is_incremental(&self) -> bool {
        self.form_number == 1
    }

    pub fn box_width(&self) -> f64 {
        self.box_width
    }

    pub fn box_height(&self) -> f64 {
        self.box_height
    }

    pub fn is_font_entity(&self) -> bool {
        self.font_entity.is_some()
    }

    pub fn font_code(&self) -> i32 {
        self.font_code
    }

    pub fn font_entity(&self) -> Option<&IGESGraphTextFontDef> {
        self.font_entity.as_ref().map(|e| &**e)
    }

    pub fn slant_angle(&self) -> f64 {
        self.slant_angle
    }

    pub fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }

    pub fn mirror_flag(&self) -> i32 {
        self.mirror_flag
    }

    pub fn rotate_flag(&self) -> i32 {
        self.rotate_flag
    }

    pub fn starting_corner(&self) -> Point3D {
        self.corner
    }

    pub fn transformed_starting_corner(&self) -> Point3D {
        self.corner
    }
}

impl Default for IGESGraphTextDisplayTemplate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let template = IGESGraphTextDisplayTemplate::new();
        assert_eq!(template.box_width(), 0.0);
        assert_eq!(template.box_height(), 0.0);
        assert!(!template.is_font_entity());
        assert!(!template.is_incremental());
    }

    #[test]
    fn test_init() {
        let mut template = IGESGraphTextDisplayTemplate::new();
        let corner = Point3D::new(1.0, 2.0, 3.0);
        template.init(10.0, 20.0, 5, None, 0.5, 1.0, 0, 0, corner);

        assert_eq!(template.box_width(), 10.0);
        assert_eq!(template.box_height(), 20.0);
        assert_eq!(template.font_code(), 5);
        assert_eq!(template.slant_angle(), 0.5);
        assert_eq!(template.rotation_angle(), 1.0);
        assert_eq!(template.mirror_flag(), 0);
        assert_eq!(template.rotate_flag(), 0);
    }

    #[test]
    fn test_set_incremental() {
        let mut template = IGESGraphTextDisplayTemplate::new();
        assert!(!template.is_incremental());
        template.set_incremental(true);
        assert!(template.is_incremental());
        template.set_incremental(false);
        assert!(!template.is_incremental());
    }

    #[test]
    fn test_starting_corner() {
        let mut template = IGESGraphTextDisplayTemplate::new();
        let corner = Point3D::new(1.5, 2.5, 3.5);
        template.init(5.0, 5.0, 1, None, 0.0, 0.0, 0, 0, corner);

        let result = template.starting_corner();
        assert_eq!(result.x, 1.5);
        assert_eq!(result.y, 2.5);
        assert_eq!(result.z, 3.5);
    }
}
