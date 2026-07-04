// FILE: iges_dimen_linear_dimension.rs
// occt: IGESDimen_LinearDimension

/// Defines LinearDimension, Type <216> Form <0>
/// in package IGESDimen
pub struct IgesDimen_LinearDimension {
    note: Option<Box<IgesDimen_GeneralNote>>,
    leaders: (Option<Box<IgesDimen_LeaderArrow>>, Option<Box<IgesDimen_LeaderArrow>>),
    witness_lines: (Option<Box<IgesDimen_WitnessLine>>, Option<Box<IgesDimen_WitnessLine>>),
}

impl IgesDimen_LinearDimension {
    pub fn new() -> Self {
        IgesDimen_LinearDimension {
            note: None,
            leaders: (None, None),
            witness_lines: (None, None),
        }
    }

    pub fn init(
        &mut self,
        a_note: IgesDimen_GeneralNote,
        leaders: (IgesDimen_LeaderArrow, IgesDimen_LeaderArrow),
        witness_lines: (Option<IgesDimen_WitnessLine>, Option<IgesDimen_WitnessLine>),
    ) {
        self.note = Some(Box::new(a_note));
        self.leaders = (
            Some(Box::new(leaders.0)),
            Some(Box::new(leaders.1)),
        );
        self.witness_lines = (
            witness_lines.0.map(Box::new),
            witness_lines.1.map(Box::new),
        );
    }

    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    pub fn first_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.leaders.0.as_ref().map(|l| l.as_ref())
    }

    pub fn second_leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.leaders.1.as_ref().map(|l| l.as_ref())
    }

    pub fn first_witness_line(&self) -> Option<&IgesDimen_WitnessLine> {
        self.witness_lines.0.as_ref().map(|l| l.as_ref())
    }

    pub fn second_witness_line(&self) -> Option<&IgesDimen_WitnessLine> {
        self.witness_lines.1.as_ref().map(|l| l.as_ref())
    }
}

impl Default for IgesDimen_LinearDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct IgesDimen_GeneralNote;

#[derive(Clone)]
pub struct IgesDimen_LeaderArrow;

#[derive(Clone)]
pub struct IgesDimen_WitnessLine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_dimension_creation() {
        let dim = IgesDimen_LinearDimension::new();
        assert!(dim.note().is_none());
    }
}
