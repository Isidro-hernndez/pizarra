use opengl_graphics::{ GlGraphics, OpenGL };
use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;
use graphics::math;
use std::fs::File;
use std::io::Write;
use chrono::Local;

use pizarra::color::Color;
use pizarra::poly::{DrawCommand, Line, Rectangle};
use pizarra::Pizarra;
use pizarra::storage::ShapeStorage;

fn main() -> std::io::Result<()> {
    let opengl = OpenGL::V3_2;

    // The host of everything
    let mut piz = Pizarra::new([800.0, 400.0]);

    // piston stuff
    let mut window: AppWindow = WindowSettings::new("Pizarra", piz.get_dimentions())
        .exit_on_esc(true).opengl(opengl).build()
        .expect("Window could not be built");
    let ref mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    // state
    let mut is_drawing = false;
    let mut is_moving = false;
    let mut storage = ShapeStorage::new();

    // Colors
    let bgcolor = Color::black().to_a();
    let guidecolor = Color::gray().to_a();

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let t = math::multiply(c.transform, piz.get_offset_t());

                graphics::clear(bgcolor, g);

                // content
                for item in storage.iter() {
                    for cmd in item.draw_commands() {
                        match cmd {
                            DrawCommand::Line{
                                color, thickness, line,
                            } => graphics::line(color, thickness, line, t, g),
                            DrawCommand::Rectangle{
                                color, rect,
                            } => graphics::rectangle(color, rect, t, g),
                        }
                    }
                }

                // UI
                graphics::line(guidecolor, 1.0, [-20.0, 0.0, 20.0, 0.0], t, g);
                graphics::line(guidecolor, 1.0, [0.0, 20.0, 0.0, -20.0], t, g);
            });
        }

        // Mouse Left Button pressed, start of drawing
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            is_drawing = true;
            storage.add(Box::new(Line::new()));
        }

        // Mouse Left Button released, end of drawing
        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            is_drawing = false;
        }

        // start of moving
        if let Some(Button::Mouse(MouseButton::Middle)) = event.press_args() {
            is_moving = true;
        }

        // end of moving
        if let Some(Button::Mouse(MouseButton::Middle)) = event.release_args() {
            is_moving = false;
        }

        // Ctrl
        match event.press_args() {
            Some(Button::Keyboard(Key::LCtrl)) | Some(Button::Keyboard(Key::RCtrl)) => {
                piz.ctrl_on = true;
            },
            _ => {},
        }
        match event.release_args() {
            Some(Button::Keyboard(Key::LCtrl)) | Some(Button::Keyboard(Key::RCtrl)) => {
                piz.ctrl_on = false;
            },
            _ => {},
        }

        // ctrl-z
        if let Some(Button::Keyboard(Key::Z)) = event.press_args() {
            if piz.ctrl_on {
                piz.undo();
                // TODO undo() on piz must return an enum of actions to be
                // taken, match the actions and in case of a deletion, delete
                // the required object
                storage.pop();
            }
        }

        // draw probably
        event.mouse_cursor(|x, y| {
            if is_drawing && !is_moving {
                if let Some(item) = storage.last_mut() {
                    item.handle(math::transform_pos(piz.get_inv_offset(), [x, y]));
                }
            }
        });

        // move canvas
        event.mouse_scroll(|dx, dy| {
            piz.delta_offset([dx, -dy]);
        });
        event.mouse_relative(|dx, dy| {
            if is_moving {
                piz.delta_offset([dx, dy]);
            }
        });
        // or handle resize
        event.resize(|w, h| {
            piz.resize([w, h]);
        });
    }

    Ok(())
}
