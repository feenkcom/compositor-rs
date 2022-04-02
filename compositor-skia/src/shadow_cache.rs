use compositor::Shadow;
use skia_safe::Image;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

/// The amount of frames after which a cached shadow image is purged if not used
pub const CACHED_SHADOW_UNUSED_FRAMES_LIMIT: usize = 5;

pub struct CachedShadowImage {
    image: Image,
    frames_to_purge: usize,
}

impl CachedShadowImage {
    pub fn new(image: Image) -> Self {
        Self {
            image,
            frames_to_purge: CACHED_SHADOW_UNUSED_FRAMES_LIMIT,
        }
    }

    pub fn mark_not_used(&mut self) {
        self.frames_to_purge = max(self.frames_to_purge - 1, 0);
    }

    pub fn mark_used(&mut self) {
        self.frames_to_purge = min(self.frames_to_purge + 1, CACHED_SHADOW_UNUSED_FRAMES_LIMIT);
    }

    pub fn should_purge(&self) -> bool {
        self.frames_to_purge <= 0
    }
}

impl Debug for CachedShadowImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("CachedShadowImage")
            .field("frames_to_purge:", &self.frames_to_purge)
            .finish()
    }
}

pub struct ShadowCache {
    pub images: HashMap<Shadow, CachedShadowImage>,
}

impl Debug for ShadowCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_map()
            .entries(self.images.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}

impl ShadowCache {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }

    pub fn get_shadow_image(&mut self, shadow: &Shadow) -> Option<&Image> {
        self.images.get_mut(shadow).and_then(|cached_image| {
            cached_image.mark_used();
            Some(&cached_image.image)
        })
    }

    pub fn has_cached_shadow(&self, shadow: &Shadow) -> bool {
        self.images.contains_key(shadow)
    }

    pub fn count_cached_shadows(&self) -> usize {
        self.images.len()
    }

    pub fn push_shadow_image(&mut self, shadow: Shadow, image: Image) {
        self.images.insert(shadow, CachedShadowImage::new(image));
    }

    pub fn clear(&mut self) {
        self.images.clear();
    }

    pub fn mark_images_as_not_used(&mut self) {
        for cached_image in self.images.values_mut() {
            cached_image.mark_not_used();
        }
    }

    pub fn remove_unused_images(&mut self) -> usize {
        let size = self.images.len();
        self.images
            .retain(|_, cached_image| !cached_image.should_purge());
        size - self.images.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_hash_cached_shadow() {
        let cache = ShadowCache::new();

        let shadow = Shadow::default();

        assert_eq!(cache.has_cached_shadow(&shadow), false);
    }
}
