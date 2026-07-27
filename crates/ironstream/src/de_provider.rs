// FILE: rust/ironstream/crates/ironstream/src/de_provider.rs

// occt-ref: DE_ShapeFixParameters // format kinds — mirrors the well-known DE format tags
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeFormat {
    Step,
    Iges,
    Gltf,
    Obj,
    Vrml,
    Stl,
    Brep,
    Unknown,
}

impl DeFormat {
    /// Return the canonical uppercase name used in OCCT provider registries.
    pub fn name(&self) -> &'static str {
        match self {
            DeFormat::Step => "STEP",
            DeFormat::Iges => "IGES",
            DeFormat::Gltf => "GLTF",
            DeFormat::Obj => "OBJ",
            DeFormat::Vrml => "VRML",
            DeFormat::Stl => "STL",
            DeFormat::Brep => "BREP",
            DeFormat::Unknown => "UNKNOWN",
        }
    }

    /// Parse from a canonical name string (case-insensitive).
    pub fn from_name(s: &str) -> DeFormat {
        match s.to_uppercase().as_str() {
            "STEP" => DeFormat::Step,
            "IGES" => DeFormat::Iges,
            "GLTF" => DeFormat::Gltf,
            "OBJ" => DeFormat::Obj,
            "VRML" => DeFormat::Vrml,
            "STL" => DeFormat::Stl,
            "BREP" => DeFormat::Brep,
            _ => DeFormat::Unknown,
        }
    }
}

// occt: DE_Provider // capability flags — mirrors DE_Provider read/write modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeCapability {
    CanRead,
    CanWrite,
    CanReadWrite,
}

impl DeCapability {
    pub fn can_read(&self) -> bool {
        matches!(self, DeCapability::CanRead | DeCapability::CanReadWrite)
    }

    pub fn can_write(&self) -> bool {
        matches!(self, DeCapability::CanWrite | DeCapability::CanReadWrite)
    }
}

// occt-ref: DE_ConfigurationNode // params — per-format provider configuration
#[derive(Clone, Debug)]
pub struct DeProviderParams {
    format: DeFormat,
    file_extension: String,
    encoding: String,
    tolerance: f64,
}

impl DeProviderParams {
    /// Construct params with a format and file extension; encoding defaults to
    /// "UTF-8" and tolerance to 1e-7 (OCCT default precision).
    pub fn new(format: DeFormat, file_extension: &str) -> Self {
        DeProviderParams {
            format,
            file_extension: file_extension.to_string(),
            encoding: "UTF-8".to_string(),
            tolerance: 1e-7,
        }
    }

    pub fn format(&self) -> DeFormat {
        self.format
    }

    pub fn file_extension(&self) -> &str {
        &self.file_extension
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    pub fn set_encoding(&mut self, enc: &str) {
        self.encoding = enc.to_string();
    }
}

// occt: DE_Provider // stub — a registered provider bound to one format
#[derive(Clone, Debug)]
pub struct DeProvider {
    name: String,
    params: DeProviderParams,
    capability: DeCapability,
}

impl DeProvider {
    pub fn new(name: &str, params: DeProviderParams, capability: DeCapability) -> Self {
        DeProvider {
            name: name.to_string(),
            params,
            capability,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn format(&self) -> DeFormat {
        self.params.format()
    }

    pub fn can_read(&self) -> bool {
        self.capability.can_read()
    }

    pub fn can_write(&self) -> bool {
        self.capability.can_write()
    }

    /// Expose the underlying params for inspection or mutation.
    pub fn params(&self) -> &DeProviderParams {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut DeProviderParams {
        &mut self.params
    }

    pub fn capability(&self) -> DeCapability {
        self.capability
    }
}

// occt-ref: DE_Wrapper // — central registry of format providers
#[derive(Debug, Default)]
pub struct DeProviderRegistry {
    providers: Vec<DeProvider>,
}

impl DeProviderRegistry {
    pub fn new() -> Self {
        DeProviderRegistry {
            providers: Vec::new(),
        }
    }

    /// Register a provider. Later registrations for the same format do not
    /// replace earlier ones; `find_by_format` returns the first match.
    pub fn register(&mut self, p: DeProvider) {
        self.providers.push(p);
    }

    /// Find the first registered provider that handles `format`.
    pub fn find_by_format(&self, format: DeFormat) -> Option<&DeProvider> {
        self.providers.iter().find(|p| p.format() == format)
    }

    /// Find the first registered provider whose file extension matches `ext`
    /// (comparison is case-insensitive).
    pub fn find_by_extension(&self, ext: &str) -> Option<&DeProvider> {
        let ext_up = ext.to_uppercase();
        self.providers
            .iter()
            .find(|p| p.params().file_extension().to_uppercase() == ext_up)
    }

    pub fn nb_providers(&self) -> usize {
        self.providers.len()
    }
}
