// FILE: blend_func.rs
// occt: Blend_Function, Blend_CSFunction, Blend_SurfRstFunction
//       Blend_CurvPointFuncInv, BlendFunc_Corde, BlendFunc_Ruled,
//       BlendFunc_EvolRad, BRepBlend_Line, BRepBlend_PointOnRst

/// Type of blend section.
/// occt: Blend_DecrochStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendDecrochStatus {
    NoDecrocha,
    DecrochRst1,
    DecrochRst2,
    DecrochBoth,
}

impl Default for BlendDecrochStatus {
    fn default() -> Self { Self::NoDecrocha }
}

/// Status of a blend walk.
// occt-ref: BRepBlend_Status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendStatus {
    StepTooLarge,
    StepTooSmall,
    Backward,
    Forward,
    Ok,
    NotDone,
}

impl Default for BlendStatus {
    fn default() -> Self { Self::NotDone }
}

impl BlendStatus {
    pub fn is_done(&self) -> bool { *self == Self::Ok }
}

/// A blend point (result along the walk).
/// occt: Blend_Point
#[derive(Clone, Debug)]
pub struct BlendPoint {
    pub parameter: f64,
    pub point_on_s1: [f64; 3],
    pub point_on_s2: [f64; 3],
    pub uv1: [f64; 2],
    pub uv2: [f64; 2],
    pub tangent: [f64; 3],
    pub normal: [f64; 3],
    pub is_tangency: bool,
}

impl BlendPoint {
    pub fn new(
        parameter: f64,
        p1: [f64; 3],
        p2: [f64; 3],
        uv1: [f64; 2],
        uv2: [f64; 2],
    ) -> Self {
        Self {
            parameter, point_on_s1: p1, point_on_s2: p2, uv1, uv2,
            tangent: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], is_tangency: false,
        }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn point_on_s1(&self) -> [f64; 3] { self.point_on_s1 }
    pub fn point_on_s2(&self) -> [f64; 3] { self.point_on_s2 }
    pub fn uv1(&self) -> [f64; 2] { self.uv1 }
    pub fn uv2(&self) -> [f64; 2] { self.uv2 }
    pub fn tangent(&self) -> [f64; 3] { self.tangent }
    pub fn is_tangency(&self) -> bool { self.is_tangency }

    pub fn set_tangent(&mut self, t: [f64; 3]) { self.tangent = t; }
    pub fn set_tangency(&mut self, v: bool) { self.is_tangency = v; }
}

/// A result line from a blend walk (sequence of blend points).
/// occt: BRepBlend_Line
#[derive(Clone, Debug, Default)]
pub struct BrepBlendLine {
    pub points: Vec<BlendPoint>,
    pub start_point: Option<BlendPoint>,
    pub end_point: Option<BlendPoint>,
    pub has_start_point: bool,
    pub has_end_point: bool,
}

impl BrepBlendLine {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, p: BlendPoint) { self.points.push(p); }

    pub fn prepend(&mut self, p: BlendPoint) { self.points.insert(0, p); }

    pub fn set_start_point(&mut self, p: BlendPoint) {
        self.start_point = Some(p);
        self.has_start_point = true;
    }

    pub fn set_end_point(&mut self, p: BlendPoint) {
        self.end_point = Some(p);
        self.has_end_point = true;
    }

    pub fn nb_points(&self) -> usize { self.points.len() }

    pub fn point(&self, i: usize) -> Option<&BlendPoint> {
        if i == 0 { None } else { self.points.get(i - 1) }
    }

    pub fn first_point(&self) -> Option<&BlendPoint> { self.points.first() }
    pub fn last_point(&self) -> Option<&BlendPoint> { self.points.last() }

    pub fn has_start_point(&self) -> bool { self.has_start_point }
    pub fn has_end_point(&self) -> bool { self.has_end_point }
}

/// Corde blend function (constant radius fillet).
/// occt: BlendFunc_Corde // / BlendFunc_EvolRad
#[derive(Clone, Debug)]
pub struct BlendFuncEvolRad {
    pub surface1_id: u32,
    pub surface2_id: u32,
    pub initial_radius: f64,
    pub final_radius: f64,
    pub is_rational: bool,
    pub nb_equations: usize,
}

impl BlendFuncEvolRad {
    pub fn new(surf1: u32, surf2: u32, radius_law_id: u32) -> Self {
        Self {
            surface1_id: surf1,
            surface2_id: surf2,
            initial_radius: 1.0,
            final_radius: 1.0,
            is_rational: radius_law_id > 0,
            nb_equations: 4,
        }
    }

