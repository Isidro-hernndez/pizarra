use super::color::Color;
use graphics::math::{Vec2d, Matrix2d};
use graphics::Graphics;

pub trait Shape {
    fn handle(&mut self, val: Vec2d);
    fn draw<G>(&self, t: Matrix2d, g: &mut G)
        where G: Graphics;
}

pub struct Line {
    points: Vec<Vec2d>,
    pub drawn: bool,
    color: [f32; 4],
    thickness: f64,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            points: Vec::new(),
            drawn: false,
            color: Color::green().to_a(),
            thickness: 1.0,
        }
    }
}

impl Line {
    pub fn new() -> Line {
        Line {
            points: Vec::with_capacity(1000),
            drawn: false,
            ..Line::default()
        }
    }

    fn push(&mut self, val: Vec2d) {
        self.points.push(val);
    }
}

impl Shape for Line {
    fn handle(&mut self, val: Vec2d) {
        self.push(val);
    }

    fn draw<G>(&self, t: Matrix2d, g: &mut G)
        where G: Graphics
    {
        for ([x1, y1], [x2, y2]) in self.points.iter().zip(self.points.iter().skip(1)) {
            graphics::line(
                self.color,
                self.thickness,
                [*x1, *y1, *x2, *y2],
                t, g
            );
        }
    }
}
