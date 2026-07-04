// FILE: iges_data_read_stage.rs
// occt: IGESData_ReadStage

//! Enumeration for the stages of IGES file reading.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadStage {
    /// Begin reading process
    Begin,
    /// Reading header section
    Header,
    /// Reading parameter section
    Params,
    /// Reading entities
    Entities,
    /// Closing
    End,
}

impl ReadStage {
    pub fn next(&self) -> Option<ReadStage> {
        match self {
            ReadStage::Begin => Some(ReadStage::Header),
            ReadStage::Header => Some(ReadStage::Params),
            ReadStage::Params => Some(ReadStage::Entities),
            ReadStage::Entities => Some(ReadStage::End),
            ReadStage::End => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ReadStage::Begin => "Begin",
            ReadStage::Header => "Header",
            ReadStage::Params => "Params",
            ReadStage::Entities => "Entities",
            ReadStage::End => "End",
        }
    }
}

impl Default for ReadStage {
    fn default() -> Self {
        ReadStage::Begin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut stage = ReadStage::Begin;
        assert_eq!(stage.next(), Some(ReadStage::Header));

        stage = ReadStage::Header;
        assert_eq!(stage.next(), Some(ReadStage::Params));

        stage = ReadStage::Params;
        assert_eq!(stage.next(), Some(ReadStage::Entities));

        stage = ReadStage::Entities;
        assert_eq!(stage.next(), Some(ReadStage::End));

        stage = ReadStage::End;
        assert_eq!(stage.next(), None);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(ReadStage::Begin.as_str(), "Begin");
        assert_eq!(ReadStage::Header.as_str(), "Header");
        assert_eq!(ReadStage::Params.as_str(), "Params");
        assert_eq!(ReadStage::Entities.as_str(), "Entities");
        assert_eq!(ReadStage::End.as_str(), "End");
    }

    #[test]
    fn test_default() {
        let stage = ReadStage::default();
        assert_eq!(stage, ReadStage::Begin);
    }
}