    pub fn set_radius(&mut self, r: f64) { self.initial_radius = r; self.final_radius = r; }
    pub fn set_radius_evolving(&mut self, r0: f64, r1: f64) {
        self.initial_radius = r0;
        self.final_radius = r1;
    }

    pub fn nb_equations(&self) -> usize { self.nb_equations }
    pub fn is_rational(&self) -> bool { self.is_rational }
    pub fn initial_radius(&self) -> f64 { self.initial_radius }
    pub fn final_radius(&self) -> f64 { self.final_radius }

    /// Evaluate the blend function at parameter t (stub).
    pub fn value(&self, _uv1: [f64; 2], _uv2: [f64; 2], _t: f64) -> [f64; 4] {
        [0.0; 4]
    }
}

/// Ruled blend function (straight-line sections).
/// occt: BlendFunc_Ruled
#[derive(Clone, Debug)]
pub struct BlendFuncRuled {
    pub surface1_id: u32,
    pub surface2_id: u32,
    pub nb_equations: usize,
}

impl BlendFuncRuled {
    pub fn new(surf1: u32, surf2: u32) -> Self {
        Self { surface1_id: surf1, surface2_id: surf2, nb_equations: 4 }
    }

    pub fn nb_equations(&self) -> usize { self.nb_equations }

    pub fn value(&self, _uv1: [f64; 2], _uv2: [f64; 2]) -> [f64; 4] { [0.0; 4] }
}

/// A point on a restriction (boundary) curve of a surface.
/// occt: BRepBlend_PointOnRst
#[derive(Clone, Debug)]
pub struct PointOnRst {
    pub arc_id: u32,
    pub parameter_on_arc: f64,
    pub transition_info: u32,
}

impl PointOnRst {
    pub fn new(arc_id: u32, param: f64) -> Self {
        Self { arc_id, parameter_on_arc: param, transition_info: 0 }
    }

    pub fn arc_id(&self) -> u32 { self.arc_id }
    pub fn parameter_on_arc(&self) -> f64 { self.parameter_on_arc }
    pub fn set_transition_info(&mut self, t: u32) { self.transition_info = t; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_status_predicates() {
        assert!(BlendStatus::Ok.is_done());
        assert!(!BlendStatus::NotDone.is_done());
        assert!(!BlendStatus::Backward.is_done());
    }

    #[test]
    fn blend_point_basic() {
        let p = BlendPoint::new(0.5, [1.0,0.0,0.0], [0.0,1.0,0.0], [0.5,0.5], [0.3,0.7]);
        assert!((p.parameter() - 0.5).abs() < 1e-12);
        assert_eq!(p.point_on_s1(), [1.0,0.0,0.0]);
        assert_eq!(p.uv1(), [0.5,0.5]);
        assert!(!p.is_tangency());
    }

    #[test]
    fn brep_blend_line_ops() {
        let mut line = BrepBlendLine::new();
        line.append(BlendPoint::new(0.0, [0.0,0.0,0.0], [0.0,0.0,0.0], [0.0,0.0], [0.0,0.0]));
        line.append(BlendPoint::new(1.0, [1.0,0.0,0.0], [1.0,0.0,0.0], [1.0,0.0], [1.0,0.0]));
        assert_eq!(line.nb_points(), 2);
        assert_eq!(line.point(1).unwrap().parameter().round() as i32, 0);
        assert!(line.point(0).is_none());
        assert!(!line.has_start_point());
    }

    #[test]
    fn blend_func_evol_rad() {
        let mut f = BlendFuncEvolRad::new(1, 2, 3);
        f.set_radius(0.5);
        assert!((f.initial_radius() - 0.5).abs() < 1e-12);
        assert_eq!(f.nb_equations(), 4);
        assert!(f.is_rational());
    }

    #[test]
    fn blend_func_ruled() {
        let f = BlendFuncRuled::new(1, 2);
        assert_eq!(f.nb_equations(), 4);
        let v = f.value([0.0,0.0], [0.0,0.0]);
        assert_eq!(v, [0.0;4]);
    }

    #[test]
    fn point_on_rst() {
        let mut p = PointOnRst::new(5, 0.3);
        assert_eq!(p.arc_id(), 5);
        assert!((p.parameter_on_arc() - 0.3).abs() < 1e-12);
        p.set_transition_info(2);
        assert_eq!(p.transition_info, 2);
    }
}
