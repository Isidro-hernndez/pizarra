use super::color::Color;
use graphics::math::Vec2d;

pub enum DrawCommand {
    Line{
        color: [f32; 4],
        thickness: f64,
        line: [f64; 4],
    },
}

pub trait Shape {
    fn handle(&mut self, val: Vec2d);
    // TODO review the posibility of sending an iterator to prevent the
    // allocation of a vector
    fn draw_commands(&self) -> Vec<DrawCommand>;
}

pub struct Line {
    points: Vec<Vec2d>,
    color: [f32; 4],
    thickness: f64,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            points: Vec::new(),
            color: Color::green().to_a(),
            thickness: 1.0,
        }
    }
}

impl Line {
    pub fn new() -> Line {
        Line {
            points: Vec::with_capacity(1000),
            ..Line::default()
        }
    }
}

impl Shape for Line {
    fn handle(&mut self, val: Vec2d) {
        self.points.push(val);
    }

    fn draw_commands(&self) -> Vec<DrawCommand> {
        self.points.iter().zip(self.points.iter().skip(1)).map(|([x1, y1], [x2, y2])| {
            DrawCommand::Line {
                color: self.color,
                thickness: self.thickness,
                line: [*x1, *y1, *x2, *y2],
            }
        }).collect()
    }
}
