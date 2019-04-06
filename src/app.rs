use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics::math::{self, Vec2d, Matrix2d};

use crate::poly::DrawCommand;
use crate::storage::ShapeStorage;
use crate::Tool;
use crate::color::Color;
use crate::serialize::Serialize;

#[derive(Copy,Clone)]
enum UndoStatus {
    InSync,
    At(usize),
}

pub struct App {
    is_drawing: bool,
    is_moving: bool,
    storage: ShapeStorage,
    selected_tool: Tool,
    gl: GlGraphics,
    offset: Vec2d,
    offset_t: Option<Matrix2d>,
    inv_offset: Option<Matrix2d>,
    dimentions: Vec2d,
    undo_status: UndoStatus,
    current_color: Color,
}

impl App {
    pub fn new(gl: GlGraphics, dimentions: Vec2d) -> App {
        App {
            is_drawing: false,
            is_moving: false,
            storage: ShapeStorage::new(),
            selected_tool: Tool::Line,
            gl,
            dimentions,
            offset: math::mul_scalar(dimentions, 0.5),
            inv_offset: None,
            offset_t: None,
            undo_status: UndoStatus::InSync,
            current_color: Color::yellow(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let bgcolor = Color::black().to_a();
        let guidecolor = Color::gray().to_a();
        let offset = self.get_offset_t();
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

    pub fn get_dimentions(&self) -> Vec2d {
        self.dimentions
    }

    fn get_inv_offset(&mut self) -> Matrix2d {
        match self.inv_offset {
            Some(inv) => inv,
            None => {
                let val = math::translate(math::mul_scalar(self.offset, -1.0));
                self.inv_offset = Some(val);

                val
            },
        }
    }

    fn get_offset_t(&mut self) -> Matrix2d {
        match self.offset_t {
            Some(t) => t,
            None => {
                let val = math::translate(self.offset);
                self.offset_t = Some(val);

                val
            }
        }
    }

    pub fn delta_offset(&mut self, delta: Vec2d) {
        self.offset = math::add(self.offset, delta);
        self.offset_t = None;
        self.inv_offset = None;
    }

    pub fn resize(&mut self, new_size: Vec2d) {
        let delta = math::mul_scalar(math::add(
            math::mul_scalar(self.dimentions, -1.0),
            new_size
        ), 0.5);

        self.delta_offset(delta);
        self.dimentions = new_size;
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.selected_tool = tool;
    }

    pub fn start_drawing(&mut self) {
        self.is_drawing = true;

        self.storage.add(self.selected_tool.make(self.current_color));
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

    pub fn handle_cursor(&mut self, pos: Vec2d) {
        if self.is_drawing && !self.is_moving {
            let inv_offset = self.get_inv_offset();

            if let Some(item) = self.storage.last_mut() {
                item.handle(math::transform_pos(inv_offset, pos));
            }
        }
    }

    pub fn handle_cursor_relative(&mut self, delta: Vec2d) {
        if self.is_moving {
            self.delta_offset(delta);
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn serialize(&self) -> String {
        self.storage.serialize()
    }

    pub fn undo(&mut self) {
        // TODO must return an enum of actions to be
        // taken, match the actions and in case of a deletion, delete
        // the required object
        self.storage.pop();
    }
}
