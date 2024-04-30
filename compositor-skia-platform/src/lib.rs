#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub use platform_compositor::*;
#[cfg(target_os = "emscripten")]
pub use webgl::*;

#[cfg(feature = "angle")]
pub use self::angle::*;
#[cfg(feature = "d3d")]
pub use self::d3d::*;
#[cfg(all(feature = "egl", target_os = "android"))]
pub use self::egl_android::*;
#[cfg(all(feature = "egl", feature = "wayland"))]
pub use self::egl_wayland::*;
#[cfg(feature = "x11")]
pub use self::gl_x11::*;
#[cfg(target_os = "ios")]
pub use self::metal_ios::*;
#[cfg(target_os = "macos")]
pub use self::metal_macos::*;

#[cfg(target_os = "macos")]
pub mod metal_macos;

#[cfg(target_os = "ios")]
pub mod metal_ios;

#[cfg(feature = "d3d")]
pub mod d3d;

#[cfg(feature = "angle")]
pub mod angle;
#[cfg(feature = "angle")]
pub mod angle_utils;

mod platform_compositor;

#[cfg(all(feature = "egl", target_os = "android"))]
pub mod egl_android;
#[cfg(feature = "wayland")]
pub mod egl_wayland;
#[cfg(feature = "x11")]
pub mod gl_x11;

#[cfg(target_os = "emscripten")]
pub mod webgl;
#[cfg(target_os = "emscripten")]
pub mod webgl_utils;
