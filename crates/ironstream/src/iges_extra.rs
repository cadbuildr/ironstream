// FILE: iges_extra.rs
// occt: IGESData_IGESEntity, IGESData_IGESModel, IGESData_Protocol,
//       IGESGeom_BSplineCurve, IGESGeom_BSplineSurface, IGESGeom_TrimmedSurface,
//       IGESData_LineFontDefTemplate, IGESData_ColorEntity

/// IGES entity type number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IgesEntityType {
    CircularArc,           // 100
    CompositeCurve,        // 102
    ConicArc,              // 104
    CopiousData,           // 106
    Line,                  // 110
    ParametricSplineCurve, // 112
    ParametricSplineSurf,  // 114
    Point,                 // 116
    RuledSurface,          // 118
    SurfaceOfRevolution,   // 120
    TabulatedCylinder,     // 122
    DirectionEntity,       // 123
    TransformationMatrix,  // 124
    OffsetCurve,           // 130
    OffsetSurface,         // 140
    BoundaryEntity,        // 141
    CurveOnSurface,        // 142
    BoundedSurface,        // 143
    TrimmedSurface,        // 144
    BSplineCurve,          // 126
    BSplineSurface,        // 128
    Plane,                 // 108
    Property,              // 406
    SingularSubfigure,     // 408
    ColorDefinition,       // 314
    Other(u32),
}

impl IgesEntityType {
    pub fn type_number(&self) -> u32 {
        match self {
            IgesEntityType::CircularArc => 100,
            IgesEntityType::CompositeCurve => 102,
            IgesEntityType::ConicArc => 104,
            IgesEntityType::CopiousData => 106,
            IgesEntityType::Line => 110,
            IgesEntityType::BSplineCurve => 126,
            IgesEntityType::BSplineSurface => 128,
            IgesEntityType::TrimmedSurface => 144,
            IgesEntityType::TransformationMatrix => 124,
            IgesEntityType::ColorDefinition => 314,
            IgesEntityType::Other(n) => *n,
            _ => 0,
        }
    }

    pub fn is_curve(&self) -> bool {
        matches!(self, IgesEntityType::CircularArc | IgesEntityType::ConicArc
            | IgesEntityType::Line | IgesEntityType::BSplineCurve
            | IgesEntityType::CompositeCurve | IgesEntityType::ParametricSplineCurve)
    }

    pub fn is_surface(&self) -> bool {
        matches!(self, IgesEntityType::BSplineSurface | IgesEntityType::TrimmedSurface
            | IgesEntityType::RuledSurface | IgesEntityType::SurfaceOfRevolution
            | IgesEntityType::OffsetSurface | IgesEntityType::ParametricSplineSurf)
    }
}

// occt: IGESData_IGESEntity
#[derive(Clone, Debug)]
pub struct IgesEntity {
    pub entity_type: IgesEntityType,
    pub entity_number: u32,     // sequence number in IGES file (odd)
    pub structure: i32,
    pub line_font: i32,
    pub level: i32,
    pub view: i32,
    pub color: i32,
    pub label: String,
    pub subscript: u32,
    pub blank_status: u8,
    pub entity_use: u8,
    pub hierarchy: u8,
    pub parameters: Vec<f64>,
}

impl IgesEntity {
    pub fn new(entity_type: IgesEntityType, seq: u32) -> Self {
        Self {
            entity_type, entity_number: seq,
            structure: 0, line_font: 0, level: 0, view: 0, color: 7,
            label: String::new(), subscript: 0,
            blank_status: 0, entity_use: 0, hierarchy: 0,
            parameters: Vec::new(),
        }
    }

    pub fn is_visible(&self) -> bool { self.blank_status == 0 }
    pub fn has_color(&self) -> bool { self.color != 0 }
    pub fn nb_params(&self) -> usize { self.parameters.len() }

    pub fn param(&self, idx: usize) -> Option<f64> { self.parameters.get(idx).copied() }

    pub fn add_param(&mut self, v: f64) { self.parameters.push(v); }
}

