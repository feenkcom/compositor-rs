use crate::{
    as_skia_point, into_skia_rect, into_skia_rrect, to_skia_point, PictureToRasterize, SkiaPath,
};
use compositor::{Geometry, Rectangle, Shadow};
use log::trace;
use skia_safe::image_filters::drop_shadow_only;
use skia_safe::paint::Style;
use skia_safe::{
    scalar, BlendMode, Canvas, ClipOp, Color, Image, Matrix, Paint, PathDirection, Point, Vector,
    M44,
};

pub(crate) fn clip_canvas(
    canvas: &mut Canvas,
    geometry: &Geometry,
    offset: Option<&compositor::Point>,
) {
    match geometry {
        Geometry::Rectangle(rectangle) => {
            let rectangle = offset.map_or(rectangle.clone(), |offset| rectangle.translate(offset));
            canvas.clip_rect(into_skia_rect(&rectangle), ClipOp::Intersect, true);
        }
        Geometry::Path(path) => {
            let skia_path = path
                .any()
                .downcast_ref::<SkiaPath>()
                .expect("Path is not Skia path!")
                .path();

            match offset {
                None => {
                    canvas.clip_path(skia_path, ClipOp::Intersect, true);
                }
                Some(offset) => {
                    canvas.clip_path(
                        &skia_path.with_offset(as_skia_point(offset).clone()),
                        ClipOp::Intersect,
                        true,
                    );
                }
            }
        }
        Geometry::RoundedRectangle(rounded_rectangle) => {
            let rounded_rectangle = offset.map_or(rounded_rectangle.clone(), |offset| {
                rounded_rectangle.translate(offset)
            });
            canvas.clip_rrect(into_skia_rrect(&rounded_rectangle), ClipOp::Intersect, true);
        }
        Geometry::None => {}
        Geometry::Circle(circle) => {
            let circle = offset.map_or(circle.clone(), |offset| circle.translate(offset));

            let path = skia_safe::Path::circle(
                as_skia_point(circle.center()).clone(),
                circle.radius().into(),
                PathDirection::CW,
            );
            canvas.clip_path(&path, ClipOp::Intersect, true);
        }
    }
}

/// Draw a given image with the following matrix
pub(crate) fn draw_image(
    canvas: &mut Canvas,
    image: &Image,
    matrix: &Matrix,
    cull_rectangle: &Rectangle,
) {
    let current_matrix = canvas.local_to_device_as_3x3();

    let device_bounds = PictureToRasterize::compute_device_bounds_rect(
        &into_skia_rect(cull_rectangle),
        &current_matrix,
    );

    canvas.save();

    let relative_matrix = Matrix::concat(&current_matrix, matrix.invert().as_ref().unwrap());

    let relative_bounds = PictureToRasterize::compute_device_bounds(
        &device_bounds.into(),
        &relative_matrix.invert().unwrap(),
    );

    canvas.reset_matrix();
    canvas.set_matrix(&M44::from(relative_matrix));

    let position = Point::new(relative_bounds.left as f32, relative_bounds.top as f32);
    trace!("Draw image at {:?}", &position);
    canvas.draw_image(image, position, None);
    canvas.restore();
}

pub(crate) fn draw_shadow(canvas: &mut Canvas, shadow: &Shadow, offset: Point) {
    trace!("Draw {:?}", shadow);

    let shadow_offset: Vector = offset + to_skia_point(shadow.inflation_offset());
    let shadow_radius: (scalar, scalar) = shadow.radius().as_tuple_f32();
    let shadow_color: Color = Color::new(shadow.color().as_argb());
    let stroke_width = if shadow_radius.0 > shadow_radius.1 {
        shadow_radius.0
    } else {
        shadow_radius.1
    };

    let drop_shadow_filter =
        drop_shadow_only(shadow_offset, shadow_radius, shadow_color, None, None);

    let mut shadow_paint = Paint::default();
    shadow_paint.set_style(Style::Stroke);
    shadow_paint.set_color(Color::WHITE);
    shadow_paint.set_anti_alias(true);
    shadow_paint.set_blend_mode(BlendMode::SrcOver);
    shadow_paint.set_stroke_width(stroke_width);
    shadow_paint.set_image_filter(drop_shadow_filter);

    draw_geometry(canvas, shadow.geometry(), &shadow_paint);
}

pub(crate) fn draw_geometry(canvas: &mut Canvas, geometry: &Geometry, paint: &Paint) {
    match geometry {
        Geometry::None => {}
        Geometry::Rectangle(rectangle) => {
            canvas.draw_rect(into_skia_rect(rectangle), &paint);
        }
        Geometry::RoundedRectangle(rounded_rectangle) => {
            canvas.draw_rrect(into_skia_rrect(rounded_rectangle), &paint);
        }
        Geometry::Path(path) => {
            let skia_path = path
                .any()
                .downcast_ref::<SkiaPath>()
                .expect("Path is not Skia path!");
            canvas.draw_path(skia_path.path(), &paint);
        }
        Geometry::Circle(circle) => {
            canvas.draw_circle(
                as_skia_point(circle.center()).clone(),
                circle.radius().into(),
                &paint,
            );
        }
    }
}
