
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Protected {
    No = 0,
    Yes = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ColorType {
    Unknown,
    /// Four channel RGBA data (8 bits per channel) packed into a LE 32-bit word.
    /// Bits: [A:31..24 B:23..16 G:15..8 R:7..0]
    RGBA8888,
    /// Three channel RGB data (8 bits per channel) packed into a LE 32-bit word. The remaining bits
    /// are ignored and alpha is forced to opaque.
    /// Bits: [x:31..24 B:23..16 G:15..8 R:7..0]
    RGB888x,
    /// Four channel BGRA data (8 bits per channel) packed into a LE 32-bit word. R and B are swapped
    /// relative to RGBA8888.
    /// Bits: [A:31..24 R:23..16 G:15..8 B:7..0]
    BGRA8888,
}

#[macro_export]
macro_rules! encode_skia_protected {
    ($protected:expr) => {{
        use skia_safe::gpu::Protected as SkiaProtected;
        use compositor_texture::Protected as TextureProtected;

        match $protected {
            SkiaProtected::No => TextureProtected::No,
            SkiaProtected::Yes => TextureProtected::Yes,
        }
    }};
}

#[macro_export]
macro_rules! decode_skia_protected {
    ($protected:expr) => {{
        use skia_safe::gpu::Protected as SkiaProtected;
        use compositor_texture::Protected as TextureProtected;

        match $protected {
            TextureProtected::No => SkiaProtected::No,
            TextureProtected::Yes => SkiaProtected::Yes,
        }
    }};
}

#[macro_export]
macro_rules! encode_skia_color_type {
    ($color_type:expr) => {{
        use skia_safe::ColorType as SkiaColorType;
        use compositor_texture::ColorType as TextureColorType;

        match $color_type {
            SkiaColorType::Unknown => TextureColorType::Unknown,
            SkiaColorType::RGBA8888 => TextureColorType::RGBA8888,
            SkiaColorType::RGB888x => TextureColorType::RGB888x,
            SkiaColorType::BGRA8888 => TextureColorType::BGRA8888,
            _ => TextureColorType::Unknown,
        }
    }};
}

#[macro_export]
macro_rules! decode_skia_color_type {
    ($color_type:expr) => {{
        use skia_safe::ColorType as SkiaColorType;
        use compositor_texture::ColorType as TextureColorType;

        match $color_type {
            TextureColorType::Unknown => SkiaColorType::Unknown,
            TextureColorType::RGBA8888 => SkiaColorType::RGBA8888,
            TextureColorType::RGB888x => SkiaColorType::RGB888x,
            TextureColorType::BGRA8888 => SkiaColorType::BGRA8888,
            _ => SkiaColorType::Unknown,
        }
    }};
}