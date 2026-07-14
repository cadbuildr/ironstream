// FILE: de_format_ext.rs
// occt-ref: DE_Format, DE_Provider, DE_Wrapper, DE_ShapeFixParameters

/// CAD file format identifier.
// occt-ref: DE_Format
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum DeFormat {
    #[default]
    Unknown,
    Step,
    Iges,
    Brep,
    Stl,
    Obj,
    Gltf,
    Vrml,
    Amf,
    Dxf,
}

impl DeFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Step  => "stp",
            Self::Iges  => "igs",
            Self::Brep  => "brep",
            Self::Stl   => "stl",
            Self::Obj   => "obj",
            Self::Gltf  => "gltf",
            Self::Vrml  => "wrl",
            Self::Amf   => "amf",
            Self::Dxf   => "dxf",
            Self::Unknown => "",
        }
    }

    /// Detect format from file name extension (case-insensitive).
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "stp" | "step" => Self::Step,
            "igs" | "iges" => Self::Iges,
            "brep" | "brp" => Self::Brep,
            "stl"          => Self::Stl,
            "obj"          => Self::Obj,
            "gltf" | "glb" => Self::Gltf,
            "wrl" | "vrml" => Self::Vrml,
            "amf"          => Self::Amf,
            "dxf"          => Self::Dxf,
            _              => Self::Unknown,
        }
    }

    pub fn from_path(path: &str) -> Self {
        let ext = path.rsplit('.').next().unwrap_or("");
        Self::from_extension(ext)
    }

    pub fn is_known(&self) -> bool { *self != Self::Unknown }
}

/// Read/write capability flags for a provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DeProviderCaps {
    pub can_read: bool,
    pub can_write: bool,
    pub supports_assembly: bool,
    pub supports_bspline: bool,
}

/// A format provider stub.
// occt-ref: DE_Provider
#[derive(Clone, Debug)]
pub struct DeProvider {
    pub format: DeFormat,
    pub name: String,
    pub caps: DeProviderCaps,
    pub priority: i32,
}

impl DeProvider {
    pub fn new(format: DeFormat, name: impl Into<String>) -> Self {
        Self {
            format,
            name: name.into(),
            caps: DeProviderCaps { can_read: true, can_write: true, supports_assembly: true, supports_bspline: true },
            priority: 0,
        }
    }

    pub fn can_read(&self) -> bool { self.caps.can_read }
    pub fn can_write(&self) -> bool { self.caps.can_write }

    /// Stub read: always returns Ok(0) (no shapes loaded).
    pub fn read(&self, path: &str) -> Result<usize, String> {
        if self.caps.can_read {
            let _ = path;
            Ok(0)
        } else {
            Err(format!("{} does not support read", self.name))
        }
    }

    /// Stub write: always returns Ok(()).
    pub fn write(&self, path: &str, nb_shapes: usize) -> Result<(), String> {
        if self.caps.can_write {
            let _ = (path, nb_shapes);
            Ok(())
        } else {
            Err(format!("{} does not support write", self.name))
        }
    }
}

/// Registry of format providers.
// occt-ref: DE_Wrapper
#[derive(Clone, Debug, Default)]
pub struct DeWrapper {
    pub providers: Vec<DeProvider>,
}

impl DeWrapper {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, p: DeProvider) { self.providers.push(p); }

    pub fn find(&self, format: DeFormat) -> Option<&DeProvider> {
        self.providers.iter()
            .filter(|p| p.format == format)
            .max_by_key(|p| p.priority)
    }

    pub fn find_by_path(&self, path: &str) -> Option<&DeProvider> {
        self.find(DeFormat::from_path(path))
    }

    pub fn nb_providers(&self) -> usize { self.providers.len() }

    pub fn supported_formats(&self) -> Vec<DeFormat> {
        let mut v: Vec<DeFormat> = self.providers.iter().map(|p| p.format).collect();
        v.sort_by_key(|f| *f as u8);
        v.dedup();
        v
    }
}

/// Shape fix parameters passed to provider on import.
// occt-ref: DE_ShapeFixParameters
#[derive(Clone, Debug, PartialEq)]
pub struct DeShapeFixParameters {
    pub fix_shape: bool,
    pub fix_tolerance: f64,
    pub sew_shells: bool,
    pub make_solids: bool,
    pub max_tolerance: f64,
}

impl Default for DeShapeFixParameters {
    fn default() -> Self {
        Self {
            fix_shape: true,
            fix_tolerance: 1e-6,
            sew_shells: true,
            make_solids: false,
            max_tolerance: 1e-3,
        }
    }
}

impl DeShapeFixParameters {
    pub fn new() -> Self { Self::default() }
    pub fn with_fix_tolerance(mut self, tol: f64) -> Self { self.fix_tolerance = tol; self }
    pub fn with_sew_shells(mut self, v: bool) -> Self { self.sew_shells = v; self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_from_extension() {
        assert_eq!(DeFormat::from_extension("step"), DeFormat::Step);
        assert_eq!(DeFormat::from_extension("STP"), DeFormat::Step);
        assert_eq!(DeFormat::from_extension("iges"), DeFormat::Iges);
        assert_eq!(DeFormat::from_extension("xyz"), DeFormat::Unknown);
    }

    #[test]
    fn format_from_path() {
        assert_eq!(DeFormat::from_path("model.brep"), DeFormat::Brep);
        assert_eq!(DeFormat::from_path("out.gltf"), DeFormat::Gltf);
    }

    #[test]
    fn wrapper_register_find() {
        let mut w = DeWrapper::new();
        let p = DeProvider::new(DeFormat::Step, "StepProvider");
        w.register(p);
        assert!(w.find(DeFormat::Step).is_some());
        assert!(w.find(DeFormat::Iges).is_none());
        assert_eq!(w.nb_providers(), 1);
    }

    #[test]
    fn wrapper_find_by_path() {
        let mut w = DeWrapper::new();
        w.register(DeProvider::new(DeFormat::Stl, "StlProvider"));
        let p = w.find_by_path("mesh.stl");
        assert!(p.is_some());
        assert_eq!(p.unwrap().format, DeFormat::Stl);
    }

    #[test]
    fn provider_read_write_stubs() {
        let p = DeProvider::new(DeFormat::Obj, "ObjProvider");
        assert!(p.read("file.obj").is_ok());
        assert!(p.write("out.obj", 3).is_ok());
    }

    #[test]
    fn shape_fix_defaults() {
        let s = DeShapeFixParameters::default();
        assert!(s.fix_shape);
        assert!(s.sew_shells);
        let s2 = s.with_fix_tolerance(1e-4).with_sew_shells(false);
        assert!(!s2.sew_shells);
        assert!((s2.fix_tolerance - 1e-4).abs() < 1e-15);
    }
}
