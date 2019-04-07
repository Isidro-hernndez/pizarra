use graphics::math::Vec2d;
use super::DrawCommand;
use super::Shape;
use crate::serialize::Serialize;
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

impl Serialize for Rectangle {
    fn serialize(&self) -> String {
        if self.borders.is_none() {
            return String::new();
        }

        let borders = self.borders.unwrap();

        format!("<rect
           style=\"opacity:1;fill:none;fill-opacity:1;stroke:{:X};stroke-width:1;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1\"
           id=\"rect815\"
           x=\"{:.4}\"
           y=\"{:.4}\"
           width=\"{:.4}\"
           height=\"{:.4}\"
           />", self.color, borders[0], borders[1], borders[2], borders[3])
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
            }],
            None => Vec::new(),
        }
    }
}
