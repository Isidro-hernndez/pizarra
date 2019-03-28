use opengl_graphics::GlGraphics;
use graphics::math;
use piston::input::RenderArgs;

use crate::poly::DrawCommand;
use crate::storage::ShapeStorage;
use crate::{Tool, Pizarra};
use crate::color::Color;
use crate::serialize::Serialize;

pub struct App {
    is_drawing: bool,
    is_moving: bool,
    storage: ShapeStorage,
    selected_tool: Tool,
    gl: GlGraphics,
    piz: Pizarra,
}

impl App {
    pub fn new(gl: GlGraphics, piz: Pizarra) -> App {
        App {
            is_drawing: false,
            is_moving: false,
            storage: ShapeStorage::new(),
            selected_tool: Tool::Line,
            gl,
            piz,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let bgcolor = Color::black().to_a();
        let guidecolor = Color::gray().to_a();
        let offset = self.piz.get_offset_t();
        let mut commands = Vec::new();

        for item in self.storage.iter() {
            commands.extend(item.draw_commands().into_iter());
        }

        self.gl.draw(args.viewport(), |c, g| {
            let t = math::multiply(c.transform, offset);

            graphics::clear(bgcolor, g);

            // content
            for cmd in commands {
                match cmd {
                    DrawCommand::Line{
                        color, thickness, line,
                    } => graphics::line(color, thickness, line, t, g),
                    DrawCommand::Rectangle{
                        color, rect,
                    } => graphics::rectangle(color, rect, t, g),
                }
            }

            // UI
            graphics::line(guidecolor, 1.0, [-20.0, 0.0, 20.0, 0.0], t, g);
            graphics::line(guidecolor, 1.0, [0.0, 20.0, 0.0, -20.0], t, g);
        });
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.selected_tool = tool;
    }

    pub fn start_drawing(&mut self) {
        self.is_drawing = true;

        self.storage.add(self.selected_tool.make());
    }

    pub fn finish_drawing(&mut self) {
        self.is_drawing = false;
    }

    pub fn start_moving(&mut self) {
        self.is_moving = true;
    }

    pub fn end_moving(&mut self) {
        self.is_moving = false;
    }

    pub fn handle_cursor(&mut self, x: f64, y: f64) {
        if self.is_drawing && !self.is_moving {
            if let Some(item) = self.storage.last_mut() {
                item.handle(math::transform_pos(self.piz.get_inv_offset(), [x, y]));
            }
        }
    }

    pub fn update_offset(&mut self, dx: f64, dy: f64) {
        self.piz.delta_offset([dx, dy]);
    }

    pub fn resize(&mut self, w: f64, h: f64) {
        self.piz.resize([w, h]);
    }

    pub fn serialize(&self) -> String {
        self.storage.serialize()
    }

    pub fn undo(&mut self) {
        self.piz.undo();
        // TODO undo() on piz must return an enum of actions to be
        // taken, match the actions and in case of a deletion, delete
        // the required object
        self.storage.pop();
    }
}
