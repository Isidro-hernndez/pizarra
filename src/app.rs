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
    dimentions: Vec2d,
    undo_status: UndoStatus,
    current_color: Color,
    next_id: usize,
    zoom: i32,
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
            undo_status: UndoStatus::InSync,
            current_color: Color::green(),
            next_id: 1,
            zoom: 0,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let bgcolor = Color::black().to_a();
        let guidecolor = Color::gray().to_a();
        let offset_t = self.get_offset_t();
        let mut commands = Vec::new();
        let zoom = self.zoom;

        for item in self.storage.iter() {
            commands.extend(item.draw_commands().into_iter());
        }

        self.gl.draw(args.viewport(), |c, g| {
            let t = math::multiply(
                math::multiply(c.transform, offset_t),
                math::scale(2.0_f64.powi(zoom), 2.0_f64.powi(zoom))
            );

            graphics::clear(bgcolor, g);

            // content
            for cmd in commands {
                match cmd {
                    DrawCommand::Line{
                        color, thickness, line,
                    } => graphics::line(color, thickness * 2.0_f64.powi(zoom), line, t, g),
                    DrawCommand::Rectangle{
                        color, rect,
                    } => graphics::rectangle(color, rect, t, g),
                    DrawCommand::Circle{
                        color, rect,
                    } => graphics::ellipse(color, rect, t, g),
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
        math::translate(math::mul_scalar(self.offset, -1.0))
    }

    fn get_offset_t(&mut self) -> Matrix2d {
        math::translate(self.offset)
    }

    pub fn delta_offset(&mut self, delta: Vec2d) {
        self.offset = math::add(self.offset, delta);
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

        self.storage.add(self.selected_tool.make(self.current_color, self.next_id));
        self.next_id += 1;
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

    pub fn zoom_in(&mut self) {
        if self.zoom < i32::max_value() {
            self.zoom += 1;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom > i32::min_value() {
            self.zoom -= 1;
        }
    }

    pub fn go_home(&mut self) {
        self.offset = math::mul_scalar(self.dimentions, 0.5);
        self.zoom = 0;
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn serialize(&self) -> String {
        self.storage.serialize()
    }

    pub fn undo(&mut self) {
        self.storage.pop();
    }
}
