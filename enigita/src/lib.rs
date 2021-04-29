//! An experiment to draw primitives on the PineTime, or any system were there is not enough memory
//! for a framebuffer, and where pixels need to be batched in rectangles for performance.
//!
//! The idea is that we have a stack of things to draw, bounding boxes so most inside/outside tests
//! are really quick, then pixel-by-pixel changes. There is also a global bounding box so we can do
//! incremental upgrades. The stack is walked for each pixel until a hit is found, which defines
//! the color.
//!
//! Use signed integers so we can have shapes that go offscreen. The offscreen bits won't be drawn.

#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![no_std]

use core::{marker::PhantomData, mem};
use embedded_graphics::pixelcolor::PixelColor;
use staticvec::StaticVec as Vec;

/// A frame to draw.
///
/// `LEN` is the maximum number of drawables that can be queued.
pub struct Frame<Color: 'static, const WIDTH: u16, const HEIGHT: u16, const LEN: usize> {
    /// This is the color that the pixel will be if nothing is over it.
    bg_color: Color,
    /// The area that needs to be redrawn.
    draw_area: Rect,
    /// The stack of things to draw, ordered last = top to first = bottom.
    draw_stack: Vec<Drawable<Color>, LEN>,
}

impl<Color, const WIDTH: u16, const HEIGHT: u16, const LEN: usize> Frame<Color, WIDTH, HEIGHT, LEN>
where
    Color: Copy + 'static,
{
    #[inline]
    pub fn new(background_color: Color) -> Self {
        Self {
            bg_color: background_color,
            draw_area: Rect::ZERO,
            draw_stack: Vec::new(),
        }
    }

    /// Remove all the queued shapes.
    ///
    /// Call this after drawing the frame to start a new frame.
    #[inline]
    pub fn new_frame(&mut self) -> &mut Self {
        self.draw_stack.clear();
        self.draw_area = Rect::ZERO;
        self
    }

    /// Returns the color that will be used as the background.
    #[inline]
    pub fn bg_color(&self) -> Color {
        self.bg_color
    }

    /// Sets the color that will be used as the background.
    #[inline]
    pub fn set_bg_color(&mut self, color: Color) -> &mut Self {
        self.bg_color = color;
        self
    }

    /// Overwrite the screen with the background color.
    ///
    /// It is best to avoid this if possible because it requires drawing the whole screen.
    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.draw_area = Rect::ZERO.with_size(Size::new(WIDTH, HEIGHT));
        self
    }

    /// Adds a shape to the draw list, on top of existing shapes.
    #[inline]
    pub fn draw<D>(&mut self, drawable: D) -> &mut Self
    where
        D: Into<Drawable<Color>>,
        Color: 'static,
    {
        let drawable = drawable.into();
        self.draw_area = self.draw_area.union(drawable.bounding_box());
        // this panics for now, maybe we want to return an error eventually.
        self.draw_stack.push(drawable);
        self
    }

    // methods to draw a frame

    /// The area that needs to be drawn
    #[inline]
    pub fn draw_area(&self) -> Rect {
        self.draw_area
    }

    /// Iterates over the pixels inside `self.draw_area()` in row-major order, left-to-right,
    /// top-to-bottom.
    pub fn render<'a>(&'a self) -> impl Iterator<Item = Color> + 'a {
        RenderFrame::new(self)
    }

    // helpers

    fn pixel_color(&self, pixel: Point) -> Color {
        self.draw_stack
            .iter()
            .filter_map(|d| d.pixel_color(pixel))
            .next_back()
            .unwrap_or(self.bg_color)
    }
}

struct RenderFrame<'a, Color: 'static, const WIDTH: u16, const HEIGHT: u16, const LEN: usize> {
    frame: &'a Frame<Color, WIDTH, HEIGHT, LEN>,
    pos: Point,
}

impl<'a, Color, const WIDTH: u16, const HEIGHT: u16, const LEN: usize>
    RenderFrame<'a, Color, WIDTH, HEIGHT, LEN>
{
    fn new(frame: &'a Frame<Color, WIDTH, HEIGHT, LEN>) -> Self {
        Self {
            frame,
            pos: frame.draw_area.top_left(),
        }
    }
}

