use graphics::math::Vec2d;

pub mod line;
pub mod rectangle;
pub mod circle;

use super::color::Color;

pub use self::line::Line;
pub use self::rectangle::Rectangle;
pub use self::circle::Circle;

pub enum Tool {
    Line,
    Rectangle,
    Circle,
}

impl Tool {
    pub fn make(&self, color: Color) -> Box<dyn Shape> {
        match *self {
            Tool::Line => Box::new(Line::new(color)),
            Tool::Rectangle => Box::new(Rectangle::new(color)),
            Tool::Circle => Box::new(Circle::new(color)),
        }
    }
}

pub enum DrawCommand {
    Line{
        color: [f32; 4],
        line: [f64; 4],
        relative_layer: i32,
    },
    Rectangle{
        color: [f32; 4],
        rect: [f64; 4],
        relative_layer: i32,
    },
    Circle{
        color: [f32; 4],
        rect: [f64; 4],
        relative_layer: i32,
    },
}

pub trait Shape {
    /// Must handle new coordinates given to this shape. If this method is
    /// called it means that the shape is being modified (thus this is the most
    /// recently added shape
    fn handle(&mut self, val: Vec2d);

    fn draw_commands(&self) -> Vec<DrawCommand>;
}
