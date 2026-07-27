// FILE: osd_env.rs
// occt: OSD_Environment, OSD_Process, OSD_Directory, OSD_Path

/// An OS environment variable entry.
/// occt: OSD_Environment
#[derive(Clone, Debug)]
pub struct OsdEnvironment {
    pub name: String,
    pub value: String,
    pub is_set: bool,
}

impl OsdEnvironment {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), value: String::new(), is_set: false }
    }

    pub fn with_value(name: &str, value: &str) -> Self {
        Self { name: name.to_string(), value: value.to_string(), is_set: true }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn value(&self) -> &str { &self.value }
    pub fn is_set(&self) -> bool { self.is_set }
    pub fn set_value(&mut self, v: &str) { self.value = v.to_string(); self.is_set = true; }
    pub fn clear(&mut self) { self.value.clear(); self.is_set = false; }
}

/// Process information stub.
/// occt: OSD_Process
#[derive(Clone, Debug)]
pub struct OsdProcess {
    pub pid: u32,
    pub user_name: String,
    pub current_dir: String,
}

impl Default for OsdProcess {
    fn default() -> Self {
        Self { pid: 1, user_name: String::from("user"), current_dir: String::from("/") }
    }
}

impl OsdProcess {
    pub fn new() -> Self { Self::default() }

    pub fn pid(&self) -> u32 { self.pid }
    pub fn user_name(&self) -> &str { &self.user_name }
    pub fn current_directory(&self) -> &str { &self.current_dir }

    pub fn nb_processors(&self) -> usize { 1 }

    pub fn is_terminated(&self) -> bool { false }
}

/// A filesystem path.
/// occt: OSD_Path
#[derive(Clone, Debug, Default)]
pub struct OsdPath {
    pub node: String,
    pub username: String,
    pub password: String,
    pub disk: String,
    pub trek: String,
    pub name: String,
    pub extension: String,
}

impl OsdPath {
    pub fn new() -> Self { Self::default() }

    pub fn from_str(path: &str) -> Self {
        let mut p = Self::default();
        if let Some(dot) = path.rfind('.') {
            p.extension = path[dot+1..].to_string();
            p.name = path[..dot].rsplit('/').next().unwrap_or("").to_string();
        } else {
            p.name = path.rsplit('/').next().unwrap_or("").to_string();
        }
        if let Some(slash) = path.rfind('/') {
            p.trek = path[..=slash].to_string();
        }
        p
    }

    pub fn system_path(&self) -> String {
        if self.extension.is_empty() {
            format!("{}{}", self.trek, self.name)
        } else {
            format!("{}{}.{}", self.trek, self.name, self.extension)
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn extension(&self) -> &str { &self.extension }
    pub fn trek(&self) -> &str { &self.trek }
}

/// Directory entry.
/// occt: OSD_Directory // / OSD_DirectoryIterator (stub)
#[derive(Clone, Debug)]
pub struct OsdDirectory {
    pub path: String,
    pub exists: bool,
}

impl OsdDirectory {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string(), exists: !path.is_empty() }
    }

    pub fn exists(&self) -> bool { self.exists }
    pub fn path(&self) -> &str { &self.path }

    pub fn build(&mut self) -> bool {
        self.exists = !self.path.is_empty();
        self.exists
    }

    /// Stub: pretend we got these sub-entries.
    pub fn sub_dirs(&self) -> Vec<String> {
        if !self.exists { return Vec::new(); }
        vec![format!("{}/subdir_a", self.path), format!("{}/subdir_b", self.path)]
    }
}

/// Environment variable table (registry).
#[derive(Clone, Debug, Default)]
pub struct OsdEnvTable {
    pub vars: Vec<OsdEnvironment>,
}

impl OsdEnvTable {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, name: &str, value: &str) {
        if let Some(v) = self.vars.iter_mut().find(|e| e.name == name) {
            v.set_value(value);
        } else {
            self.vars.push(OsdEnvironment::with_value(name, value));
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.vars.iter().find(|e| e.name == name && e.is_set).map(|e| e.value.as_str())
    }

    pub fn unset(&mut self, name: &str) {
        if let Some(v) = self.vars.iter_mut().find(|e| e.name == name) {
            v.clear();
        }
    }

    pub fn nb_vars(&self) -> usize { self.vars.iter().filter(|e| e.is_set).count() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_var_set_get() {
        let mut e = OsdEnvironment::new("MY_VAR");
        assert!(!e.is_set());
        e.set_value("hello");
        assert!(e.is_set());
        assert_eq!(e.value(), "hello");
        e.clear();
        assert!(!e.is_set());
    }

    #[test]
    fn process_defaults() {
        let p = OsdProcess::new();
        assert!(p.pid() > 0);
        assert!(!p.is_terminated());
        assert!(p.nb_processors() >= 1);
    }

    #[test]
    fn osd_path_parse() {
        let p = OsdPath::from_str("/home/user/model.step");
        assert_eq!(p.extension(), "step");
        assert_eq!(p.name(), "model");
        assert!(p.system_path().contains("model.step"));
    }

    #[test]
    fn osd_directory_sub_dirs() {
        let d = OsdDirectory::new("/tmp/test");
        assert!(d.exists());
        let subs = d.sub_dirs();
        assert_eq!(subs.len(), 2);
    }

    #[test]
    fn env_table_set_get_unset() {
        let mut t = OsdEnvTable::new();
        t.set("PATH", "/usr/bin");
        t.set("HOME", "/home/user");
        assert_eq!(t.get("PATH"), Some("/usr/bin"));
        assert_eq!(t.nb_vars(), 2);
        t.unset("PATH");
        assert!(t.get("PATH").is_none());
        assert_eq!(t.nb_vars(), 1);
    }
}
