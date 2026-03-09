use std::ffi::c_void;

#[derive(Debug, Clone)]
pub enum Platform {
    #[cfg(target_os = "macos")]
    Metal(crate::MetalPlatform),
    #[cfg(target_os = "windows")]
    Angle(crate::OpenGLPlatform),
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

    pub fn try_as_opengl_platform(&self) -> Option<&crate::OpenGLPlatform> {
        match self {
            #[cfg(target_os = "windows")]
            Platform::Angle(platform) => Some(platform),
            _ => None,
        }
    }
}
