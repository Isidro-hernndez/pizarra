use graphics::math::Vec2d;
use super::DrawCommand;
use super::Shape;
use crate::color::Color;

pub struct Rectangle {
    borders: Option<[f64; 4]>,
    color: Color,
}

impl Default for Rectangle {
    fn default() -> Rectangle {
        Rectangle {
            borders: None,
            color: Color::green(),
        }
    }
}

impl Rectangle {
    pub fn new(color: Color) -> Rectangle {
        Rectangle {
            color,
            ..Rectangle::default()
        }
    }
}

impl Shape for Rectangle {
    fn handle(&mut self, val: Vec2d) {
        match self.borders.as_mut() {
            Some(bb) => {
                self.borders = Some([bb[0], bb[1], val[0]-bb[0], val[1]-bb[1]]);
            },
            None => {
                self.borders = Some([val[0], val[1], 0.0, 0.0]);
            },
        }
    }

    fn draw_commands(&self) -> Vec<DrawCommand> {
        match self.borders {
            Some(bb) => vec![DrawCommand::Rectangle{
                color: self.color.to_a(),
                rect: bb,
                relative_layer: 0,
            }],
            None => Vec::new(),
        }
    }
}