// occt: IGESGeom_BSplineCurve (entity type 126)
#[derive(Clone, Debug)]
pub struct IgesBSplineCurve {
    pub entity: IgesEntity,
    pub degree: usize,
    pub nb_control_pts: usize,
    pub control_pts: Vec<[f64; 3]>,
    pub knots: Vec<f64>,
    pub weights: Vec<f64>,
    pub is_closed: bool,
    pub is_rational: bool,
    pub is_periodic: bool,
    pub is_planar: bool,
    pub start_param: f64,
    pub end_param: f64,
}

impl IgesBSplineCurve {
    pub fn new(degree: usize, control_pts: Vec<[f64; 3]>) -> Self {
        let n = control_pts.len();
        let nb_knots = n + degree + 1;
        let knots: Vec<f64> = (0..nb_knots).map(|i| i as f64 / (nb_knots - 1) as f64).collect();
        let weights = vec![1.0; n];
        Self {
            entity: IgesEntity::new(IgesEntityType::BSplineCurve, 1),
            degree, nb_control_pts: n, control_pts, knots, weights,
            is_closed: false, is_rational: false, is_periodic: false, is_planar: false,
            start_param: 0.0, end_param: 1.0,
        }
    }

    pub fn param_range(&self) -> f64 { self.end_param - self.start_param }
    pub fn is_valid(&self) -> bool {
        !self.control_pts.is_empty() && self.degree > 0
    }
}

// occt: IGESGeom_BSplineSurface (entity type 128)
#[derive(Clone, Debug)]
pub struct IgesBSplineSurface {
    pub entity: IgesEntity,
    pub degree_u: usize,
    pub degree_v: usize,
    pub nb_ctrl_u: usize,
    pub nb_ctrl_v: usize,
    pub control_pts: Vec<Vec<[f64; 3]>>,   // [u][v]
    pub knots_u: Vec<f64>,
    pub knots_v: Vec<f64>,
    pub weights: Vec<Vec<f64>>,
    pub is_closed_u: bool,
    pub is_closed_v: bool,
    pub is_rational: bool,
    pub is_polynomial: bool,
}

impl IgesBSplineSurface {
    pub fn new(du: usize, dv: usize, nu: usize, nv: usize) -> Self {
        let knots_u: Vec<f64> = (0..nu+du+1).map(|i| i as f64 / (nu+du) as f64).collect();
        let knots_v: Vec<f64> = (0..nv+dv+1).map(|i| i as f64 / (nv+dv) as f64).collect();
        let control_pts = vec![vec![[0.0; 3]; nv]; nu];
        let weights = vec![vec![1.0; nv]; nu];
        Self {
            entity: IgesEntity::new(IgesEntityType::BSplineSurface, 1),
            degree_u: du, degree_v: dv,
            nb_ctrl_u: nu, nb_ctrl_v: nv,
            control_pts, knots_u, knots_v, weights,
            is_closed_u: false, is_closed_v: false,
            is_rational: false, is_polynomial: true,
        }
    }

    pub fn nb_control_pts(&self) -> usize { self.nb_ctrl_u * self.nb_ctrl_v }

    pub fn control_pt(&self, u: usize, v: usize) -> Option<[f64; 3]> {
        self.control_pts.get(u)?.get(v).copied()
    }
}

// occt: IGESGeom_TrimmedSurface (entity type 144)
#[derive(Clone, Debug)]
pub struct IgesTrimmedSurface {
    pub entity: IgesEntity,
    pub surface_ref: u32,         // entity sequence ref to underlying surface
    pub outer_boundary: Option<u32>,  // entity seq ref
    pub inner_boundaries: Vec<u32>,
    pub is_outer_default: bool,
}

impl IgesTrimmedSurface {
    pub fn new(surface_ref: u32) -> Self {
        Self {
            entity: IgesEntity::new(IgesEntityType::TrimmedSurface, 1),
            surface_ref,
            outer_boundary: None,
            inner_boundaries: Vec::new(),
            is_outer_default: true,
        }
    }

    pub fn set_outer(&mut self, boundary_ref: u32) {
        self.outer_boundary = Some(boundary_ref);
        self.is_outer_default = false;
    }

    pub fn add_inner(&mut self, boundary_ref: u32) {
        self.inner_boundaries.push(boundary_ref);
    }

