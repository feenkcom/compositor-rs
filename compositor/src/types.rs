use euclid::{Point2D, Size2D, Vector2D};
use ordered_float::OrderedFloat;
use std::any::Any;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Add;

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
pub struct RoundedRectangle {}

#[repr(transparent)]
#[derive(Debug)]
pub struct Path(Box<dyn VectorPath>);

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
            Geometry::RoundedRectangle(_) => {
                todo!()
            }
            Geometry::Path(path) => path.bounds(),
        }
    }
}

impl Point {
    pub fn zero() -> Self {
        Self(euclid::Point2D::<Scalar, Scalar>::zero())
    }

    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self(euclid::Point2D::<Scalar, Scalar>::new(x, y))
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

impl Rectangle {
    pub fn zero() -> Self {
        Self(euclid::Rect::<Scalar, Scalar>::zero())
    }

    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
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
    pub fn new(x: f32, y: f32) -> Self {
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
    fn hash_box(&self, state: &mut dyn Hasher);
    fn any(&self) -> &dyn Any;
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash_box(state);
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_box(&other.0)
    }
}

impl Eq for Path {}
