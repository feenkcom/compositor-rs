use std::ffi::c_void;

#[repr(C)]
pub struct Texture {
    backend: Backend,
    texture: *const c_void
}

#[repr(u8)]
pub enum Backend {
    Metal,
    Unsupported
}

#[cfg(target_os = "macos")]
impl Texture {
    pub fn new_metal(width: u32, height: u32) -> Self {

    }
}
