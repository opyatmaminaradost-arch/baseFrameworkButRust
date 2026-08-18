#[derive(Clone)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_secs: u64,
}

impl ApiConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {  base_url: base_url.into(), 
                timeout_secs: 10,
        }
    }
}
