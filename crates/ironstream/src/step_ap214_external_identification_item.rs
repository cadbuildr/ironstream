// FILE: step_ap214_external_identification_item.rs
// occt: StepAP214_ExternalIdentificationItem

/// Representation of STEP AP214 ExternalIdentificationItem SelectType.
#[derive(Clone, Debug)]
pub enum ExternalIdentificationItem {
    DocumentFile,
    ExternallyDefinedClass,
    ExternallyDefinedGeneralProperty,
    ProductDefinition,
    AppliedOrganizationAssignment,
    AppliedPersonAndOrganizationAssignment,
    Approval,
    ApprovalStatus,
    ExternalSource,
    OrganizationalAddress,
    SecurityClassification,
    TrimmedCurve,
    VersionedActionRequest,
    DateAndTimeAssignment,
    DateAssignment,
}

impl ExternalIdentificationItem {
    pub fn case_num(&self) -> i32 {
        match self {
            ExternalIdentificationItem::DocumentFile => 1,
            ExternalIdentificationItem::ExternallyDefinedClass => 2,
            ExternalIdentificationItem::ExternallyDefinedGeneralProperty => 3,
            ExternalIdentificationItem::ProductDefinition => 4,
            ExternalIdentificationItem::AppliedOrganizationAssignment => 5,
            ExternalIdentificationItem::AppliedPersonAndOrganizationAssignment => 6,
            ExternalIdentificationItem::Approval => 7,
            ExternalIdentificationItem::ApprovalStatus => 8,
            ExternalIdentificationItem::ExternalSource => 9,
            ExternalIdentificationItem::OrganizationalAddress => 10,
            ExternalIdentificationItem::SecurityClassification => 11,
            ExternalIdentificationItem::TrimmedCurve => 12,
            ExternalIdentificationItem::VersionedActionRequest => 13,
            ExternalIdentificationItem::DateAndTimeAssignment => 14,
            ExternalIdentificationItem::DateAssignment => 15,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(ExternalIdentificationItem::DocumentFile.case_num(), 1);
        assert_eq!(ExternalIdentificationItem::DateAssignment.case_num(), 15);
    }
}
