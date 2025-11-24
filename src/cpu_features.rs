pub struct CpuFeatures {
    pub name: String,
    pub supported: bool,
}

impl CpuFeatures {
    pub fn new(name: &str, supported: bool) -> Self {
        Self {
            name: name.to_string(),
            supported,
        }
    }

    pub fn format(&self, width: usize) -> String {
        let status = if self.supported {
            "YES!"
        } else {
            "NO!"
        };
        format!("{:width$} Supported: {}", self.name, status, width = width)
    }
}