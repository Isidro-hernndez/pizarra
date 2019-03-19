use graphics::math::Vec2d;
use super::DrawCommand;
use super::Shape;
use crate::serialize::Serialize;

pub struct Line {
    points: Vec<Vec2d>,
    color: [f32; 4],
    thickness: f64,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            points: Vec::new(),
            color: [
                0xdd as f32/0xff as f32,
                0xfc as f32/0xff as f32,
                0xad as f32/0xff as f32,
                1.0,
            ],
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

impl Serialize for Line {
    fn serialize(&self) -> String {
        let contents: Vec<String> = self
            .points
            .iter()
            .map(|p| format!("{:.6},{:.6}", p[0], p[1]))
            .collect();

        format!("<path style=\"opacity:1;fill:none;fill-opacity:1;stroke:#000000;stroke-width:1;stroke-opacity:1;stroke-miterlimit:4;stroke-dasharray:none\" d=\"M {}\" id=\"path815\" />", contents.join(" "))
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
