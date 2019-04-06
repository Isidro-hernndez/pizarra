use graphics::math::Vec2d;
use super::serialize::Serialize;
use super::color::Color;

pub mod line;
pub mod rectangle;

pub use self::line::Line;
pub use self::rectangle::Rectangle;

pub enum Tool {
    Line,
    Rectangle,
}

impl Tool {
    pub fn make(&self, color: Color) -> Box<dyn Shape> {
        match *self {
            Tool::Line => Box::new(Line::new(color)),
            Tool::Rectangle => Box::new(Rectangle::new(color)),
        }
    }
}

pub enum DrawCommand {
    Line{
        color: [f32; 4],
        thickness: f64,
        line: [f64; 4],
    },
    Rectangle{
        color: [f32; 4],
        rect: [f64; 4],
    },
}

pub trait Shape: Serialize {
    fn handle(&mut self, val: Vec2d);
    // TODO review the posibility of sending an iterator to prevent the
    // allocation of a vector
    fn draw_commands(&self) -> Vec<DrawCommand>;
}
