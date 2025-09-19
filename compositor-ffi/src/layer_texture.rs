use compositor::{BorrowedTexture, Layer, Texture, TextureLayer};
use compositor_texture::TextureDesc;
use std::sync::Arc;
use value_box::ValueBox;

#[no_mangle]
pub fn compositor_texture_layer_new_borrowed(
    width: u32,
    height: u32,
    rendering: extern "C" fn(*const TextureDesc, *const std::ffi::c_void),
    payload: *const std::ffi::c_void,
) -> *mut ValueBox<Arc<dyn Layer>> {
    let rendering_fn = move |texture, payload| {
        let desc = Box::into_raw(Box::new(texture));
        rendering(desc, payload);
        unsafe {
            let _ = Box::from_raw(desc);
        };
    };

    ValueBox::new(Arc::new(TextureLayer::new(
        width,
        height,
        Texture::Borrowed(BorrowedTexture {
            rendering: Arc::new(rendering_fn),
            payload,
        }),
    )) as Arc<dyn Layer>)
    .into_raw()
}
