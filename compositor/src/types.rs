use euclid::{Point2D, Size2D, Vector2D};
use ordered_float::OrderedFloat;
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Neg};

pub type Scalar = OrderedFloat<f32>;

#[repr(transparent)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Rectangle(euclid::Rect<Scalar, Scalar>);

#[repr(transparent)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Point(euclid::Point2D<Scalar, Scalar>);

#[repr(transparent)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Radius((Scalar, Scalar));

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RoundedRectangle {
    rectangle: Rectangle,
    top_left_radius: Radius,
    top_right_radius: Radius,
    bottom_right_radius: Radius,
    bottom_left_radius: Radius,
}

impl RoundedRectangle {
    pub fn new(
        rectangle: Rectangle,
        top_left_radius: Radius,
        top_right_radius: Radius,
        bottom_right_radius: Radius,
        bottom_left_radius: Radius,
    ) -> Self {
        Self {
            rectangle,
            top_left_radius,
            top_right_radius,
            bottom_right_radius,
            bottom_left_radius,
        }
    }

    pub fn rectangle(&self) -> &Rectangle {
        &self.rectangle
    }

    pub fn radii(&self) -> [&Radius; 4] {
        [
            &self.top_left_radius,
            &self.top_right_radius,
            &self.bottom_right_radius,
            &self.bottom_left_radius,
        ]
    }

    pub fn translate(&self, offset: &Point) -> Self {
        let mut translated = self.clone();
        translated.rectangle = self.rectangle.translate(offset);
        translated
    }

    pub fn bounds(&self) -> Rectangle {
        self.rectangle.clone()
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Path(Box<dyn VectorPath>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Circle {
    center: Point,
    radius: Scalar,
}

impl Circle {
    pub fn new(center: impl Into<Point>, radius: impl Into<Scalar>) -> Self {
        Self {
            center: center.into(),
            radius: radius.into(),
        }
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn radius(&self) -> Scalar {
        self.radius
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.center.x() - self.radius,
            self.center.y() - self.radius,
            self.radius * 2.0.into(),
            self.radius * 2.0.into(),
        )
    }

    pub fn translate(&self, offset: &Point) -> Self {
        Self {
            center: &self.center + offset,
            radius: self.radius,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Color {
    argb: u32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Matrix([Scalar; 9usize]);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Geometry {
    None,
    Rectangle(Rectangle),
    RoundedRectangle(RoundedRectangle),
    Circle(Circle),
    Path(Path),
}

impl Default for Geometry {
    fn default() -> Self {
        Self::None
    }
}

impl Geometry {
    pub fn bounds(&self) -> Rectangle {
        match self {
            Geometry::None => Rectangle::zero(),
            Geometry::Rectangle(rectangle) => rectangle.clone(),
            Geometry::RoundedRectangle(rounded_rectangle) => rounded_rectangle.bounds(),
            Geometry::Circle(circle) => circle.bounds(),
            Geometry::Path(path) => path.bounds(),
        }
    }
}

impl Point {
    pub fn zero() -> Self {
        Self(euclid::Point2D::<Scalar, Scalar>::zero())
    }

    pub fn new(x: impl Into<Scalar>, y: impl Into<Scalar>) -> Self {
        Self(euclid::Point2D::<Scalar, Scalar>::new(x.into(), y.into()))
    }

    pub fn new_f32(x: f32, y: f32) -> Self {
        Self(euclid::Point2D::<Scalar, Scalar>::new(x.into(), y.into()))
    }

    pub fn x(&self) -> Scalar {
        self.0.x.into()
    }

    pub fn y(&self) -> Scalar {
        self.0.y.into()
    }

    pub fn as_tuple_f32(&self) -> (f32, f32) {
        (self.0.x.into(), self.0.y.into())
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(self.x().neg(), self.y().neg())
    }
}

impl From<Point> for Vector2D<Scalar, Scalar> {
    fn from(point: Point) -> Self {
        Vector2D::new(point.x(), point.y())
    }
}

impl From<&Point> for Vector2D<Scalar, Scalar> {
    fn from(point: &Point) -> Self {
        Vector2D::new(point.x(), point.y())
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Rectangle {
    pub fn zero() -> Self {
        Self(euclid::Rect::<Scalar, Scalar>::zero())
    }

    pub fn new(
        left: impl Into<Scalar>,
        top: impl Into<Scalar>,
        width: impl Into<Scalar>,
        height: impl Into<Scalar>,
    ) -> Self {
        Self(euclid::Rect::<Scalar, Scalar>::new(
            Point2D::new(left.into(), top.into()),
            Size2D::new(width.into(), height.into()),
        ))
    }

    pub fn extent(width: f32, height: f32) -> Self {
        Self::new(0.0, 0.0, width, height)
    }

    pub fn left(&self) -> Scalar {
        self.0.origin.x
    }

    pub fn top(&self) -> Scalar {
        self.0.origin.y
    }

    pub fn width(&self) -> Scalar {
        self.0.size.width
    }

    pub fn height(&self) -> Scalar {
        self.0.size.height
    }

    pub fn right(&self) -> Scalar {
        self.left() + self.width()
    }

    pub fn bottom(&self) -> Scalar {
        self.top() + self.height()
    }

    pub fn inflate(&self, width: Scalar, height: Scalar) -> Self {
        Self(self.0.inflate(width, height))
    }

    pub fn translate(&self, offset: &Point) -> Self {
        Self(self.0.translate(offset.into()))
    }
}

impl Matrix {
    pub fn from_9(buffer: [Scalar; 9usize]) -> Self {
        Self(buffer)
    }
    pub fn get_9(&self) -> &[Scalar; 9usize] {
        &self.0
    }
}

impl Radius {
    pub fn new(x: impl Into<Scalar>, y: impl Into<Scalar>) -> Self {
        Self((x.into(), y.into()))
    }

    pub fn width(&self) -> Scalar {
        self.0 .0
    }

    pub fn height(&self) -> Scalar {
        self.0 .1
    }

    pub fn as_tuple_f32(&self) -> (f32, f32) {
        (self.0 .0.into(), self.0 .1.into())
    }
}

impl Color {
    pub fn from_argb(argb: u32) -> Self {
        Self { argb }
    }

    pub fn as_argb(&self) -> u32 {
        self.argb
    }
}

impl Path {
    pub fn new(inner: Box<dyn VectorPath>) -> Self {
        Self(inner)
    }

    pub fn any(&self) -> &dyn Any {
        self.0.any()
    }

    pub fn bounds(&self) -> Rectangle {
        self.0.bounds()
    }
}

pub trait VectorPath: Debug {
    fn bounds(&self) -> Rectangle;
    fn clone_box(&self) -> Box<dyn VectorPath>;
    fn eq_box(&self, other: &Box<dyn VectorPath>) -> bool;
    fn hash_box(&self, state: &mut DefaultHasher);
    fn any(&self) -> &dyn Any;
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::default();
        self.0.hash_box(&mut hasher);
        state.write_u64(hasher.finish())
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_box(&other.0)
    }
}

impl Eq for Path {}
