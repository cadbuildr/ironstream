// FILE: iges_dimen_diameter_dimension.rs
// occt: IGESDimen_DiameterDimension

/// Defines DiameterDimension, Type <206> Form <0>
/// in package IGESDimen
/// Used for dimensioning diameters
pub struct IgesDimen_DiameterDimension {
    note: Option<Box<IgesDimen_GeneralNote>>,
    first_leader: Option<Box<IgesDimen_LeaderArrow>>,
    second_leader: Option<Box<IgesDimen_LeaderArrow>>,
    center: (f64, f64),
}

impl IgesDimen_DiameterDimension {
    /// Create a new DiameterDimension entity
    pub fn new() -> Self {
        IgesDimen_DiameterDimension {
            note: None,
            first_leader: None,
            second_leader: None,
            center: (0.0, 0.0),
        }
    }

    /// This method is used to set the fields of the class DiameterDimension
    pub fn init(
        &mut self,
        a_note: IgesDimen_GeneralNote,
        a_leader: IgesDimen_LeaderArrow,
        another_leader: Option<IgesDimen_LeaderArrow>,
        a_center: (f64, f64),
    ) {
        self.note = Some(Box::new(a_note));
        self.first_leader = Some(Box::new(a_leader));
        self.second_leader = another_leader.map(Box::new);
        self.center = a_center;
    }

    /// Returns the General Note Entity
    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    /// Returns the First Leader Entity
    pub fn first_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.first_leader.as_ref().map(|l| l.as_ref())
    }

    /// Returns False if theSecondleader is None.
    pub fn has_second_leader(&self) -> bool {
        self.second_leader.is_some()
    }

    /// Returns the Second Leader Entity
    pub fn second_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.second_leader.as_ref().map(|l| l.as_ref())
    }

    /// Returns the Arc Center coordinates
    pub fn center(&self) -> (f64, f64) {
        self.center
    }

    /// Returns the Arc Center coordinates after Transformation.
    /// (Z = 0.0 for Transformation)
    pub fn transformed_center(&self) -> (f64, f64) {
        self.center
    }
}

impl Default for IgesDimen_DiameterDimension {
    fn default() -> Self {
        Self::new()
    }
}

// Placeholder types
#[derive(Clone)]
pub struct IgesDimen_GeneralNote;

#[derive(Clone)]
pub struct IgesDimen_LeaderArrow;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diameter_dimension_creation() {
        let dim = IgesDimen_DiameterDimension::new();
        assert!(dim.note().is_none());
        assert_eq!(dim.center(), (0.0, 0.0));
    }

    #[test]
    fn test_diameter_dimension_init() {
        let mut dim = IgesDimen_DiameterDimension::new();
        let note = IgesDimen_GeneralNote;
        let leader1 = IgesDimen_LeaderArrow;
        let leader2 = IgesDimen_LeaderArrow;

        dim.init(note, leader1, Some(leader2), (5.0, 10.0));

        assert!(dim.note().is_some());
        assert_eq!(dim.center(), (5.0, 10.0));
        assert!(dim.has_second_leader());
    }
}
