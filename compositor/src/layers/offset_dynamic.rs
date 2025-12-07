use crate::{Compositor, Layer, Point};
use std::any::Any;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DynamicOffsetLayer {
    layers: Vec<Arc<dyn Layer>>,
    payload: Payload,
    offset_fn: unsafe extern "C" fn(*mut c_void) -> Point,
}

impl DynamicOffsetLayer {
    pub fn new(
        offset_fn: unsafe extern "C" fn(*mut c_void) -> Point,
        payload: *mut c_void,
        clone_fn: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        free_fn: unsafe extern "C" fn(*mut c_void),
    ) -> Self {
        Self {
            layers: vec![],
            payload: Payload {
                payload,
                clone_fn,
                free_fn,
            },
            offset_fn,
        }
    }

    pub fn offset(&self) -> Point {
        unsafe { (self.offset_fn)(self.payload.payload) }
    }
}

impl Layer for DynamicOffsetLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_dynamic_offset(self);
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn with_layers(&self, layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer> {
        Arc::new(Self {
            layers,
            payload: self.payload.clone(),
            offset_fn: self.offset_fn,
        })
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Payload {
    payload: *mut c_void,
    clone_fn: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    free_fn: unsafe extern "C" fn(*mut c_void),
}

impl Clone for Payload {
    fn clone(&self) -> Self {
        unsafe {
            Self {
                payload: (self.clone_fn)(self.payload),
                clone_fn: self.clone_fn,
                free_fn: self.free_fn,
            }
        }
    }
}

impl Drop for Payload {
    fn drop(&mut self) {
        unsafe {
            (self.free_fn)(self.payload);
            self.payload = null_mut();
        }
    }
}

unsafe impl Send for Payload {}
unsafe impl Sync for Payload {}
