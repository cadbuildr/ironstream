// FILE: iges_draw_drawing.rs
// occt: IGESDraw_Drawing

/// defines IGESDrawing, Type <404> Form <0>
/// in package IGESDraw
///
/// Specifies a drawing as a collection of annotation entities
/// defined in drawing space, and views which together
/// constitute a single representation of a part
pub struct IgesDrawDrawing {
    views: Vec<Box<dyn std::any::Any>>,
    view_origins: Vec<(f64, f64)>,
    annotations: Vec<Box<dyn std::any::Any>>,
}

impl IgesDrawDrawing {
    /// Create a new Drawing
    pub fn new() -> Self {
        IgesDrawDrawing {
            views: Vec::new(),
            view_origins: Vec::new(),
            annotations: Vec::new(),
        }
    }

    /// This method is used to set the fields of the class Drawing
    pub fn init(
        &mut self,
        views: Vec<Box<dyn std::any::Any>>,
        view_origins: Vec<(f64, f64)>,
        annotations: Vec<Box<dyn std::any::Any>>,
    ) {
        if views.len() != view_origins.len() {
            panic!("Lengths of allViews and allViewOrigins are not same");
        }
        self.views = views;
        self.view_origins = view_origins;
        self.annotations = annotations;
    }

    /// returns the number of view pointers in this
    pub fn nb_views(&self) -> i32 {
        self.views.len() as i32
    }

    /// returns the ViewKindEntity indicated by ViewIndex
    pub fn view_item(&self, view_index: i32) -> Option<&Box<dyn std::any::Any>> {
        if view_index <= 0 || view_index > self.nb_views() {
            panic!("ViewIndex out of bounds");
        }
        self.views.get((view_index - 1) as usize)
    }

    /// returns the Drawing space coordinates of the origin of the Transformed view
    pub fn view_origin(&self, t_view_index: i32) -> (f64, f64) {
        if t_view_index <= 0 || t_view_index > self.nb_views() {
            panic!("TViewIndex out of bounds");
        }
        self.view_origins[(t_view_index - 1) as usize]
    }

    /// returns the number of Annotation entities in this
    pub fn nb_annotations(&self) -> i32 {
        self.annotations.len() as i32
    }

    /// returns the Annotation entity in this Drawing
    pub fn annotation(&self, annotation_index: i32) -> Option<&Box<dyn std::any::Any>> {
        if annotation_index <= 0 || annotation_index > self.nb_annotations() {
            panic!("AnnotationIndex out of bounds");
        }
        self.annotations.get((annotation_index - 1) as usize)
    }

    /// TODO: Implement ViewToDrawing transformation
    pub fn view_to_drawing(&self, _num_view: i32, _view_coords: (f64, f64, f64)) -> (f64, f64) {
        (0.0, 0.0)
    }

    /// Returns the Drawing Unit Value if it is specified
    pub fn drawing_unit(&self) -> Option<f64> {
        None
    }

    /// Returns the Drawing Size if it is specified
    pub fn drawing_size(&self) -> Option<(f64, f64)> {
        None
    }
}

impl Default for IgesDrawDrawing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let drawing = IgesDrawDrawing::new();
        assert_eq!(drawing.nb_views(), 0);
        assert_eq!(drawing.nb_annotations(), 0);
    }

    #[test]
    fn test_init() {
        let mut drawing = IgesDrawDrawing::new();
        let views = vec![];
        let origins = vec![];
        let annotations = vec![];
        drawing.init(views, origins, annotations);

        assert_eq!(drawing.nb_views(), 0);
        assert_eq!(drawing.nb_annotations(), 0);
    }

    #[test]
    fn test_init_with_data() {
        let mut drawing = IgesDrawDrawing::new();
        let views = vec![Box::new(1) as Box<dyn std::any::Any>];
        let origins = vec![(1.0, 2.0)];
        let annotations = vec![];
        drawing.init(views, origins, annotations);

        assert_eq!(drawing.nb_views(), 1);
        assert_eq!(drawing.view_origin(1), (1.0, 2.0));
    }

    #[test]
    #[should_panic]
    fn test_init_mismatch_length() {
        let mut drawing = IgesDrawDrawing::new();
        let views = vec![Box::new(1) as Box<dyn std::any::Any>];
        let origins = vec![(1.0, 2.0), (3.0, 4.0)];
        drawing.init(views, origins, vec![]);
    }
}
