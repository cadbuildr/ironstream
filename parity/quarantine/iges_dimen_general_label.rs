// FILE: iges_dimen_general_label.rs
// occt: IGESDimen_GeneralLabel

/// Defines GeneralLabel, Type <210> Form <0>
/// in package IGESDimen
pub struct IgesDimen_GeneralLabel {
    note: Option<Box<IgesDimen_GeneralNote>>,
    entities: Vec<IgesData_IgesEntity>,
}

impl IgesDimen_GeneralLabel {
    pub fn new() -> Self {
        IgesDimen_GeneralLabel {
            note: None,
            entities: Vec::new(),
        }
    }

    pub fn init(&mut self, a_note: IgesDimen_GeneralNote, ents: Vec<IgesData_IgesEntity>) {
        self.note = Some(Box::new(a_note));
        self.entities = ents;
    }

    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn entity(&self, index: usize) -> Option<&IgesData_IgesEntity> {
        if index == 0 || index > self.entities.len() {
            return None;
        }
        Some(&self.entities[index - 1])
    }
}

impl Default for IgesDimen_GeneralLabel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct IgesDimen_GeneralNote;

#[derive(Clone)]
pub struct IgesData_IgesEntity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_label_creation() {
        let label = IgesDimen_GeneralLabel::new();
        assert_eq!(label.nb_entities(), 0);
    }
}
