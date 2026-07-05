// FILE: open_gl_aspects.rs
// occt: OpenGl_Aspects

use std::rc::Rc;
use std::cell::RefCell;

/// Graphic3d_TypeOfShadingModel enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dTypeOfShadingModel {
    Unlit,
    Flat,
    Gouraud,
    Phong,
}

/// Graphic3d_Aspects holds graphics attributes.
#[derive(Clone, Debug)]
pub struct Graphic3dAspects {
    shading_model: Graphic3dTypeOfShadingModel,
    color_r: f32,
    color_g: f32,
    color_b: f32,
    alpha: f32,
}

impl Graphic3dAspects {
    pub fn new() -> Self {
        Graphic3dAspects {
            shading_model: Graphic3dTypeOfShadingModel::Gouraud,
            color_r: 1.0,
            color_g: 1.0,
            color_b: 1.0,
            alpha: 1.0,
        }
    }

    pub fn shading_model(&self) -> Graphic3dTypeOfShadingModel {
        self.shading_model
    }

    pub fn set_shading_model(&mut self, model: Graphic3dTypeOfShadingModel) {
        self.shading_model = model;
    }
}

pub type Graphic3dAspectsHandle = Rc<RefCell<Graphic3dAspects>>;

/// OpenGl_Element is the base class for OpenGL rendering elements.
pub trait OpenGlElement {
    fn render(&self);
}

/// OpenGl_Aspects represents graphics aspects for rendering.
pub struct OpenGlAspects {
    aspect: Option<Graphic3dAspectsHandle>,
    shading_model: Graphic3dTypeOfShadingModel,
}

impl OpenGlAspects {
    pub fn new() -> Self {
        OpenGlAspects {
            aspect: None,
            shading_model: Graphic3dTypeOfShadingModel::Gouraud,
        }
    }

    pub fn with_aspect(aspect: Graphic3dAspectsHandle) -> Self {
        let shading_model = aspect.borrow().shading_model();
        OpenGlAspects {
            aspect: Some(aspect),
            shading_model,
        }
    }

    pub fn aspect(&self) -> Option<&Graphic3dAspectsHandle> {
        self.aspect.as_ref()
    }

    pub fn set_aspect(&mut self, aspect: Graphic3dAspectsHandle) {
        self.shading_model = aspect.borrow().shading_model();
        self.aspect = Some(aspect);
    }

    pub fn shading_model(&self) -> Graphic3dTypeOfShadingModel {
        self.shading_model
    }

    pub fn set_no_lighting(&mut self) {
        self.shading_model = Graphic3dTypeOfShadingModel::Unlit;
    }
}

impl OpenGlElement for OpenGlAspects {
    fn render(&self) {
        // Rendering logic would be implemented here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphic3d_aspects_creation() {
        let aspects = Graphic3dAspects::new();
        assert_eq!(aspects.shading_model(), Graphic3dTypeOfShadingModel::Gouraud);
    }

    #[test]
    fn test_graphic3d_aspects_shading_model() {
        let mut aspects = Graphic3dAspects::new();
        aspects.set_shading_model(Graphic3dTypeOfShadingModel::Phong);
        assert_eq!(aspects.shading_model(), Graphic3dTypeOfShadingModel::Phong);
    }

    #[test]
    fn test_opengl_aspects_creation() {
        let aspects = OpenGlAspects::new();
        assert_eq!(aspects.shading_model(), Graphic3dTypeOfShadingModel::Gouraud);
    }

    #[test]
    fn test_opengl_aspects_with_aspect() {
        let aspect_handle = Rc::new(RefCell::new(Graphic3dAspects::new()));
        let gl_aspects = OpenGlAspects::with_aspect(aspect_handle.clone());
        assert_eq!(gl_aspects.aspect(), Some(&aspect_handle));
    }

    #[test]
    fn test_opengl_aspects_no_lighting() {
        let mut aspects = OpenGlAspects::new();
        aspects.set_no_lighting();
        assert_eq!(aspects.shading_model(), Graphic3dTypeOfShadingModel::Unlit);
    }

    #[test]
    fn test_opengl_aspects_render() {
        let aspects = OpenGlAspects::new();
        aspects.render();
    }
}
