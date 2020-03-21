use graphics::math::Vec2d;
use super::DrawCommand;
use super::Shape;
use crate::color::Color;

pub struct Line {
    points: Vec<Vec2d>,
    color: Color,
}

impl Line {
    pub fn new(color: Color) -> Line {
        Line {
            points: Vec::with_capacity(1000),
            color,
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
                color: self.color.to_a(),
                line: [*x1, *y1, *x2, *y2],
                relative_layer: 0,
            }
        }).collect()
    }
}
