// FILE: iges_geom_offset_curve.rs
// occt: IGESGeom_OffsetCurve

/// Defines IGESOffsetCurve, Type <130> Form <0> in package IGESGeom.
/// An OffsetCurve entity contains the data necessary to determine the offset
/// of a given curve. This entity points to the base curve to be offset and
/// contains offset distance and other pertinent information.
#[derive(Clone, Debug)]
pub struct OffsetCurve {
    /// The curve entity to be offset
    base_curve_id: Option<i32>,
    /// Offset distance flag:
    /// 1 = Single value, uniform distance
    /// 2 = Varying linearly
    /// 3 = As a specified function
    offset_type: i32,
    /// Curve entity describing offset as a function (if offset_type = 3)
    function_id: Option<i32>,
    /// Particular coordinate of curve describing offset (if offset_type = 3)
    function_parameter: i32,
    /// Tapered offset type flag:
    /// 1 = Function of arc length
    /// 2 = Function of parameter
    tapered_offset_type: i32,
    /// First offset distance (used if offset_type = 1 or 2)
    offset_distance1: f64,
    /// Arc length or parameter value of first offset distance (if offset_type = 2)
    arc_length1: f64,
    /// Second offset distance
    offset_distance2: f64,
    /// Arc length or parameter value of second offset distance (if offset_type = 2)
    arc_length2: f64,
    /// Unit vector normal to plane containing curve to be offset
    normal_vector: [f64; 3],
    /// Start parameter value of offset curve
    offset_param1: f64,
    /// End parameter value of offset curve
    offset_param2: f64,
    /// Entity type for IGES (always 130)
    entity_type: i32,
}

impl OffsetCurve {
    /// Creates a new OffsetCurve entity.
    pub fn new() -> Self {
        OffsetCurve {
            base_curve_id: None,
            offset_type: 1,
            function_id: None,
            function_parameter: 0,
            tapered_offset_type: 1,
            offset_distance1: 0.0,
            arc_length1: 0.0,
            offset_distance2: 0.0,
            arc_length2: 0.0,
            normal_vector: [0.0, 0.0, 1.0],
            offset_param1: 0.0,
            offset_param2: 0.0,
            entity_type: 130,
        }
    }

    /// Initializes the OffsetCurve with all parameters.
    pub fn init(
        &mut self,
        base_curve: Option<i32>,
        offset_type: i32,
        function: Option<i32>,
        function_coord: i32,
        tapered_offset_type: i32,
        off_distance1: f64,
        arc_length1: f64,
        off_distance2: f64,
        arc_length2: f64,
        normal_vec: [f64; 3],
        offset_param1: f64,
        offset_param2: f64,
    ) {
        self.base_curve_id = base_curve;
        self.offset_type = offset_type;
        self.function_id = function;
        self.function_parameter = function_coord;
        self.tapered_offset_type = tapered_offset_type;
        self.offset_distance1 = off_distance1;
        self.arc_length1 = arc_length1;
        self.offset_distance2 = off_distance2;
        self.arc_length2 = arc_length2;
        self.normal_vector = normal_vec;
        self.offset_param1 = offset_param1;
        self.offset_param2 = offset_param2;
    }

    /// Returns the base curve entity ID.
    pub fn base_curve(&self) -> Option<i32> {
        self.base_curve_id
    }

    /// Returns the offset type (1, 2, or 3).
    pub fn offset_type(&self) -> i32 {
        self.offset_type
    }

    /// Returns the function entity ID defining the offset, if present.
    pub fn function(&self) -> Option<i32> {
        self.function_id
    }

    /// Returns true if a function defining the offset is present.
    pub fn has_function(&self) -> bool {
        self.function_id.is_some()
    }

    /// Returns the function parameter (coordinate of curve describing offset).
    pub fn function_parameter(&self) -> i32 {
        self.function_parameter
    }

    /// Returns the tapered offset type (1 or 2).
    pub fn tapered_offset_type(&self) -> i32 {
        self.tapered_offset_type
    }

