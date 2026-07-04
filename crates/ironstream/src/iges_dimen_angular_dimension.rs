// FILE: iges_dimen_angular_dimension.rs
// occt: IGESDimen_AngularDimension

/// Defines AngularDimension, Type <202> Form <0>
/// in package IGESDimen
/// Used to dimension angles
pub struct IgesDimen_AngularDimension {
    note: Option<Box<IgesDimen_GeneralNote>>,
    first_witness_line: Option<Box<IgesDimen_WitnessLine>>,
    second_witness_line: Option<Box<IgesDimen_WitnessLine>>,
    vertex: (f64, f64),
    radius: f64,
    first_leader: Option<Box<IgesDimen_LeaderArrow>>,
    second_leader: Option<Box<IgesDimen_LeaderArrow>>,
}

impl IgesDimen_AngularDimension {
    /// Create a new AngularDimension entity
    pub fn new() -> Self {
        IgesDimen_AngularDimension {
            note: None,
            first_witness_line: None,
            second_witness_line: None,
            vertex: (0.0, 0.0),
            radius: 0.0,
            first_leader: None,
            second_leader: None,
        }
    }

    /// This method is used to set the fields of the class AngularDimension
    /// - a_note         : General Note Entity
    /// - a_line         : First Witness Line Entity or None
    /// - another_line   : Second Witness Line Entity or None
    /// - a_vertex       : Coordinates of vertex point (x, y)
    /// - a_radius       : Radius of leader arcs
    /// - a_leader       : First Leader Entity
    /// - another_leader : Second Leader Entity
    pub fn init(
        &mut self,
        a_note: IgesDimen_GeneralNote,
        a_line: Option<IgesDimen_WitnessLine>,
        another_line: Option<IgesDimen_WitnessLine>,
        a_vertex: (f64, f64),
        a_radius: f64,
        a_leader: IgesDimen_LeaderArrow,
        another_leader: IgesDimen_LeaderArrow,
    ) {
        self.note = Some(Box::new(a_note));
        self.first_witness_line = a_line.map(Box::new);
        self.second_witness_line = another_line.map(Box::new);
        self.vertex = a_vertex;
        self.radius = a_radius;
        self.first_leader = Some(Box::new(a_leader));
        self.second_leader = Some(Box::new(another_leader));
    }

    /// Returns the General Note Entity of the Dimension.
    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    /// Returns False if theFirstWitnessLine is None.
    pub fn has_first_witness_line(&self) -> bool {
        self.first_witness_line.is_some()
    }

    /// Returns the First Witness Line Entity or None.
    pub fn first_witness_line(&self) -> Option<&IgesDimen_WitnessLine> {
        self.first_witness_line.as_ref().map(|l| l.as_ref())
    }

    /// Returns False if theSecondWitnessLine is None.
    pub fn has_second_witness_line(&self) -> bool {
        self.second_witness_line.is_some()
    }

    /// Returns the Second Witness Line Entity or None.
    pub fn second_witness_line(&self) -> Option<&IgesDimen_WitnessLine> {
        self.second_witness_line.as_ref().map(|l| l.as_ref())
    }

    /// Returns the coordinates of the Vertex point.
    pub fn vertex(&self) -> (f64, f64) {
        self.vertex
    }

    /// Returns the coordinates of the Vertex point after Transformation.
    /// (Z = 0.0 for Transformation)
    pub fn transformed_vertex(&self) -> (f64, f64) {
        self.vertex
    }

    /// Returns the Radius of the Leader arcs.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the First Leader Entity.
    pub fn first_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.first_leader.as_ref().map(|l| l.as_ref())
    }

    /// Returns the Second Leader Entity.
    pub fn second_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.second_leader.as_ref().map(|l| l.as_ref())
    }
}

impl Default for IgesDimen_AngularDimension {
    fn default() -> Self {
        Self::new()
    }
}

// Placeholder types
#[derive(Clone)]
pub struct IgesDimen_GeneralNote;

#[derive(Clone)]
pub struct IgesDimen_WitnessLine;

#[derive(Clone)]
pub struct IgesDimen_LeaderArrow;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angular_dimension_creation() {
        let dim = IgesDimen_AngularDimension::new();
        assert_eq!(dim.vertex(), (0.0, 0.0));
        assert_eq!(dim.radius(), 0.0);
        assert!(!dim.has_first_witness_line());
        assert!(!dim.has_second_witness_line());
    }

    #[test]
    fn test_angular_dimension_init() {
        let mut dim = IgesDimen_AngularDimension::new();
        let note = IgesDimen_GeneralNote;
        let line1 = IgesDimen_WitnessLine;
        let leader1 = IgesDimen_LeaderArrow;
        let leader2 = IgesDimen_LeaderArrow;

        dim.init(
            note,
            Some(line1),
            None,
            (10.0, 20.0),
            5.0,
            leader1,
            leader2,
        );

        assert_eq!(dim.vertex(), (10.0, 20.0));
        assert_eq!(dim.radius(), 5.0);
        assert!(dim.has_first_witness_line());
        assert!(!dim.has_second_witness_line());
    }

    #[test]
    fn test_angular_dimension_access() {
        let mut dim = IgesDimen_AngularDimension::new();
        let note = IgesDimen_GeneralNote;
        let leader1 = IgesDimen_LeaderArrow;
        let leader2 = IgesDimen_LeaderArrow;

        dim.init(note, None, None, (5.0, 15.0), 3.5, leader1, leader2);

        assert!(dim.note().is_some());
        assert!(dim.first_leader().is_some());
        assert!(dim.second_leader().is_some());
        assert_eq!(dim.transformed_vertex(), (5.0, 15.0));
    }
}