    pub fn nb_inner(&self) -> usize { self.inner_boundaries.len() }
}

// occt: IGESData_IGESModel
#[derive(Clone, Debug, Default)]
pub struct IgesModel {
    pub entities: Vec<IgesEntity>,
    pub author: String,
    pub organization: String,
    pub file_name: String,
    pub preprocessor: String,
    pub system_id: String,
    pub scale: f64,
}

impl IgesModel {
    pub fn new() -> Self { Self { scale: 1.0, ..Default::default() } }

    pub fn add_entity(&mut self, e: IgesEntity) -> u32 {
        let seq = (self.entities.len() * 2 + 1) as u32;
        self.entities.push(e);
        seq
    }

    pub fn nb_entities(&self) -> usize { self.entities.len() }

    pub fn entity_at(&self, seq: u32) -> Option<&IgesEntity> {
        let idx = ((seq as usize).saturating_sub(1)) / 2;
        self.entities.get(idx)
    }

    pub fn entities_of_type(&self, etype: IgesEntityType) -> Vec<&IgesEntity> {
        self.entities.iter().filter(|e| e.entity_type == etype).collect()
    }

    pub fn nb_curves(&self) -> usize {
        self.entities.iter().filter(|e| e.entity_type.is_curve()).count()
    }

    pub fn nb_surfaces(&self) -> usize {
        self.entities.iter().filter(|e| e.entity_type.is_surface()).count()
    }
}

// occt: IGESData_ColorEntity (entity 314)
#[derive(Clone, Debug)]
pub struct IgesColorDef {
    pub entity: IgesEntity,
    pub r: f64,   // 0.0–100.0
    pub g: f64,
    pub b: f64,
    pub name: String,
}

impl IgesColorDef {
    pub fn new(r: f64, g: f64, b: f64, name: impl Into<String>) -> Self {
        Self {
            entity: IgesEntity::new(IgesEntityType::ColorDefinition, 1),
            r: r.clamp(0.0, 100.0),
            g: g.clamp(0.0, 100.0),
            b: b.clamp(0.0, 100.0),
            name: name.into(),
        }
    }

    pub fn to_linear(&self) -> [f32; 3] {
        [(self.r / 100.0) as f32, (self.g / 100.0) as f32, (self.b / 100.0) as f32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_type_classification() {
        assert!(IgesEntityType::BSplineCurve.is_curve());
        assert!(!IgesEntityType::BSplineCurve.is_surface());
        assert!(IgesEntityType::TrimmedSurface.is_surface());
        assert_eq!(IgesEntityType::BSplineCurve.type_number(), 126);
    }

    #[test]
    fn iges_entity_basics() {
        let mut e = IgesEntity::new(IgesEntityType::Line, 1);
        e.add_param(1.0);
        e.add_param(2.0);
        assert_eq!(e.nb_params(), 2);
        assert!(e.is_visible());
    }

    #[test]
    fn bspline_curve() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
        let c = IgesBSplineCurve::new(2, pts);
        assert!(c.is_valid());
        assert_eq!(c.nb_control_pts, 3);
        assert!((c.param_range() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn iges_model_entities() {
        let mut model = IgesModel::new();
        let e1 = IgesEntity::new(IgesEntityType::Line, 1);
        let e2 = IgesEntity::new(IgesEntityType::BSplineSurface, 3);
        model.add_entity(e1);
        model.add_entity(e2);
        assert_eq!(model.nb_entities(), 2);
        assert_eq!(model.nb_curves(), 1);
        assert_eq!(model.nb_surfaces(), 1);
    }

    #[test]
    fn trimmed_surface() {
        let mut ts = IgesTrimmedSurface::new(1);
        ts.set_outer(3);
        ts.add_inner(5);
        assert_eq!(ts.nb_inner(), 1);
        assert!(!ts.is_outer_default);
    }

    #[test]
    fn color_def() {
        let c = IgesColorDef::new(100.0, 0.0, 0.0, "red");
        let lin = c.to_linear();
        assert!((lin[0] - 1.0).abs() < 1e-6);
        assert!((lin[1]).abs() < 1e-6);
    }
}
