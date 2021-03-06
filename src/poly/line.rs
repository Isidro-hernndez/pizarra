use graphics::math::Vec2d;
use super::DrawCommand;
use super::Shape;
use crate::serialize::Serialize;
use crate::color::Color;

pub struct Line {
    points: Vec<Vec2d>,
    color: Color,
    thickness: f64,
    id: usize,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            points: Vec::new(),
            color: Color::green(),
            thickness: 2.0,
            id: 1,
        }
    }
}

impl Line {
    pub fn new(color: Color, id: usize) -> Line {
        Line {
            points: Vec::with_capacity(1000),
            color,
            id,
            ..Line::default()
        }
    }
}

impl Serialize for Line {
    fn serialize(&self) -> String {
        let contents: Vec<String> = self
            .points
            .iter()
            .map(|p| format!("{:.6},{:.6}", p[0], p[1]))
            .collect();

        format!("<path
            id=\"path{}\"
            style=\"opacity:1;fill:none;fill-opacity:1;stroke:{:X};stroke-width:{};stroke-opacity:1;stroke-miterlimit:4;stroke-dasharray:none\"
            d=\"M {}\"
            />", self.id, self.color, self.thickness, contents.join(" "))
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
                thickness: self.thickness,
                line: [*x1, *y1, *x2, *y2],
            }
        }).collect()
    }
}
