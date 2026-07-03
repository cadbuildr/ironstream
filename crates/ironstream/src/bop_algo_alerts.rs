// FILE: bop_algo_alerts.rs
// occt: BOPAlgo_Alerts

/// Alert types used in Boolean Operation algorithms
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BopAlgoAlert {
    /// Boolean operation was stopped by user
    UserBreak,
    /// Boolean operation of given type is not allowed on the given inputs
    BopNotAllowed,
    /// The type of Boolean Operation is not set
    BopNotSet,
    /// Building of the result shape has failed
    BuilderFailed,
    /// The intersection of the arguments has failed
    IntersectionFailed,
    /// More than one argument is provided
    MultipleArguments,
    /// The Pave Filler (the intersection tool) has not been created
    NoFiller,
    /// Null input shapes
    NullInputShapes,
    /// Cannot connect face intersection curves
    PostTreatFF,
    /// The BuilderSolid algorithm has failed
    SolidBuilderFailed,
    /// There are no enough arguments to perform the operation
    TooFewArguments,
    /// The positioning of the shapes leads to creation of the small edges without valid range
    BadPositioning,
    /// Some of the arguments are empty shapes
    EmptyShape,
    /// Some edges are very small and have such a small valid range, that they cannot be split
    NotSplittableEdge,
    /// Removal of internal boundaries among Edges has failed
    RemovalOfIBForEdgesFailed,
    /// Removal of internal boundaries among Faces has failed
    RemovalOfIBForFacesFailed,
    /// Removal of internal boundaries among the multi-dimensional shapes is not supported yet
    RemovalOfIBForMDimShapes,
    /// Removal of internal boundaries among Solids has failed
    RemovalOfIBForSolidsFailed,
    /// Some of the arguments are self-interfering shapes
    SelfInterferingShape,
    /// The positioning of the shapes leads to creation of the small edges without valid range
    ShellSplitterFailed,
    /// Some edges are too small and have no valid range
    TooSmallEdge,
    /// Intersection of pair of shapes has failed
    IntersectionOfPairOfShapesFailed,
    /// Building 2D curve of edge on face has failed
    BuildingPCurveFailed,
    /// Some sub-shapes of some of the argument become connected through other shapes and the argument became self-interfered
    AcquiredSelfIntersection,
    /// Unsupported type of input shape
    UnsupportedType,
    /// No faces have been found for removal
    NoFacesToRemove,
    /// Unable to remove the feature
    UnableToRemoveTheFeature,
    /// The Feature Removal algorithm has failed
    RemoveFeaturesFailed,
    /// Some of the faces passed to the Solid Builder algorithm have not been classified and not used for solids creation
    SolidBuilderUnusedFaces,
    /// Some of the edges passed to the Face Builder algorithm have not been classified and not used for faces creation
    FaceBuilderUnusedEdges,
    /// Unable to orient the shape correctly
    UnableToOrientTheShape,
    /// Shape is unknown for operation
    UnknownShape,
    /// No periodicity has been requested for the shape
    NoPeriodicityRequired,
    /// Unable to trim the shape for making it periodic (BOP Common fails)
    UnableToTrim,
    /// Unable to make the shape to look identical on opposite sides (Splitter fails)
    UnableToMakeIdentical,
    /// Unable to repeat the shape (Gluer fails)
    UnableToRepeat,
    /// Multi-dimensional arguments
    MultiDimensionalArguments,
    /// Unable to make the shape periodic
    UnableToMakePeriodic,
    /// Unable to glue the shapes
    UnableToGlue,
    /// The shape is not periodic
    ShapeIsNotPeriodic,
    /// Unable to make closed edge on face (to make a seam)
    UnableToMakeClosedEdgeOnFace,
}

