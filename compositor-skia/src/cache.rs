use crate::{ImageCache, ShadowCache};
use compositor::Shadow;
use log::info;
use skia_safe::{Image, Matrix};

#[derive(Debug)]
pub struct Cache {
    pub(crate) shadow_cache: ShadowCache,
    pub(crate) image_cache: ImageCache,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            shadow_cache: ShadowCache::new(),
            image_cache: ImageCache::new(),
        }
    }

    pub fn mark_images_as_not_used(&mut self) {
        self.image_cache.mark_images_as_not_used();
        self.shadow_cache.mark_images_as_not_used();
    }

    pub fn remove_unused_images(&mut self) {
        let removed_pictures = self.image_cache.remove_unused_images();
        let removed_shadows = self.shadow_cache.remove_unused_images();
        info!(
            "Removed {} unused cached pictures. {} left.",
            removed_pictures,
            self.image_cache.count_cached_images()
        );
        info!(
            "Removed {} unused cached shadows. {} left.",
            removed_shadows,
            self.shadow_cache.count_cached_shadows()
        );
    }

    pub fn get_shadow_image(&mut self, shadow: &Shadow) -> Option<&Image> {
        self.shadow_cache.get_shadow_image(shadow)
    }

    pub fn push_shadow_image(&mut self, shadow: Shadow, image: Image) {
        self.shadow_cache.push_shadow_image(shadow, image);
    }

    pub fn get_picture_image(&mut self, picture_id: u32) -> Option<(Image, Matrix)> {
        self.image_cache.get_picture_image(picture_id)
    }

    pub fn push_id_image(&mut self, picture_id: u32, image: Image, matrix: Matrix) {
        self.image_cache.push_id_image(picture_id, image, matrix);
    }
}
