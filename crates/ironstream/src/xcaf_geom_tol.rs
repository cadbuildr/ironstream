// FILE: xcaf_geom_tol.rs
// occt: XCAFDimTolObjects_GeomToleranceObject, XCAFDimTolObjects_DatumObject, XCAFDoc_GeomTolerance

// occt: XCAFDimTolObjects_GeomToleranceType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomToleranceType {
    Straightness,
    Flatness,
    Circularity,
    Cylindricity,
    ProfileOfLine,
    ProfileOfSurface,
    Angularity,
    Perpendicularity,
    Parallelism,
    Position,
    Concentricity,
    Symmetry,
    CircularRunout,
    TotalRunout,
}

impl GeomToleranceType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Straightness => "Straightness",
            Self::Flatness => "Flatness",
            Self::Circularity => "Circularity",
            Self::Cylindricity => "Cylindricity",
            Self::ProfileOfLine => "ProfileOfLine",
            Self::ProfileOfSurface => "ProfileOfSurface",
            Self::Angularity => "Angularity",
            Self::Perpendicularity => "Perpendicularity",
            Self::Parallelism => "Parallelism",
            Self::Position => "Position",
            Self::Concentricity => "Concentricity",
            Self::Symmetry => "Symmetry",
            Self::CircularRunout => "CircularRunout",
            Self::TotalRunout => "TotalRunout",
        }
    }

    pub fn is_form(&self) -> bool {
        matches!(self, Self::Straightness | Self::Flatness | Self::Circularity | Self::Cylindricity)
    }

    pub fn is_orientation(&self) -> bool {
        matches!(self, Self::Angularity | Self::Perpendicularity | Self::Parallelism)
    }

    pub fn is_location(&self) -> bool {
        matches!(self, Self::Position | Self::Concentricity | Self::Symmetry)
    }

    pub fn is_runout(&self) -> bool {
        matches!(self, Self::CircularRunout | Self::TotalRunout)
    }
}

// occt: XCAFDimTolObjects_GeomToleranceObject
#[derive(Clone, Debug)]
pub struct GeomToleranceObject {
    pub tol_type: GeomToleranceType,
    pub value: f64,
    pub modifier: Option<GeomToleranceModifier>,
    pub datum_refs: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomToleranceModifier {
    MaximumMaterialCondition,
    LeastMaterialCondition,
    Free,
    Regardless,
    Envelope,
}

impl GeomToleranceObject {
    pub fn new(tol_type: GeomToleranceType, value: f64) -> Self {
        Self { tol_type, value, modifier: None, datum_refs: Vec::new() }
    }

    pub fn with_modifier(mut self, m: GeomToleranceModifier) -> Self {
        self.modifier = Some(m); self
    }

    pub fn add_datum_ref(&mut self, label: impl Into<String>) {
        self.datum_refs.push(label.into());
    }

    pub fn has_datum_refs(&self) -> bool { !self.datum_refs.is_empty() }
    pub fn value(&self) -> f64 { self.value }
    pub fn tol_type(&self) -> GeomToleranceType { self.tol_type }
}

// occt: XCAFDimTolObjects_DatumObject
#[derive(Clone, Debug)]
pub struct DatumObject {
    pub label: String,
    pub modifiers: Vec<DatumModifier>,
    pub target_type: DatumTargetType,
    pub target_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatumModifier {
    Translation,
    NotConvex,
    CollinearZone,
    MinorDiameter,
    MajorDiameter,
    LineDatum,
    Any,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatumTargetType {
    Point,
    Line,
    Rectangle,
    Circle,
    Any,
}

impl DatumObject {
    pub fn new(label: impl Into<String>, target_type: DatumTargetType) -> Self {
        Self { label: label.into(), modifiers: Vec::new(), target_type, target_value: 0.0 }
    }

    pub fn add_modifier(&mut self, m: DatumModifier) { self.modifiers.push(m); }
    pub fn has_modifiers(&self) -> bool { !self.modifiers.is_empty() }
    pub fn name(&self) -> &str { &self.label }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geom_tolerance_types() {
        assert!(GeomToleranceType::Flatness.is_form());
        assert!(GeomToleranceType::Perpendicularity.is_orientation());
        assert!(GeomToleranceType::Position.is_location());
        assert!(GeomToleranceType::TotalRunout.is_runout());
    }

    #[test]
    fn geom_tolerance_object() {
        let mut t = GeomToleranceObject::new(GeomToleranceType::Circularity, 0.05);
        t.add_datum_ref("A");
        assert!(t.has_datum_refs());
        assert!((t.value() - 0.05).abs() < 1e-10);
        assert_eq!(t.tol_type().name(), "Circularity");
    }

    #[test]
    fn datum_object() {
        let mut d = DatumObject::new("A", DatumTargetType::Point);
        d.add_modifier(DatumModifier::Translation);
        assert!(d.has_modifiers());
        assert_eq!(d.name(), "A");
    }

    #[test]
    fn tolerance_with_modifier() {
        let t = GeomToleranceObject::new(GeomToleranceType::Position, 0.1)
            .with_modifier(GeomToleranceModifier::MaximumMaterialCondition);
        assert_eq!(t.modifier, Some(GeomToleranceModifier::MaximumMaterialCondition));
    }

    #[test]
    fn tolerance_name() {
        assert_eq!(GeomToleranceType::ProfileOfSurface.name(), "ProfileOfSurface");
    }
}