impl BopAlgoAlert {
    /// Returns a human-readable description of the alert
    pub fn description(&self) -> &'static str {
        match self {
            BopAlgoAlert::UserBreak => "Boolean operation was stopped by user",
            BopAlgoAlert::BopNotAllowed => "Boolean operation of given type is not allowed on the given inputs",
            BopAlgoAlert::BopNotSet => "The type of Boolean Operation is not set",
            BopAlgoAlert::BuilderFailed => "Building of the result shape has failed",
            BopAlgoAlert::IntersectionFailed => "The intersection of the arguments has failed",
            BopAlgoAlert::MultipleArguments => "More than one argument is provided",
            BopAlgoAlert::NoFiller => "The Pave Filler (the intersection tool) has not been created",
            BopAlgoAlert::NullInputShapes => "Null input shapes",
            BopAlgoAlert::PostTreatFF => "Cannot connect face intersection curves",
            BopAlgoAlert::SolidBuilderFailed => "The BuilderSolid algorithm has failed",
            BopAlgoAlert::TooFewArguments => "There are no enough arguments to perform the operation",
            BopAlgoAlert::BadPositioning => "The positioning of the shapes leads to creation of the small edges without valid range",
            BopAlgoAlert::EmptyShape => "Some of the arguments are empty shapes",
            BopAlgoAlert::NotSplittableEdge => "Some edges are very small and have such a small valid range, that they cannot be split",
            BopAlgoAlert::RemovalOfIBForEdgesFailed => "Removal of internal boundaries among Edges has failed",
            BopAlgoAlert::RemovalOfIBForFacesFailed => "Removal of internal boundaries among Faces has failed",
            BopAlgoAlert::RemovalOfIBForMDimShapes => "Removal of internal boundaries among the multi-dimensional shapes is not supported yet",
            BopAlgoAlert::RemovalOfIBForSolidsFailed => "Removal of internal boundaries among Solids has failed",
            BopAlgoAlert::SelfInterferingShape => "Some of the arguments are self-interfering shapes",
            BopAlgoAlert::ShellSplitterFailed => "The positioning of the shapes leads to creation of the small edges without valid range",
            BopAlgoAlert::TooSmallEdge => "Some edges are too small and have no valid range",
            BopAlgoAlert::IntersectionOfPairOfShapesFailed => "Intersection of pair of shapes has failed",
            BopAlgoAlert::BuildingPCurveFailed => "Building 2D curve of edge on face has failed",
            BopAlgoAlert::AcquiredSelfIntersection => "Some sub-shapes of some of the argument become connected through other shapes and the argument became self-interfered",
            BopAlgoAlert::UnsupportedType => "Unsupported type of input shape",
            BopAlgoAlert::NoFacesToRemove => "No faces have been found for removal",
            BopAlgoAlert::UnableToRemoveTheFeature => "Unable to remove the feature",
            BopAlgoAlert::RemoveFeaturesFailed => "The Feature Removal algorithm has failed",
            BopAlgoAlert::SolidBuilderUnusedFaces => "Some of the faces passed to the Solid Builder algorithm have not been classified and not used for solids creation",
            BopAlgoAlert::FaceBuilderUnusedEdges => "Some of the edges passed to the Face Builder algorithm have not been classified and not used for faces creation",
            BopAlgoAlert::UnableToOrientTheShape => "Unable to orient the shape correctly",
            BopAlgoAlert::UnknownShape => "Shape is unknown for operation",
            BopAlgoAlert::NoPeriodicityRequired => "No periodicity has been requested for the shape",
            BopAlgoAlert::UnableToTrim => "Unable to trim the shape for making it periodic (BOP Common fails)",
            BopAlgoAlert::UnableToMakeIdentical => "Unable to make the shape to look identical on opposite sides (Splitter fails)",
            BopAlgoAlert::UnableToRepeat => "Unable to repeat the shape (Gluer fails)",
            BopAlgoAlert::MultiDimensionalArguments => "Multi-dimensional arguments",
            BopAlgoAlert::UnableToMakePeriodic => "Unable to make the shape periodic",
            BopAlgoAlert::UnableToGlue => "Unable to glue the shapes",
            BopAlgoAlert::ShapeIsNotPeriodic => "The shape is not periodic",
            BopAlgoAlert::UnableToMakeClosedEdgeOnFace => "Unable to make closed edge on face (to make a seam)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_description() {
        let alert = BopAlgoAlert::UserBreak;
        assert_eq!(alert.description(), "Boolean operation was stopped by user");
    }

    #[test]
    fn test_alert_variants() {
        let alerts = vec![
            BopAlgoAlert::UserBreak,
            BopAlgoAlert::BopNotAllowed,
            BopAlgoAlert::BuilderFailed,
            BopAlgoAlert::IntersectionFailed,
        ];

        for alert in alerts {
            let desc = alert.description();
            assert!(!desc.is_empty());
        }
    }

    #[test]
    fn test_alert_equality() {
        let alert1 = BopAlgoAlert::BuilderFailed;
        let alert2 = BopAlgoAlert::BuilderFailed;
        assert_eq!(alert1, alert2);
    }

    #[test]
    fn test_alert_inequality() {
        let alert1 = BopAlgoAlert::BuilderFailed;
        let alert2 = BopAlgoAlert::IntersectionFailed;
        assert_ne!(alert1, alert2);
    }

    #[test]
    fn test_all_alert_descriptions() {
        let all_alerts = vec![
            BopAlgoAlert::UserBreak,
            BopAlgoAlert::BopNotAllowed,
            BopAlgoAlert::BopNotSet,
            BopAlgoAlert::BuilderFailed,
            BopAlgoAlert::IntersectionFailed,
            BopAlgoAlert::MultipleArguments,
            BopAlgoAlert::NoFiller,
            BopAlgoAlert::NullInputShapes,
            BopAlgoAlert::PostTreatFF,
            BopAlgoAlert::SolidBuilderFailed,
            BopAlgoAlert::TooFewArguments,
        ];

        for alert in all_alerts {
            let desc = alert.description();
            assert!(!desc.is_empty(), "Alert {:?} has empty description", alert);
        }
    }
}
