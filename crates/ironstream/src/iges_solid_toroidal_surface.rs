// FILE: iges_solid_toroidal_surface.rs
// occt: IGESSolid_ToroidalSurface

/// Defines ToroidalSurface, Type <198> Form Number <0,1>
/// This entity is defined by the center point, the axis direction
/// and the major and minor radii. In case of parametrised surface
/// a reference direction is provided.
#[derive(Clone, Debug)]
pub struct IgesSolidToroidalSurface {
    center: Option<IgesEntity>,
    axis: Option<IgesEntity>,
    major_radius: f64,
    minor_radius: f64,
    reference_dir: Option<IgesEntity>,
    form_number: i32,
}

impl IgesSolidToroidalSurface {
    /// Creates a new ToroidalSurface
    pub fn new() -> Self {
        Self {
            center: None,
            axis: None,
            major_radius: 0.0,
            minor_radius: 0.0,
            reference_dir: None,
            form_number: 0,
        }
    }

    /// This method is used to set the fields of the class ToroidalSurface
    /// - aCenter   : the center point entity
    /// - anAxis    : the direction of the axis entity
    /// - majRadius : the major radius
    /// - minRadius : the minor radius
    /// - refdir    : the reference direction (parametrised)
    ///               default None for unparametrised surface
    pub fn init(
        &mut self,
        center: IgesEntity,
        axis: IgesEntity,
        major_radius: f64,
        minor_radius: f64,
        reference_dir: Option<IgesEntity>,
    ) {
        self.center = Some(center);
        self.axis = Some(axis);
        self.major_radius = major_radius;
        self.minor_radius = minor_radius;
        self.reference_dir = reference_dir.clone();
        // Form 0 if unparametrised, Form 1 if parametrised
        self.form_number = if reference_dir.is_some() { 1 } else { 0 };
    }

    /// Returns the center point entity of the surface
    pub fn center(&self) -> Option<IgesEntity> {
        self.center.clone()
    }

    /// Returns the direction of the axis
    pub fn axis(&self) -> Option<IgesEntity> {
        self.axis.clone()
    }

    /// Returns the major radius of the surface
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Returns the minor radius of the surface
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// Returns the reference direction (parametrised surface)
    /// None is returned if the surface is not parametrised
    pub fn reference_dir(&self) -> Option<IgesEntity> {
        self.reference_dir.clone()
    }

    /// Returns True if the surface is parametrised, else False
    pub fn is_parametrised(&self) -> bool {
        self.reference_dir.is_some()
    }

    /// Returns the form number (0 for unparametrised, 1 for parametrised)
    pub fn form_number(&self) -> i32 {
        self.form_number
    }
}

impl Default for IgesSolidToroidalSurface {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub type for IGES entities
#[derive(Clone, Debug, Default)]
pub struct IgesEntity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_unparametrised() {
        let surface = IgesSolidToroidalSurface::new();
        assert!(!surface.is_parametrised());
        assert_eq!(surface.form_number(), 0);
        assert_eq!(surface.major_radius(), 0.0);
        assert_eq!(surface.minor_radius(), 0.0);
    }

    #[test]
    fn test_init_unparametrised() {
        let mut surface = IgesSolidToroidalSurface::new();
        let center = IgesEntity::default();
        let axis = IgesEntity::default();
        surface.init(center, axis, 5.0, 2.0, None);

        assert!(!surface.is_parametrised());
        assert_eq!(surface.form_number(), 0);
        assert_eq!(surface.major_radius(), 5.0);
        assert_eq!(surface.minor_radius(), 2.0);
        assert!(surface.center().is_some());
        assert!(surface.axis().is_some());
        assert!(surface.reference_dir().is_none());
    }

    #[test]
    fn test_init_parametrised() {
        let mut surface = IgesSolidToroidalSurface::new();
        let center = IgesEntity::default();
        let axis = IgesEntity::default();
        let refdir = Some(IgesEntity::default());

        surface.init(center, axis, 5.0, 2.0, refdir);

        assert!(surface.is_parametrised());
        assert_eq!(surface.form_number(), 1);
        assert_eq!(surface.major_radius(), 5.0);
        assert_eq!(surface.minor_radius(), 2.0);
        assert!(surface.reference_dir().is_some());
    }

    #[test]
    fn test_radii_stored_correctly() {
        let mut surface = IgesSolidToroidalSurface::new();
        surface.init(
            IgesEntity::default(),
            IgesEntity::default(),
            10.5,
            3.2,
            None,
        );

        assert_eq!(surface.major_radius(), 10.5);
        assert_eq!(surface.minor_radius(), 3.2);
    }

    #[test]
    fn test_entities_stored_correctly() {
        let mut surface = IgesSolidToroidalSurface::new();
        let center = IgesEntity::default();
        let axis = IgesEntity::default();

        surface.init(center.clone(), axis.clone(), 5.0, 2.0, None);

        assert!(surface.center().is_some());
        assert!(surface.axis().is_some());
    }

    #[test]
    fn test_form_number_switches_on_refdir() {
        let mut surface = IgesSolidToroidalSurface::new();
        surface.init(
            IgesEntity::default(),
            IgesEntity::default(),
            5.0,
            2.0,
            None,
        );
        assert_eq!(surface.form_number(), 0);

        surface.init(
            IgesEntity::default(),
            IgesEntity::default(),
            5.0,
            2.0,
            Some(IgesEntity::default()),
        );
        assert_eq!(surface.form_number(), 1);
    }

    #[test]
    fn test_clone() {
        let mut surface = IgesSolidToroidalSurface::new();
        surface.init(
            IgesEntity::default(),
            IgesEntity::default(),
            5.0,
            2.0,
            Some(IgesEntity::default()),
        );

        let cloned = surface.clone();
        assert_eq!(cloned.major_radius(), 5.0);
        assert_eq!(cloned.minor_radius(), 2.0);
        assert!(cloned.is_parametrised());
    }
}