impl<'a, Color, const WIDTH: u16, const HEIGHT: u16, const LEN: usize> Iterator
    for RenderFrame<'a, Color, WIDTH, HEIGHT, LEN>
where
    Color: Copy,
{
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frame.draw_area.y1 <= self.pos.y {
            return None;
        }
        let color = self.frame.pixel_color(self.pos);
        self.pos.x += 1;
        // check for new row
        if self.pos.x == self.frame.draw_area.x1 {
            self.pos.x = self.frame.draw_area.x0;
            self.pos.y += 1;
        }
        Some(color)
    }
}

pub enum Drawable<Color: 'static> {
    Shape(StyledShape<Color>),
    Image(Image<Color>),
}

impl<Color: 'static + Copy> Drawable<Color> {
    fn pixel_color(&self, pixel: Point) -> Option<Color> {
        match self {
            Drawable::Shape(s) => s.pixel_color(pixel),
            Drawable::Image(i) => i.pixel_color(pixel),
        }
    }

    fn bounding_box(&self) -> Rect {
        match self {
            Drawable::Shape(s) => s.bounding_box(),
            Drawable::Image(i) => i.bounding_box(),
        }
    }
}

impl<Color: 'static> From<StyledShape<Color>> for Drawable<Color> {
    fn from(shape: StyledShape<Color>) -> Self {
        Drawable::Shape(shape)
    }
}

pub struct StyledShape<Color: 'static> {
    fill_color: Color,
    shape: Shape,
}

impl<Color: 'static + Copy> StyledShape<Color> {
    fn pixel_color(&self, pixel: Point) -> Option<Color> {
        match self.shape {
            Shape::Rect(r) => {
                if r.hit_test(pixel) {
                    Some(self.fill_color)
                } else {
                    None
                }
            }
        }
    }

    fn bounding_box(&self) -> Rect {
        match self.shape {
            Shape::Rect(r) => r,
        }
    }
}

pub enum Shape {
    Rect(Rect),
}

impl Shape {
    #[inline]
    pub fn bounding_box(&self) -> Rect {
        match self {
            Shape::Rect(r) => (*r).bounding_box(),
        }
    }

    pub fn hit_test(&self, point: Point) -> bool {
        match self {
            Shape::Rect(r) => r.hit_test(point),
        }
    }

    pub fn style<Color>(self, fill_color: Color) -> StyledShape<Color> {
        StyledShape {
            fill_color,
            shape: self,
        }
    }
}

// Rect

/// A rectangle
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    /// top left x of the rectangle
    x0: i16,
    /// top left y of the rectangle
    y0: i16,
    /// bottom right x of the rectangle
    x1: i16,
    /// bottom right y of the rectangle
    y1: i16,
}

impl Rect {
    pub const ZERO: Rect = Rect {
        x0: 0,
        y0: 0,
        x1: 0,
        y1: 0,
    };

    /// Create a new `Rect`.
    ///
    /// Returns `None` if `x0 > x1` or `y0 > y1`.
    #[inline]
    pub fn new(x0: i16, y0: i16, x1: i16, y1: i16) -> Option<Self> {
        if x0 > x1 || y0 > y1 {
            None
        } else {
            Some(Self { x0, y0, x1, y1 })
        }
    }

    /// Create a new rect.
    ///
    /// This function does not check that `x1 >= x0` and `y1 >= y0`, which are assumed elsewhere in
    /// the code. Not satisfying this invariant will lead to unpredictable results.
    #[inline]
    pub fn new_unchecked(x0: i16, y0: i16, x1: i16, y1: i16) -> Self {
        Self { x0, y0, x1, y1 }
    }

    #[inline]
    pub fn x0(self) -> i16 {
        self.x0
    }

    #[inline]
    pub fn y0(self) -> i16 {
        self.y0
    }

    #[inline]
    pub fn x1(self) -> i16 {
        self.x1
    }

    #[inline]
    pub fn y1(self) -> i16 {
        self.y1
    }

    #[inline]
    pub fn top_left(self) -> Point {
        (self.x0, self.y0).into()
    }

    #[inline]
    pub fn bottom_right(self) -> Point {
        (self.x1, self.y1).into()
    }

    #[inline]
    pub fn with_origin(self, origin: Point) -> Self {
        Rect::from_origin_size(origin, self.size())
    }

    #[inline]
    pub fn with_size(self, size: Size) -> Self {
        Rect::from_origin_size(self.top_left(), size)
    }

