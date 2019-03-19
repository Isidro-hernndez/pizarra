use graphics::math::Vec2d;
use super::DrawCommand;
use super::Shape;
use crate::serialize::Serialize;

pub struct Rectangle {
    borders: Option<[f64; 4]>,
    color: [f32; 4],
}

impl Default for Rectangle {
    fn default() -> Rectangle {
        Rectangle {
            borders: None,
            color: [
                0xdd as f32/0xff as f32,
                0xfc as f32/0xff as f32,
                0xad as f32/0xff as f32,
                1.0,
            ],
        }
    }
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
            ..Rectangle::default()
        }
    }
}

impl Serialize for Rectangle {
    fn serialize(&self) -> String {
        String::new()
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
                color: self.color,
                rect: bb,
            }],
            None => Vec::new(),
        }
    }
}
