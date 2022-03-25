use std::any::Any;
use std::slice;

pub fn as_skia_point(point: &compositor::Point) -> &skia_safe::Point {
    unsafe { &*(point as *const compositor::Point as *const skia_safe::Point) }
}

pub fn to_skia_point(point: compositor::Point) -> skia_safe::Point {
    unsafe { std::mem::transmute(point) }
}

pub fn into_skia_rect(rectangle: &compositor::Rectangle) -> skia_safe::Rect {
    skia_safe::Rect::new(
        rectangle.left().into(),
        rectangle.top().into(),
        rectangle.right().into(),
        rectangle.bottom().into(),
    )
}

pub fn into_skia_rrect(rounded_rectangle: &compositor::RoundedRectangle) -> skia_safe::RRect {
    let compositor_radii = rounded_rectangle.radii();
    let skia_radii = [
        skia_safe::Vector::from(compositor_radii[0].as_tuple_f32()),
        skia_safe::Vector::from(compositor_radii[1].as_tuple_f32()),
        skia_safe::Vector::from(compositor_radii[2].as_tuple_f32()),
        skia_safe::Vector::from(compositor_radii[3].as_tuple_f32()),
    ];

    skia_safe::RRect::new_rect_radii(&into_skia_rect(rounded_rectangle.rectangle()), &skia_radii)
}

pub fn to_compositor_color(color: skia_safe::Color) -> compositor::Color {
    unsafe { std::mem::transmute(color) }
}

pub fn into_skia_matrix(compositor_matrix: &compositor::Matrix) -> skia_safe::Matrix {
    let mut skia_matrix = skia_safe::Matrix::new_identity();

    let compositor_buffer = compositor_matrix.get_9();
    let ptr = compositor_buffer.as_ptr() as *const skia_safe::scalar;

    let buffer: &[skia_safe::scalar; 9] =
        unsafe { slice::from_raw_parts(ptr, compositor_buffer.len()) }
            .try_into()
            .expect("slice with incorrect length");

    skia_matrix.set_9(buffer);
    skia_matrix
}

pub fn to_compositor_rectangle(skia_rectangle: skia_safe::Rect) -> compositor::Rectangle {
    compositor::Rectangle::new(
        skia_rectangle.left,
        skia_rectangle.top,
        skia_rectangle.width(),
        skia_rectangle.height(),
    )
}

pub fn into_compositor_matrix(skia_matrix: &skia_safe::Matrix) -> compositor::Matrix {
    let mut skia_buffer: [skia_safe::scalar; 9] = [0.0; 9];
    skia_matrix.get_9(&mut skia_buffer);

    let compositor_buffer: [compositor::Scalar; 9] = unsafe { std::mem::transmute(skia_buffer) };
    compositor::Matrix::from_9(compositor_buffer.clone())
}

#[derive(Debug, Clone)]
pub struct SkiaPicture(skia_safe::Picture);

impl compositor::Picture for SkiaPicture {
    fn unique_id(&self) -> u32 {
        self.0.unique_id()
    }

    fn cull_rect(&self) -> compositor::Rectangle {
        to_compositor_rectangle(self.0.cull_rect())
    }

    fn any(&self) -> &dyn Any {
        &self.0
    }
}

impl SkiaPicture {
    pub fn new(picture: skia_safe::Picture) -> Self {
        Self(picture)
    }
}

impl From<skia_safe::Picture> for SkiaPicture {
    fn from(picture: skia_safe::Picture) -> Self {
        Self(picture)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let compositor_point = compositor::Point::new_f32(10.0, 20.0);

        let skia_point = as_skia_point(&compositor_point);

        assert_eq!(skia_point.x, compositor_point.x().into());
        assert_eq!(skia_point.y, compositor_point.y().into());
    }

    #[test]
    fn test_into_rectangle() {
        let compositor_rectangle = compositor::Rectangle::new(100.0, 200.0, 200.0, 300.0);
        let skia_rectangle = into_skia_rect(&compositor_rectangle);

        assert_eq!(skia_rectangle.left(), compositor_rectangle.left().into());
        assert_eq!(skia_rectangle.top(), compositor_rectangle.top().into());
        assert_eq!(skia_rectangle.width(), compositor_rectangle.width().into());
        assert_eq!(
            skia_rectangle.height(),
            compositor_rectangle.height().into()
        );
    }
}
