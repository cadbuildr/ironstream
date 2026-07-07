// FILE: osd_host.rs
// occt: OSD_Host

/// Host system information.
pub struct Host {
    hostname: String,
}

impl Host {
    pub fn new() -> Self {
        #[cfg(not(target_os = "windows"))]
        let hostname = "localhost".to_string();

        #[cfg(target_os = "windows")]
        let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "localhost".to_string());

        Self { hostname }
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

impl Default for Host {
    fn default() -> Self {
        Self::new()
    }
}
