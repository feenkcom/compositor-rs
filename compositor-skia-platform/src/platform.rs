#[derive(Debug, Clone)]
pub enum Platform {
    #[cfg(target_os = "macos")]
    Metal(crate::MetalPlatform),
    Unsupported,
}

impl Platform {
    #[cfg(target_os = "macos")]
    pub fn try_as_metal_platform(&self) -> Option<&crate::MetalPlatform> {
        match self {
            Platform::Metal(platform) => Some(platform),
            _ => None,
        }
    }
}
