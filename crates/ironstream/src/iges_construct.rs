// FILE: iges_construct.rs
// occt: IGESControl_Controller
// occt-ref: IGESData_IGESModel, IGESData_IGESEntity

// occt: IGESData_DefType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesDefType {
    Undefined,
    Reference,
    RankColor,
    RankLineFont,
    RankLevel,
}

// occt: IGESData_Status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesStatus {
    Ok,
    Unknown,
    Error,
}

// occt-ref: IGESData_IGESEntity // (simplified)
#[derive(Clone, Debug)]
pub struct IgesEntity {
    pub entity_type: u32,
    pub form_number: u16,
    pub label: String,
    pub status: IgesStatus,
    pub level: i32,
}

impl IgesEntity {
    pub fn new(entity_type: u32, form_number: u16) -> Self {
        Self {
            entity_type, form_number,
            label: String::new(), status: IgesStatus::Ok, level: 0,
        }
    }

    pub fn type_number(&self) -> u32 { self.entity_type }
    pub fn form_number(&self) -> u16 { self.form_number }
    pub fn is_ok(&self) -> bool { self.status == IgesStatus::Ok }
}

// occt-ref: IGESData_IGESModel
/// In-memory IGES model: ordered list of entities with a global section.
#[derive(Clone, Debug, Default)]
pub struct IgesModel {
    pub entities: Vec<IgesEntity>,
    pub author: String,
    pub organisation: String,
    pub file_name: String,
    pub unit_flag: u32,
    pub unit_name: String,
}

impl IgesModel {
    pub fn new() -> Self { Self::default() }

    pub fn add_entity(&mut self, e: IgesEntity) {
        self.entities.push(e);
    }

    pub fn nb_entities(&self) -> usize { self.entities.len() }

    pub fn entity(&self, idx: usize) -> Option<&IgesEntity> {
        self.entities.get(idx)
    }

    pub fn entities_of_type(&self, etype: u32) -> Vec<&IgesEntity> {
        self.entities.iter().filter(|e| e.entity_type == etype).collect()
    }
}

// occt: IGESControl_Controller
/// Controls reading and writing of IGES files.
#[derive(Clone, Debug)]
pub struct IgesController {
    pub mode: IgesMode,
    pub unit: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesMode {
    Read,
    Write,
    ReadWrite,
}

impl IgesController {
    pub fn new(mode: IgesMode) -> Self {
        Self { mode, unit: 1.0 }
    }

    pub fn read_file(&self, _path: &str) -> IgesModel {
        IgesModel::new()
    }

    pub fn write_model(&self, _model: &IgesModel, _path: &str) -> bool {
        self.mode == IgesMode::Write || self.mode == IgesMode::ReadWrite
    }
}

// occt: IGESData_IGESReaderTool
#[derive(Clone, Debug, Default)]
pub struct IgesReaderTool {
    pub nb_read: usize,
}

impl IgesReaderTool {
    pub fn new() -> Self { Self::default() }

    pub fn load(&mut self, model: &IgesModel) {
        self.nb_read = model.nb_entities();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iges_model_add_entity() {
        let mut m = IgesModel::new();
        m.add_entity(IgesEntity::new(110, 0));
        m.add_entity(IgesEntity::new(110, 0));
        m.add_entity(IgesEntity::new(100, 0));
        assert_eq!(m.nb_entities(), 3);
        assert_eq!(m.entities_of_type(110).len(), 2);
    }

    #[test]
    fn iges_controller_read() {
        let ctrl = IgesController::new(IgesMode::ReadWrite);
        let model = ctrl.read_file("test.igs");
        assert_eq!(model.nb_entities(), 0);
    }

    #[test]
    fn iges_entity_ok() {
        let e = IgesEntity::new(116, 0);
        assert!(e.is_ok());
        assert_eq!(e.type_number(), 116);
    }
}