    /// Warning: you will get garbage out if fields overflow, which won't happen unless your screen
    /// is massive (> 8000 pixels, say)
    #[inline]
    pub fn from_origin_size(origin: Point, size: Size) -> Self {
        debug_assert!(size.width < i16::MAX as u16 && size.height < i16::MAX as u16);
        Self {
            x0: origin.x,
            y0: origin.y,
            x1: size.width as i16 + origin.x,
            y1: size.height as i16 + origin.y,
        }
    }

    #[inline]
    pub fn size(self) -> Size {
        let Rect { x0, y0, x1, y1 } = self;
        // x1 >= x0 and y1 >= y0
        Size {
            width: (x1 - x0) as u16,
            height: (y1 - y0) as u16,
        }
    }

    #[inline]
    pub fn area(self) -> u16 {
        let size = self.size();
        size.width * size.height
    }

    #[inline]
    pub fn bounding_box(self) -> Rect {
        self
    }

    #[inline]
    pub fn hit_test(&self, point: Point) -> bool {
        self.x0 <= point.x && point.x < self.x1 && self.y0 <= point.y && point.y < self.y1
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x0 == self.x1 || self.y0 == self.y1
    }

    /// Return one of the smallest rectangles that contains the area of both parameter rectangles.
    ///
    /// The output is not necessarily unique.
    // I'm not super happy with the branching in this function. Maybe it would be better to
    // represent no rects as an Option::None rather than Rect::ZERO, but when I experimented things
    // got more complex.
    #[inline]
    pub fn union(self, other: Self) -> Self {
        if self.is_empty() {
            other
        } else if other.is_empty() {
            self
        } else {
            Self {
                x0: self.x0.min(other.x0),
                x1: self.x1.max(other.x1),
                y0: self.y0.min(other.y0),
                y1: self.y1.max(other.y1),
            }
        }
    }

    pub fn intersect(self, other: Self) -> Self {
        Self {
            x0: self.x0.max(other.x0),
            x1: self.x1.min(other.x1),
            y0: self.y0.max(other.y0),
            y1: self.y1.min(other.y1),
        }
    }

    #[inline]
    pub fn style<Color>(self, fill_color: Color) -> StyledShape<Color> {
        StyledShape {
            fill_color,
            shape: self.into(),
        }
    }
}

impl From<Rect> for Shape {
    fn from(rect: Rect) -> Self {
        Self::Rect(rect)
    }
}

// Circle - this is not included because drawing scanlines of circles is hard. Recommended way to
// draw circles is to render them to memory using 1 bit per pixel in an ordering suitable for fast
// conversion into scanlines.

/*
pub struct Circle {
    center: Point,
    radius: u16,
}

impl Ellipse {
    pub fn new(center: Point, radius: u16) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn set_center(&mut self, center: Point) -> &mut Self {
        self.center = center;
        self
    }

    pub fn radius(&self) -> u16 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: u16) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn hit_test(&self, point: Point) -> bool {}
}
*/

// Image

pub struct Image<C: 'static> {
    area: Rect,
    /// Require that images are available for the lifetime of the program (for now).
    buf: &'static ImageBuf<C>,
}

impl<Color: 'static> Image<Color> {
    fn pixel_color(&self, pixel: Point) -> Option<Color> {
        todo!()
    }

    fn bounding_box(&self) -> Rect {
        todo!()
    }
}

/// An image buffer with the color type, width and height specified at compiletime.
///
/// It is up to the caller of `new` to ensure that the data is a valid image.
pub struct ImageBuf<C: PixelColor, const WIDTH: usize, const HEIGHT: usize> {
    data: [u8; WIDTH * HEIGHT * mem::size_of::<C::Raw::Storage>()],
    color: PhantomData<C>,
}

impl<C: PixelColor, const WIDTH: usize, const HEIGHT: usize> ImageBuf<C, WIDTH, HEIGHT> {
    pub const fn new(data: [u8; WIDTH * HEIGHT * mem::size_of::<C::Raw::Storage>()]) -> Self {
        ImageBuf {
            data,
            color: PhantomData,
        }
    }
}

// Point

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
}

impl From<(i16, i16)> for Point {
    #[inline]
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

// Size

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size { width, height }
    }
}
