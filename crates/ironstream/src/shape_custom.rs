// FILE: rust/ironstream/crates/ironstream/src/shape_custom.rs

// occt: ShapeCustom // modification kind
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeCustomModType {
    None,
    DirectFaces,
    SweptToElementary,
    BSplineToElementary,
}

// occt: ShapeCustom_Modification
pub struct ShapeCustomModification {
    mod_type: ShapeCustomModType,
    tolerance: f64,
    is_done: bool,
    nb_modified: u32,
}

impl ShapeCustomModification {
    pub fn new(mod_type: ShapeCustomModType, tolerance: f64) -> Self {
        Self {
            mod_type,
            tolerance,
            is_done: false,
            nb_modified: 0,
        }
    }

    pub fn perform(&mut self) {
        self.is_done = true;
        self.nb_modified = 0;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_modified_faces(&self) -> u32 {
        self.nb_modified
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn mod_type(&self) -> ShapeCustomModType {
        self.mod_type
    }
}

// occt: ShapeCustom_DirectModification
pub struct ShapeCustomDirectFaces {
    inner: ShapeCustomModification,
}

impl ShapeCustomDirectFaces {
    pub fn new() -> Self {
        Self {
            inner: ShapeCustomModification::new(ShapeCustomModType::DirectFaces, 1e-7),
        }
    }

    pub fn perform(&mut self) {
        self.inner.perform();
    }

    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    pub fn nb_modified_faces(&self) -> u32 {
        self.inner.nb_modified_faces()
    }

    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    pub fn mod_type(&self) -> ShapeCustomModType {
        self.inner.mod_type()
    }
}

impl Default for ShapeCustomDirectFaces {
    fn default() -> Self {
        Self::new()
    }
}

// occt: ShapeCustom_SweptToElementary
pub struct ShapeCustomSweptToElementary {
    inner: ShapeCustomModification,
}

impl ShapeCustomSweptToElementary {
    pub fn new() -> Self {
        Self {
            inner: ShapeCustomModification::new(ShapeCustomModType::SweptToElementary, 1e-7),
        }
    }

    pub fn perform(&mut self) {
        self.inner.perform();
    }

    pub fn is_done(&self) -> bool {
        self.inner.is_done()
    }

    pub fn nb_modified_faces(&self) -> u32 {
        self.inner.nb_modified_faces()
    }

    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    pub fn mod_type(&self) -> ShapeCustomModType {
        self.inner.mod_type()
    }
}

impl Default for ShapeCustomSweptToElementary {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: ShapeCustom_RestrictionParameters
pub struct ShapeCustomRestricted {
    pub gmax_degree: u32,
    pub gmax_seg: u32,
    pub convert_bspline_to_bezier: bool,
    pub convert_swept_to_elementary: bool,
    pub convert_plane_to_canonical: bool,
    pub convert_elementary_to_canonical: bool,
}

impl Default for ShapeCustomRestricted {
    fn default() -> Self {
        Self {
            gmax_degree: 15,
            gmax_seg: 10000,
            convert_bspline_to_bezier: true,
            convert_swept_to_elementary: true,
            convert_plane_to_canonical: true,
            convert_elementary_to_canonical: true,
        }
    }
}
