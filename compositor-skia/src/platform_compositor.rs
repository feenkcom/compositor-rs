use std::error::Error;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use skia_safe::{Color, Color4f, Font, FontMgr, FontStyle, ISize, Paint, Point, Surface};

use crate::{Cache, SkiaCachelessCompositor, SkiaCompositor};
use compositor::{Compositor, Layer};
use compositor_skia_platform::{Platform, PlatformContext};

lazy_static! {
    static ref FPS_FONT: Font = Font::new(
        FontMgr::new()
            .new_from_data(include_bytes!("../assets/SourceSans3-Regular.ttf"), None)
            .unwrap(),
        60.0
    );
    static ref FPS_PAINT: Paint = Paint::new(Color4f::from(Color::BLUE), None);
}

pub struct PlatformCompositor {
    platform: Platform,
    context: PlatformContext,
    latest_frame: Mutex<Option<Arc<dyn Layer>>>,
    cache: Cache,
    render_fps: Option<fps_counter::FPSCounter>,
    scale_factor: f32,
}

impl PlatformCompositor {
    pub fn new(platform: Platform, context: PlatformContext) -> Self {
        Self {
            platform,
            context,
            latest_frame: Mutex::new(None),
            cache: Cache::new(),
            render_fps: None,
            scale_factor: 1.0,
        }
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
    }

    /// Resize the surface we render on. Must only be called from the main thread
    pub fn resize_surface(&mut self, size: impl Into<ISize>) {
        self.context.resize_surface(size.into());
    }

    /// Submit the new layer to be rendered next. Can be called from any thread
    pub fn submit_layer(&mut self, layer: Arc<dyn Layer>) -> Result<(), Box<dyn Error>> {
        self.latest_frame
            .lock()
            .map(|mut frame| {
                frame.replace(layer);
            })
            .map_err(|error| format!("Failed to acquire Mutex lock: {}", error).into())
    }

    pub fn enable_fps(&mut self) {
        self.render_fps.replace(fps_counter::FPSCounter::default());
    }

    pub fn disable_fps(&mut self) {
        self.render_fps.take();
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        let current_layer = self
            .latest_frame
            .lock()
            .map_err(|error| -> Box<dyn Error> {
                format!("Failed to acquire Mutex lock: {}", error).into()
            })?
            .clone();

        if let Some(layer) = current_layer {
            self.context.with_surface(|surface| {
                let canvas = surface.canvas();
                canvas.clear(Color::WHITE);
                canvas.reset_matrix();
                canvas.scale((self.scale_factor, self.scale_factor));

                SkiaCompositor::new(Some(self.platform.clone()), canvas, &mut self.cache)
                    .compose(layer);

                self.render_fps.as_mut().map(|counter| {
                    canvas.draw_str(
                        &format!("{}", counter.tick()),
                        Point::new(20.0, 70.0),
                        &FPS_FONT,
                        &FPS_PAINT,
                    );
                });
            })
        }

        Ok(())
    }

    pub fn draw_cacheless(&mut self) -> Result<(), Box<dyn Error>> {
        let current_layer = self
            .latest_frame
            .lock()
            .map_err(|error| -> Box<dyn Error> {
                format!("Failed to acquire Mutex lock: {}", error).into()
            })?
            .clone();

        if let Some(layer) = current_layer {
            self.context.with_surface(|surface| {
                let canvas = surface.canvas();
                canvas.clear(Color::WHITE);

                SkiaCachelessCompositor::new(canvas).compose(layer);

                self.render_fps.as_mut().map(|counter| {
                    canvas.draw_str(
                        &format!("{}", counter.tick()),
                        Point::new(20.0, 70.0),
                        &FPS_FONT,
                        &FPS_PAINT,
                    );
                });
            })
        }

        Ok(())
    }
}
