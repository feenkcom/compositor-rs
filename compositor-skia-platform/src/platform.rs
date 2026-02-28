use std::ffi::c_void;

#[derive(Debug, Clone)]
pub enum Platform {
    #[cfg(target_os = "macos")]
    Metal(crate::MetalPlatform),
    #[cfg(all(feature = "angle", target_os = "windows"))]
    Angle(crate::AnglePlatform),
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

    pub fn try_as_egl_handles(&self) -> Option<(*mut c_void, *mut c_void)> {
        match self {
            #[cfg(all(feature = "angle", target_os = "windows"))]
            Platform::Angle(platform) => Some((platform.display, platform.context)),
            _ => None,
        }
    }
}
