// FILE: iges_dimen_curve_dimension.rs
// occt: IGESDimen_CurveDimension

/// Defines CurveDimension, Type <204> Form <0>
/// in package IGESDimen
/// Used to dimension curves
pub struct IgesDimen_CurveDimension {
    note: Option<Box<IgesDimen_GeneralNote>>,
    first_curve: Option<Box<IgesData_IgesEntity>>,
    second_curve: Option<Box<IgesData_IgesEntity>>,
    first_leader: Option<Box<IgesDimen_LeaderArrow>>,
    second_leader: Option<Box<IgesDimen_LeaderArrow>>,
    first_witness_line: Option<Box<IgesDimen_WitnessLine>>,
    second_witness_line: Option<Box<IgesDimen_WitnessLine>>,
}

impl IgesDimen_CurveDimension {
    /// Create a new CurveDimension entity
    pub fn new() -> Self {
        IgesDimen_CurveDimension {
            note: None,
            first_curve: None,
            second_curve: None,
            first_leader: None,
            second_leader: None,
            first_witness_line: None,
            second_witness_line: None,
        }
    }

    /// This method is used to set the fields of the class CurveDimension
    pub fn init(
        &mut self,
        a_note: IgesDimen_GeneralNote,
        a_curve: IgesData_IgesEntity,
        another_curve: IgesData_IgesEntity,
        a_leader: IgesDimen_LeaderArrow,
        another_leader: IgesDimen_LeaderArrow,
        a_line: Option<IgesDimen_WitnessLine>,
        another_line: Option<IgesDimen_WitnessLine>,
    ) {
        self.note = Some(Box::new(a_note));
        self.first_curve = Some(Box::new(a_curve));
        self.second_curve = Some(Box::new(another_curve));
        self.first_leader = Some(Box::new(a_leader));
        self.second_leader = Some(Box::new(another_leader));
        self.first_witness_line = a_line.map(Box::new);
        self.second_witness_line = another_line.map(Box::new);
    }

    /// Returns the General Note Entity
    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    /// Returns the First curve Entity
    pub fn first_curve(&self) -> Option<&IgesData_IgesEntity> {
        self.first_curve.as_ref().map(|c| c.as_ref())
    }

    /// Returns False if theSecondCurve is None.
    pub fn has_second_curve(&self) -> bool {
        self.second_curve.is_some()
    }

    /// Returns the Second curve Entity or None.
    pub fn second_curve(&self) -> Option<&IgesData_IgesEntity> {
        self.second_curve.as_ref().map(|c| c.as_ref())
    }

    /// Returns the First Leader Entity
    pub fn first_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.first_leader.as_ref().map(|l| l.as_ref())
    }

    /// Returns the Second Leader Entity
    pub fn second_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.second_leader.as_ref().map(|l| l.as_ref())
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
}

impl Default for IgesDimen_CurveDimension {
    fn default() -> Self {
        Self::new()
    }
}

// Placeholder types
#[derive(Clone)]
pub struct IgesDimen_GeneralNote;

#[derive(Clone)]
pub struct IgesData_IgesEntity;

#[derive(Clone)]
pub struct IgesDimen_LeaderArrow;

#[derive(Clone)]
pub struct IgesDimen_WitnessLine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_dimension_creation() {
        let dim = IgesDimen_CurveDimension::new();
        assert!(dim.note().is_none());
        assert!(dim.first_curve().is_none());
        assert!(!dim.has_second_curve());
    }
}