    /// Returns the first offset distance.
    pub fn first_offset_distance(&self) -> f64 {
        self.offset_distance1
    }

    /// Returns the arc length or parameter value of first offset distance.
    pub fn arc_length1(&self) -> f64 {
        self.arc_length1
    }

    /// Returns the second offset distance.
    pub fn second_offset_distance(&self) -> f64 {
        self.offset_distance2
    }

    /// Returns the arc length or parameter value of second offset distance.
    pub fn arc_length2(&self) -> f64 {
        self.arc_length2
    }

    /// Returns the unit vector normal to the plane containing the curve.
    pub fn normal_vector(&self) -> [f64; 3] {
        self.normal_vector
    }

    /// Returns the normal vector after transformation.
    /// In full implementation, this would apply the entity's transformation.
    pub fn transformed_normal_vector(&self) -> [f64; 3] {
        // TODO: Apply transformation matrix if present
        self.normal_vector
    }

    /// Returns start and end parameters.
    pub fn parameters(&self) -> (f64, f64) {
        (self.offset_param1, self.offset_param2)
    }

    /// Returns the start parameter value of the offset curve.
    pub fn start_parameter(&self) -> f64 {
        self.offset_param1
    }

    /// Returns the end parameter value of the offset curve.
    pub fn end_parameter(&self) -> f64 {
        self.offset_param2
    }

    /// Returns the entity type number (always 130).
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for OffsetCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_offset_curve() {
        let curve = OffsetCurve::new();
        assert_eq!(curve.base_curve(), None);
        assert_eq!(curve.offset_type(), 1);
        assert!(!curve.has_function());
        assert_eq!(curve.normal_vector(), [0.0, 0.0, 1.0]);
        assert_eq!(curve.entity_type(), 130);
    }

    #[test]
    fn test_init_offset_curve() {
        let mut curve = OffsetCurve::new();
        curve.init(
            Some(1),
            1,
            None,
            0,
            1,
            0.5,
            0.0,
            0.5,
            0.0,
            [0.0, 0.0, 1.0],
            0.0,
            1.0,
        );

        assert_eq!(curve.base_curve(), Some(1));
        assert_eq!(curve.offset_type(), 1);
        assert!(!curve.has_function());
        assert_eq!(curve.first_offset_distance(), 0.5);
        assert_eq!(curve.second_offset_distance(), 0.5);
    }

    #[test]
    fn test_offset_curve_with_function() {
        let mut curve = OffsetCurve::new();
        curve.init(
            Some(1),
            3,
            Some(2),
            1,
            2,
            0.0,
            0.0,
            0.0,
            0.0,
            [0.0, 0.0, 1.0],
            0.0,
            1.0,
        );

        assert_eq!(curve.offset_type(), 3);
        assert!(curve.has_function());
        assert_eq!(curve.function(), Some(2));
        assert_eq!(curve.function_parameter(), 1);
        assert_eq!(curve.tapered_offset_type(), 2);
    }

    #[test]
    fn test_parameters() {
        let mut curve = OffsetCurve::new();
        curve.init(None, 1, None, 0, 1, 1.0, 0.0, 1.0, 0.0, [0.0, 1.0, 0.0], 0.5, 2.5);

        let (start, end) = curve.parameters();
        assert_eq!(start, 0.5);
        assert_eq!(end, 2.5);
        assert_eq!(curve.start_parameter(), 0.5);
        assert_eq!(curve.end_parameter(), 2.5);
    }

    #[test]
    fn test_transformed_normal_vector() {
        let mut curve = OffsetCurve::new();
        curve.init(None, 1, None, 0, 1, 1.0, 0.0, 1.0, 0.0, [1.0, 2.0, 3.0], 0.0, 1.0);

        // Without transformation, transformed vector equals original
        assert_eq!(curve.transformed_normal_vector(), [1.0, 2.0, 3.0]);
    }
}
